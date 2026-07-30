//! Label-expression parser/compiler/evaluator, migrated to safe Rust (P1).
//!
//! Mapping from the c2rust original: malloc'd node tree → index-based arena
//! (`Vec<Node>`, -1 = null) so REF-shared subtrees need no free discipline;
//! NUL-terminated C string input → byte slice with `peek()==0` at end
//! (matching C NUL semantics); the matcher's `static mut` stack → local
//! array (single-use per call, strictly safer, same results); rpn_to_infix's
//! malloc'd strings → Vec<u8>. Bytecode format, printf-style error messages,
//! and `parser_data` layout (#[repr(C)], consumers share it) are unchanged.

pub type size_t = usize;
pub type int8_t = i8;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _parser_data {
    pub uniqmask: uint32_t,
    pub labels_mode: uint8_t,
    pub ec_data_chksum_parts: uint8_t,
    pub labelscnt: uint8_t,
    pub labelexpr: [[uint8_t; 128]; 9],
}
impl Default for _parser_data {
    fn default() -> Self {
        _parser_data {
            uniqmask: 0,
            labels_mode: 0,
            ec_data_chksum_parts: 0,
            labelscnt: 0,
            labelexpr: [[0; 128]; 9],
        }
    }
}
pub type parser_data = _parser_data;

pub const REF: uint8_t = 3;
pub const ANY: uint8_t = 4;
pub const NOT: uint8_t = 2;
pub const AND: uint8_t = 1;
pub const OR: uint8_t = 0;
pub const SYM: uint8_t = 5;
pub const SCLASS_EXPR_MAX_SIZE: usize = 128;
pub const SCLASS_EXPR_TYPE_MASK: ::core::ffi::c_int = 0xc0;
pub const SCLASS_EXPR_VALUE_MASK: ::core::ffi::c_int = 0x3f;
pub const SCLASS_EXPR_SYMBOL: ::core::ffi::c_int = 192;
pub const SCLASS_EXPR_SYMBOL_ANY: ::core::ffi::c_int = 0xff;
pub const SCLASS_EXPR_OP_AND: ::core::ffi::c_int = 128;
pub const SCLASS_EXPR_OP_OR: ::core::ffi::c_int = 64;
pub const SCLASS_EXPR_OP_NOT: ::core::ffi::c_int = 0x1;
pub const UNIQ_MASK_IP: uint32_t = 1 << 26;
pub const UNIQ_MASK_RACK: uint32_t = 1 << 27;
pub const LABELS_MODE_LOOSE: ::core::ffi::c_int = 0;
pub const LABELS_MODE_STD: ::core::ffi::c_int = 1;
pub const LABELS_MODE_STRICT: ::core::ffi::c_int = 2;
pub const LABELS_MODE_GLOBAL: ::core::ffi::c_int = 0xff;

#[derive(Clone, Copy)]
struct Node {
    op: uint8_t,
    val: uint8_t,
    arg1: i32, // -1 = null
    arg2: i32,
}

struct Parser {
    arena: Vec<Node>,
}

impl Parser {
    fn newnode(&mut self, op: uint8_t, val: uint8_t, arg1: i32, arg2: i32) -> i32 {
        self.arena.push(Node { op, val, arg1, arg2 });
        (self.arena.len() - 1) as i32
    }
}

struct Expr<'a> {
    s: &'a [u8],
    pos: usize,
    uniqmask: uint32_t,
    labels_mode: uint8_t,
    terms: [i32; 9],
    erroroccured: bool,
    ec_data_chksum_parts: uint8_t,
    p: Parser,
}

impl<'a> Expr<'a> {
    fn peek(&self) -> u8 {
        *self.s.get(self.pos).unwrap_or(&0)
    }
    fn peek_at(&self, off: usize) -> u8 {
        *self.s.get(self.pos + off).unwrap_or(&0)
    }
    fn bump(&mut self, n: usize) {
        self.pos += n;
    }
    fn eat_white(&mut self) {
        while matches!(self.peek(), b' ' | b'\t') {
            self.pos += 1;
        }
    }
    fn parse_error(&mut self, extramsg: &str) {
        let c = self.peek();
        if (c as i8) >= 32 {
            print!("parse error, {}next char: '{}'\n", extramsg, c as char);
        } else {
            print!("parse error, {}next code: 0x{:02X}\n", extramsg, c);
        }
        self.erroroccured = true;
    }

    fn expr_sym(&mut self) -> i32 {
        match self.peek() {
            b'(' | b'[' => {
                let close = if self.peek() == b'(' { b')' } else { b']' };
                let msg = if close == b')' {
                    "closing round bracket expected, "
                } else {
                    "closing square bracket expected, "
                };
                self.bump(1);
                self.eat_white();
                let a = self.expr_or();
                self.eat_white();
                if self.peek() == close {
                    self.bump(1);
                    a
                } else {
                    self.parse_error(msg);
                    -1
                }
            }
            b'*' => {
                self.bump(1);
                self.p.newnode(ANY, 0, -1, -1)
            }
            b'!' | b'~' | b'-' => {
                self.bump(1);
                self.eat_white();
                let a = self.expr_sym();
                self.eat_white();
                self.p.newnode(NOT, 0, a, -1)
            }
            c if c.is_ascii_uppercase() => {
                self.bump(1);
                self.p.newnode(SYM, c - b'A', -1, -1)
            }
            c if c.is_ascii_lowercase() => {
                self.bump(1);
                self.p.newnode(SYM, c - b'a', -1, -1)
            }
            _ => {
                self.parse_error("");
                -1
            }
        }
    }

    fn expr_and(&mut self) -> i32 {
        self.eat_white();
        let a = self.expr_sym();
        self.eat_white();
        if self.peek() == b'&' && self.peek_at(1) == b'&' {
            self.bump(2);
            let b = self.expr_and();
            self.p.newnode(AND, 0, a, b)
        } else if matches!(self.peek(), b'&' | b'*') {
            self.bump(1);
            let b = self.expr_and();
            self.p.newnode(AND, 0, a, b)
        } else if matches!(self.peek(), b'(' | b'[')
            || self.peek().is_ascii_uppercase()
            || self.peek().is_ascii_lowercase()
        {
            let b = self.expr_and();
            self.p.newnode(AND, 0, a, b)
        } else {
            a
        }
    }

    fn expr_or(&mut self) -> i32 {
        self.eat_white();
        let a = self.expr_and();
        self.eat_white();
        if self.peek() == b'|' && self.peek_at(1) == b'|' {
            self.bump(2);
            let b = self.expr_or();
            self.p.newnode(OR, 0, a, b)
        } else if matches!(self.peek(), b'|' | b'+') {
            self.bump(1);
            let b = self.expr_or();
            self.p.newnode(OR, 0, a, b)
        } else {
            a
        }
    }

    fn expr_uniqmask(&mut self) {
        if self.peek() == b'[' {
            self.bump(1);
            match self.peek() {
                b'I' | b'i' => {
                    self.bump(1);
                    if matches!(self.peek(), b'P' | b'p') {
                        self.bump(1);
                    }
                    if self.peek() != b']' {
                        self.parse_error("");
                    }
                    self.bump(1);
                    self.uniqmask = UNIQ_MASK_IP;
                }
                b'R' | b'r' => {
                    self.bump(1);
                    if matches!(self.peek(), b'A' | b'a') {
                        self.bump(1);
                        if matches!(self.peek(), b'C' | b'c') {
                            self.bump(1);
                            if matches!(self.peek(), b'K' | b'k') {
                                self.bump(1);
                            }
                        }
                    }
                    if self.peek() != b']' {
                        self.parse_error("");
                    }
                    self.bump(1);
                    self.uniqmask = UNIQ_MASK_RACK;
                }
                _ => self.parse_error(""),
            }
            return;
        }
        let mut last: u8 = 0xff;
        let mut range = false;
        loop {
            self.eat_white();
            let mut current: u8 = 0xff;
            let c = self.peek();
            if c.is_ascii_uppercase() {
                current = c - b'A';
                self.bump(1);
            } else if c.is_ascii_lowercase() {
                current = c - b'a';
                self.bump(1);
            } else if c == b'-' && !range && last != 0xff {
                self.bump(1);
                range = true;
            } else {
                if range {
                    print!("parse error, expected character after '-'\n");
                    self.erroroccured = true;
                }
                return;
            }
            if range && current != 0xff && last != 0xff {
                if current < last {
                    while current <= last {
                        self.uniqmask |= 1 << current;
                        current = current.wrapping_add(1);
                    }
                } else {
                    while last <= current {
                        self.uniqmask |= 1 << last;
                        last = last.wrapping_add(1);
                    }
                }
                last = 0xff;
                range = false;
            } else if current != 0xff {
                self.uniqmask |= 1 << current;
                last = current;
            }
        }
    }

    fn expr_mode(&mut self) {
        let rest = &self.s[self.pos.min(self.s.len())..];
        match self.peek() {
            b'S' | b's' => {
                if rest.starts_with(b"STD") || rest[1..].starts_with(b"td") {
                    self.labels_mode = LABELS_MODE_STD as uint8_t;
                    self.bump(3);
                } else if rest.starts_with(b"STRICT") || rest[1..].starts_with(b"trict") {
                    self.labels_mode = LABELS_MODE_STRICT as uint8_t;
                    self.bump(6);
                } else {
                    self.labels_mode = LABELS_MODE_STRICT as uint8_t;
                    self.bump(1);
                }
            }
            b'L' | b'l' => {
                if rest.starts_with(b"LOOSE") || rest[1..].starts_with(b"oose") {
                    self.labels_mode = LABELS_MODE_LOOSE as uint8_t;
                    self.bump(5);
                } else {
                    self.labels_mode = LABELS_MODE_LOOSE as uint8_t;
                    self.bump(1);
                }
            }
            b'D' | b'd' => {
                self.labels_mode = LABELS_MODE_STD as uint8_t;
                self.bump(1);
            }
            _ => {}
        }
    }

    fn expr_ending(&mut self) {
        if self.peek() == 0 {
            return;
        }
        if self.peek() == b'/' {
            self.bump(1);
            self.expr_uniqmask();
        }
        if self.peek() == b':' {
            self.bump(1);
            self.expr_mode();
        }
        if self.peek() != 0 {
            self.parse_error("");
        }
    }

    fn expr_top(&mut self) {
        self.eat_white();
        if self.peek() == b'-' {
            let save = self.pos;
            self.bump(1);
            self.eat_white();
            if self.peek() == 0 {
                return;
            }
            self.pos = save;
        }
        if matches!(self.peek(), b'@' | b'=') {
            self.bump(1);
            self.eat_white();
            let c = self.peek();
            if c.is_ascii_digit() && c != b'0' {
                self.ec_data_chksum_parts = c - b'0';
                self.bump(1);
            } else {
                print!("parse error, in ec mode expected number of checksums or data parts after '@'\n");
                self.erroroccured = true;
                return;
            }
            self.eat_white();
            if self.peek() == b'+' {
                self.bump(1);
                self.eat_white();
                let c = self.peek();
                if c.is_ascii_digit()
                    && c != b'0'
                    && (self.ec_data_chksum_parts == 4 || self.ec_data_chksum_parts == 8)
                {
                    self.ec_data_chksum_parts = (self.ec_data_chksum_parts << 4) | (c - b'0' & 0xf);
                    self.bump(1);
                    self.eat_white();
                } else {
                    print!("parse error, in ec mode expected number of checksums after '+' and data parts be set to '4' or '8'\n");
                    self.erroroccured = true;
                    return;
                }
            }
            if matches!(self.peek(), b',' | b';') {
                self.bump(1);
                self.eat_white();
                self.terms[0] = self.expr_or();
                self.eat_white();
                if matches!(self.peek(), b',' | b';') {
                    self.bump(1);
                    self.eat_white();
                    self.terms[1] = self.expr_or();
                    self.eat_white();
                }
            } else {
                self.terms[0] = self.p.newnode(ANY, 0, -1, -1);
            }
            self.expr_ending();
        } else {
            self.ec_data_chksum_parts = 0;
            let mut i: u32 = 0;
            while i < 9 {
                self.eat_white();
                let mut g: u32 = 1;
                let mut f = false;
                let c = self.peek();
                if c.is_ascii_digit() && c != b'0' {
                    g = (c - b'0') as u32;
                    self.bump(1);
                    f = true;
                }
                self.eat_white();
                let a = if i == 0 && f && matches!(self.peek(), 0 | b'/') {
                    self.p.newnode(ANY, 0, -1, -1)
                } else {
                    self.expr_or()
                };
                self.eat_white();
                if self.erroroccured {
                    return;
                }
                if i.wrapping_add(g) > 9 {
                    break;
                }
                let mut first = true;
                while g > 0 {
                    if !first {
                        let idx = self.p.newnode(REF, 0, a, -1);
                        self.terms[i as usize] = idx;
                    } else {
                        self.terms[i as usize] = a;
                        first = false;
                    }
                    i = i.wrapping_add(1);
                    g -= 1;
                }
                if matches!(self.peek(), b',' | b';') {
                    self.bump(1);
                } else if self.peek() != 0 {
                    self.expr_ending();
                    return;
                } else {
                    return;
                }
            }
            print!("parse error, too many copies\n");
            self.erroroccured = true;
        }
    }
}

// --- RPN buffer ---

#[derive(Clone)]
struct Rpn {
    data: [uint8_t; SCLASS_EXPR_MAX_SIZE],
    pos: usize,
}
impl Default for Rpn {
    fn default() -> Self {
        Rpn {
            data: [0; SCLASS_EXPR_MAX_SIZE],
            pos: 0,
        }
    }
}

impl Rpn {
    fn add(&mut self, d: uint8_t) {
        if self.pos < SCLASS_EXPR_MAX_SIZE {
            self.data[self.pos] = d;
            self.pos += 1;
        }
    }
    fn top(&self) -> uint8_t {
        if self.pos > 0 { self.data[self.pos - 1] } else { 0 }
    }
    fn exchg_top(&mut self, d: uint8_t) {
        if self.pos > 0 {
            self.data[self.pos - 1] = d;
        }
    }
}

fn convert_to_rpn(arena: &[Node], n: i32, o: &mut Rpn) {
    let node = arena[n as usize];
    match node.op {
        REF => convert_to_rpn(arena, node.arg1, o),
        SYM => o.add((SCLASS_EXPR_SYMBOL | node.val as ::core::ffi::c_int) as uint8_t),
        OR | AND => {
            convert_to_rpn(arena, node.arg1, o);
            convert_to_rpn(arena, node.arg2, o);
            let t = o.top() as ::core::ffi::c_int;
            let (mask, base) = if node.op == OR {
                (SCLASS_EXPR_OP_OR, SCLASS_EXPR_OP_OR)
            } else {
                (SCLASS_EXPR_OP_AND, SCLASS_EXPR_OP_AND)
            };
            if t & SCLASS_EXPR_TYPE_MASK == mask
                && t & SCLASS_EXPR_VALUE_MASK < SCLASS_EXPR_VALUE_MASK
            {
                o.exchg_top((t + 1) as uint8_t);
            } else {
                o.add(base as uint8_t);
            }
        }
        NOT => {
            convert_to_rpn(arena, node.arg1, o);
            o.add(SCLASS_EXPR_OP_NOT as uint8_t);
        }
        ANY => o.add(SCLASS_EXPR_SYMBOL_ANY as uint8_t),
        _ => {}
    }
}

/// safe core of parse_label_expr: input bytes WITHOUT the trailing NUL
fn parse_label_imp(exprstr: &[u8], pd: &mut parser_data) -> ::core::ffi::c_int {
    *pd = parser_data::default();
    // "-" / "~" alone means "no labels" (original skips parsing entirely)
    if !(exprstr.len() == 1 && matches!(exprstr[0], b'-' | b'~')) {
        let mut e = Expr {
            s: exprstr,
            pos: 0,
            uniqmask: 0,
            labels_mode: LABELS_MODE_GLOBAL as uint8_t,
            terms: [-1; 9],
            erroroccured: false,
            ec_data_chksum_parts: 0,
            p: Parser { arena: Vec::new() },
        };
        e.expr_top();
        let mut res = 0;
        if e.erroroccured {
            res = -1;
        }
        let mut i = 0usize;
        while i < 9 && res == 0 && e.terms[i] >= 0 {
            let mut rpn = Rpn::default();
            convert_to_rpn(&e.p.arena, e.terms[i], &mut rpn);
            if rpn.pos == SCLASS_EXPR_MAX_SIZE {
                print!("parse error, too many terms in expression\n");
                res = -1;
            } else {
                pd.labelexpr[i] = [0; 128];
                pd.labelexpr[i][..rpn.pos].copy_from_slice(&rpn.data[..rpn.pos]);
            }
            i += 1;
        }
        if res == 0 {
            pd.labelscnt = i as uint8_t;
            pd.uniqmask = e.uniqmask;
            pd.labels_mode = e.labels_mode;
            pd.ec_data_chksum_parts = e.ec_data_chksum_parts;
        }
        return res;
    }
    0
}

/// # Safety
/// `exprstr` must be a valid NUL-terminated C string; `pd` writable
/// (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_label_expr(
    mut exprstr: *const ::core::ffi::c_char,
    mut pd: *mut parser_data,
) -> ::core::ffi::c_int {
    // SAFETY: per fn contract.
    let bytes = unsafe { std::ffi::CStr::from_ptr(exprstr) }.to_bytes();
    // SAFETY: per fn contract.
    unsafe { parse_label_imp(bytes, &mut *pd) }
}

/// # Safety
/// `labelexpr` must point to a NUL-terminated RPN bytecode sequence
/// (≤128 bytes, C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn labelmask_matches_labelexpr(
    mut labelmask: uint32_t,
    mut labelexpr: *const uint8_t,
) -> uint8_t {
    // SAFETY: per fn contract; first byte read unconditionally.
    if unsafe { *labelexpr } == 0 {
        return 1;
    }
    // SAFETY: per fn contract; walks to NUL within 128 bytes by format.
    let mut code: &[uint8_t] = unsafe { std::slice::from_raw_parts(labelexpr, 128) };
    if let Some(end) = code.iter().position(|&b| b == 0) {
        code = &code[..end];
    }
    matches_imp(labelmask, code)
}

fn matches_imp(labelmask: uint32_t, code: &[uint8_t]) -> uint8_t {
    let mut stack = [0u8; 128];
    let mut sp = 0usize;
    for &n0 in code {
        let n = n0 as ::core::ffi::c_int;
        match n & SCLASS_EXPR_TYPE_MASK {
            SCLASS_EXPR_SYMBOL => {
                stack[sp] = if n == SCLASS_EXPR_SYMBOL_ANY
                    || labelmask & (1 << (n & SCLASS_EXPR_VALUE_MASK)) != 0
                {
                    1
                } else {
                    0
                };
                sp += 1;
            }
            SCLASS_EXPR_OP_AND => {
                let n = ((n & SCLASS_EXPR_VALUE_MASK) + 2) as usize;
                if n > sp {
                    return 0;
                }
                let mut r = 1u8;
                for _ in 0..n {
                    sp -= 1;
                    if stack[sp] == 0 {
                        r = 0;
                    }
                }
                stack[sp] = r;
                sp += 1;
            }
            SCLASS_EXPR_OP_OR => {
                let n = ((n & SCLASS_EXPR_VALUE_MASK) + 2) as usize;
                if n > sp {
                    return 0;
                }
                let mut r = 0u8;
                for _ in 0..n {
                    sp -= 1;
                    if stack[sp] == 1 {
                        r = 1;
                    }
                }
                stack[sp] = r;
                sp += 1;
            }
            0 => {
                if n & SCLASS_EXPR_VALUE_MASK == SCLASS_EXPR_OP_NOT {
                    if sp == 0 {
                        return 0;
                    }
                    stack[sp - 1] = 1 - stack[sp - 1];
                }
            }
            _ => {}
        }
    }
    if sp == 1 {
        return stack[0];
    }
    0
}

// --- RPN → infix (for make_label_expr) ---

fn labelexpr_eq(a: &[uint8_t], b: &[uint8_t]) -> bool {
    // compare NUL-terminated bytecode sequences
    let end_a = a.iter().position(|&b| b == 0).unwrap_or(a.len());
    let end_b = b.iter().position(|&b| b == 0).unwrap_or(b.len());
    a[..end_a] == b[..end_b]
}

fn rpn_to_infix(code: &[uint8_t]) -> Vec<u8> {
    if code.first() == Some(&0) || code.is_empty() {
        return b"*".to_vec();
    }
    let code = match code.iter().position(|&b| b == 0) {
        Some(e) => &code[..e],
        None => code,
    };
    let mut stack: Vec<(u8, Vec<u8>)> = Vec::new(); // (level, string)
    for &n0 in code {
        let n = n0 as ::core::ffi::c_int;
        match n & SCLASS_EXPR_TYPE_MASK {
            SCLASS_EXPR_SYMBOL => {
                let s = if n == SCLASS_EXPR_SYMBOL_ANY {
                    b"*".to_vec()
                } else {
                    vec![b'A' + (n & SCLASS_EXPR_VALUE_MASK) as u8]
                };
                stack.push((0, s));
            }
            SCLASS_EXPR_OP_AND | SCLASS_EXPR_OP_OR => {
                let is_and = n & SCLASS_EXPR_TYPE_MASK == SCLASS_EXPR_OP_AND;
                let n = ((n & SCLASS_EXPR_VALUE_MASK) + 2) as usize;
                if n > stack.len() {
                    return b"ERROR".to_vec();
                }
                let paren_level = if is_and { 1 } else { 2 };
                let sep = if is_and { b'&' } else { b'|' };
                let mut str_: Vec<u8> = Vec::new();
                let parts: Vec<(u8, Vec<u8>)> =
                    stack.drain(stack.len() - n..).collect();
                for (i, (level, s)) in parts.into_iter().enumerate() {
                    if i > 0 {
                        str_.push(sep);
                    }
                    if level > paren_level {
                        str_.push(b'(');
                        str_.extend_from_slice(&s);
                        str_.push(b')');
                    } else {
                        str_.extend_from_slice(&s);
                    }
                }
                stack.push((paren_level, str_));
            }
            0 => {
                if n & SCLASS_EXPR_VALUE_MASK == SCLASS_EXPR_OP_NOT {
                    let Some((level, s)) = stack.pop() else {
                        return b"ERROR".to_vec();
                    };
                    let mut str_ = b"~".to_vec();
                    if level > 0 {
                        str_.push(b'(');
                        str_.extend_from_slice(&s);
                        str_.push(b')');
                    } else {
                        str_.extend_from_slice(&s);
                    }
                    stack.push((0, str_));
                }
            }
            _ => {}
        }
    }
    if stack.len() == 1 {
        stack.pop().unwrap().1
    } else {
        b"ERROR".to_vec()
    }
}

/// safe core of make_label_expr
fn make_label_imp(pd: &parser_data) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    if pd.ec_data_chksum_parts != 0 {
        let i = pd.ec_data_chksum_parts >> 4;
        let j = pd.ec_data_chksum_parts & 0xf;
        out.push(b'@');
        if i == 8 || i == 4 {
            out.push(b'0' + i);
            out.push(b'+');
        }
        out.push(b'0' + j);
    } else if pd.labelscnt == 0 {
        out.push(b'-');
        return out;
    }
    let mut i = 0usize;
    while i < pd.labelscnt as usize {
        if i > 0 || pd.ec_data_chksum_parts > 0 {
            out.push(b',');
        }
        let mut c = 1usize;
        while i + c < pd.labelscnt as usize
            && labelexpr_eq(&pd.labelexpr[i], &pd.labelexpr[i + c])
        {
            c += 1;
        }
        if c > 1 {
            out.push(b'0' + c as u8);
        }
        out.extend_from_slice(&rpn_to_infix(&pd.labelexpr[i]));
        i += c;
    }
    if pd.uniqmask != 0 {
        out.push(b'/');
        if pd.uniqmask & UNIQ_MASK_IP != 0 {
            out.extend_from_slice(b"[IP]");
        } else if pd.uniqmask & UNIQ_MASK_RACK != 0 {
            out.extend_from_slice(b"[RACK]");
        } else {
            let mut i = 0u8;
            while i < 26 {
                if pd.uniqmask & (1 << i) != 0 {
                    if i < 24 && pd.uniqmask >> i & 7 == 7 {
                        out.push(b'A' + i);
                        out.push(b'-');
                        while pd.uniqmask & (1 << i) != 0 && i < 26 {
                            i = i.wrapping_add(1);
                        }
                        i = i.wrapping_sub(1);
                        out.push(b'A' + i);
                    } else {
                        out.push(b'A' + i);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if matches!(
        pd.labels_mode as ::core::ffi::c_int,
        LABELS_MODE_STD | LABELS_MODE_LOOSE | LABELS_MODE_STRICT
    ) {
        out.push(b':');
        match pd.labels_mode as ::core::ffi::c_int {
            LABELS_MODE_STRICT => out.extend_from_slice(b"STRICT"),
            LABELS_MODE_LOOSE => out.extend_from_slice(b"LOOSE"),
            _ => out.extend_from_slice(b"STD"),
        }
    }
    out
}

/// # Safety
/// `strbuff` must be writable for the rendered expression + NUL
/// (C caller contract; callers provide a sufficiently large buffer,
/// as in the original).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_label_expr(
    mut strbuff: *mut ::core::ffi::c_char,
    mut pd: *const parser_data,
) -> *mut ::core::ffi::c_char {
    // SAFETY: per fn contract.
    let out = unsafe { make_label_imp(&*pd) };
    // SAFETY: per fn contract; writes out.len()+1 bytes.
    unsafe {
        std::ptr::copy_nonoverlapping(out.as_ptr() as *const ::core::ffi::c_char, strbuff, out.len());
        *strbuff.add(out.len()) = 0;
    }
    strbuff
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> (i32, parser_data) {
        let mut pd = parser_data::default();
        let rc = parse_label_imp(s.as_bytes(), &mut pd);
        (rc, pd)
    }

    fn render(pd: &parser_data) -> String {
        String::from_utf8(make_label_imp(pd)).unwrap()
    }

    #[test]
    fn parse_and_render_roundtrip() {
        for (src, expect) in [
            ("A", "A"),
            ("AB", "A&B"),
            ("A,B", "A,B"),
            ("2A", "2A"),
            ("A|B", "A|B"),
            ("*", "*"),
            ("!A", "~A"),
            ("-", "-"),
            ("A/B", "A/B"),
            ("A/[IP]", "A/[IP]"),
            ("A:LOOSE", "A:LOOSE"),
            ("@4+2", "@4+2,*"), // implicit ANY term after ec spec (original behavior)
        ] {
            let (rc, pd) = parse(src);
            assert_eq!(rc, 0, "parse {src}");
            assert_eq!(render(&pd), expect, "render {src}");
        }
    }

    #[test]
    fn matcher_semantics() {
        // "A" matches mask with bit 0
        let (_, pd) = parse("A");
        assert_eq!(matches_imp(0b1, &pd.labelexpr[0]), 1);
        assert_eq!(matches_imp(0b10, &pd.labelexpr[0]), 0);
        // "A&B" needs both
        let (_, pd) = parse("AB");
        assert_eq!(matches_imp(0b11, &pd.labelexpr[0]), 1);
        assert_eq!(matches_imp(0b01, &pd.labelexpr[0]), 0);
        // "A|B"
        let (_, pd) = parse("A|B");
        assert_eq!(matches_imp(0b10, &pd.labelexpr[0]), 1);
        // "!A"
        let (_, pd) = parse("!A");
        assert_eq!(matches_imp(0b10, &pd.labelexpr[0]), 1);
        assert_eq!(matches_imp(0b01, &pd.labelexpr[0]), 0);
        // "*"
        let (_, pd) = parse("*");
        assert_eq!(matches_imp(0, &pd.labelexpr[0]), 1);
    }

    #[test]
    fn parse_errors() {
        assert_eq!(parse("1A,B,C,D,E,F,G,H,I,J").0, -1); // too many copies
        assert_eq!(parse("(A").0, -1);
        assert_eq!(parse("A$").0, -1);
    }
}
