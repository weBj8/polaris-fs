#![deny(unsafe_code)]
// allow: SIZE_OK — request grammar, HTTP state, and response vectors form one protocol core.

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

pub const HTTP_HEADER_MAX: usize = 16_384;
pub const FORWARD_BUFFER_SIZE: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Method {
    None = 0,
    Get = 1,
    Head = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Status {
    Ok = 0,
    Found = 1,
    NotFound = 2,
    Forbidden = 3,
    NotModified = 4,
    BadRequest = 5,
    MethodNotAllowed = 6,
    InternalError = 7,
}

impl Status {
    pub const fn line(self) -> &'static [u8] {
        match self {
            Self::Ok => b"HTTP/1.1 200 OK",
            Self::Found => b"HTTP/1.1 302 Found",
            Self::NotModified => b"HTTP/1.1 304 Not Modified",
            Self::BadRequest => b"HTTP/1.1 400 Bad Request",
            Self::MethodNotAllowed => b"HTTP/1.1 405 Method Not Allowed",
            Self::NotFound => b"HTTP/1.1 404 Not Found",
            Self::Forbidden => b"HTTP/1.1 403 Forbidden",
            Self::InternalError => b"HTTP/1.1 500 Internal Server Error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RequestKind {
    File = 1,
    Cgi = 2,
    Redirect = 3,
    Directory = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValidatorMatch {
    Unknown,
    No,
    Yes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub kind: RequestKind,
    pub route: Vec<u8>,
    pub target: Vec<u8>,
    pub extra: Option<Vec<u8>>,
    pub mime: Option<&'static [u8]>,
    pub modified_seconds: i64,
    pub modified_text: Option<Vec<u8>>,
    pub etag: Option<[u8; 32]>,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeConfig {
    pub root: Vec<u8>,
    pub requests_file: Vec<u8>,
    pub timeout: u16,
    pub listen_host: Vec<u8>,
    pub listen_port: Vec<u8>,
    pub os_path: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScanStamp {
    pub root_mtime: i64,
    pub requests_mtime: i64,
    pub requests_len: i64,
}

#[derive(Clone, Debug)]
pub struct ServerState {
    pub config: RuntimeConfig,
    pub requests: Vec<Request>,
    pub stamp: ScanStamp,
}

pub fn clamp_timeout(timeout: u32) -> u16 {
    timeout.clamp(10, u16::MAX as u32) as u16
}

pub fn requests_path(root: &Path, configured: &[u8]) -> Vec<u8> {
    root.join(OsStr::from_bytes(configured))
        .into_os_string()
        .into_vec()
}

pub fn parse_config_line(line: &[u8], root: &Path) -> Result<Option<Request>, ()> {
    let line = line.split(|byte| *byte == 0).next().unwrap_or_default();
    let Some((&kind, rest)) = line.split_first() else {
        return Err(());
    };
    let kind = match kind {
        b'f' | b'F' => RequestKind::File,
        b'c' | b'C' => RequestKind::Cgi,
        b'r' | b'R' => RequestKind::Redirect,
        b'd' | b'D' => RequestKind::Directory,
        b'#' | b';' | b'/' => return Ok(None),
        _ => return Err(()),
    };
    if rest
        .first()
        .is_some_and(|byte| !matches!(byte, b' ' | b'\t'))
    {
        return Err(());
    }
    let mut fields = rest
        .split(|byte| matches!(byte, b' ' | b'\t'))
        .filter(|field| !field.is_empty());
    let route_field = fields.next().ok_or(())?;
    let route = route_field
        .iter()
        .skip_while(|byte| **byte == b'/')
        .copied()
        .collect::<Vec<_>>();
    let target_field = fields.next();
    let target = match kind {
        RequestKind::Redirect => {
            let target = target_field.ok_or(())?;
            target
                .iter()
                .skip_while(|byte| **byte == b'/')
                .copied()
                .collect()
        }
        RequestKind::File | RequestKind::Cgi | RequestKind::Directory => {
            let relative = target_field.unwrap_or(route.as_slice());
            let mut target = root.as_os_str().as_bytes().to_vec();
            target.push(b'/');
            target.extend_from_slice(relative);
            target
        }
    };
    let extra = fields.next().map(<[u8]>::to_vec);
    let mime = match kind {
        RequestKind::File | RequestKind::Directory if extra.is_none() => {
            mime_for_path(&target).or_else(|| mime_for_path(&route))
        }
        RequestKind::File | RequestKind::Cgi | RequestKind::Redirect | RequestKind::Directory => {
            None
        }
    };
    Ok(Some(Request {
        kind,
        route,
        target,
        extra,
        mime,
        modified_seconds: 0,
        modified_text: None,
        etag: None,
        data: Vec::new(),
    }))
}

pub fn mime_for_path(path: &[u8]) -> Option<&'static [u8]> {
    const MIMES: &[(&[u8], &[u8])] = &[
        (b"txt", b"text/plain"),
        (b"html", b"text/html; charset=utf-8"),
        (b"css", b"text/css"),
        (b"js", b"text/javascript"),
        (b"ico", b"image/vnd.microsoft.icon"),
        (b"gif", b"image/gif"),
        (b"jpg", b"image/jpeg"),
        (b"jpeg", b"image/jpeg"),
        (b"png", b"image/png"),
        (b"tiff", b"image/tiff"),
        (b"tif", b"image/tiff"),
        (b"bmp", b"image/bmp"),
        (b"zip", b"application/zip"),
        (b"xml", b"text/xml"),
        (b"svg", b"image/svg+xml"),
        (b"pdf", b"application/pdf"),
        (b"ttf", b"application/octet-stream"),
    ];
    MIMES.iter().find_map(|(extension, mime)| {
        (path.len() > extension.len()
            && path[path.len() - extension.len() - 1] == b'.'
            && path.ends_with(extension))
        .then_some(*mime)
    })
}

fn route_index(requests: &[Request], url: &[u8]) -> Option<usize> {
    requests.iter().position(|request| match request.kind {
        RequestKind::Directory => {
            request.route.len() < url.len()
                && url.starts_with(&request.route)
                && url[request.route.len()] == b'/'
                || request.route == url
        }
        RequestKind::File | RequestKind::Cgi | RequestKind::Redirect => request.route == url,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedRequest {
    pub method: Method,
    pub url: Vec<u8>,
    pub args: Option<Vec<u8>>,
    pub http_version: u8,
    pub keep_alive: bool,
    pub status: Status,
    pub route_index: Option<usize>,
    etag_match: ValidatorMatch,
    modified_match: ValidatorMatch,
}

impl ParsedRequest {
    fn rejected(status: Status) -> Self {
        Self {
            method: Method::None,
            url: Vec::new(),
            args: None,
            http_version: 0,
            keep_alive: false,
            status,
            route_index: None,
            etag_match: ValidatorMatch::Unknown,
            modified_match: ValidatorMatch::Unknown,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseProgress {
    Pending,
    Ready(ParsedRequest),
    Closed,
}

pub struct RequestParser {
    buffered: Vec<u8>,
    method: Method,
}

impl RequestParser {
    pub const fn new() -> Self {
        Self {
            buffered: Vec::new(),
            method: Method::None,
        }
    }

    pub fn feed(&mut self, fragment: &[u8], requests: &[Request]) -> ParseProgress {
        if self.buffered.len().saturating_add(fragment.len()) >= HTTP_HEADER_MAX {
            return ParseProgress::Closed;
        }
        self.buffered.extend_from_slice(fragment);
        if self.method == Method::None && self.buffered.len() >= 8 {
            self.method = if self.buffered.starts_with(b"GET ") {
                Method::Get
            } else if self.buffered.starts_with(b"HEAD ") {
                Method::Head
            } else {
                return ParseProgress::Ready(ParsedRequest::rejected(Status::MethodNotAllowed));
            };
        }
        let Some(end) = self
            .buffered
            .windows(4)
            .position(|bytes| bytes == b"\r\n\r\n")
        else {
            return ParseProgress::Pending;
        };
        ParseProgress::Ready(parse_complete(
            &self.buffered[..end + 2],
            self.method,
            requests,
        ))
    }
}

fn parse_complete(data: &[u8], method: Method, requests: &[Request]) -> ParsedRequest {
    let line_end = data.windows(2).position(|bytes| bytes == b"\r\n");
    let Some(line_end) = line_end else {
        return ParsedRequest::rejected(Status::BadRequest);
    };
    let line = &data[..line_end];
    let prefix = match method {
        Method::Get => 4,
        Method::Head => 5,
        Method::None => return ParsedRequest::rejected(Status::MethodNotAllowed),
    };
    if line.get(prefix) != Some(&b'/') {
        return ParsedRequest::rejected(Status::BadRequest);
    }
    let path_start = prefix + 1;
    let Some(delimiter) = line[path_start..]
        .iter()
        .position(|byte| matches!(byte, b' ' | b'?' | b'&'))
        .map(|index| path_start + index)
    else {
        return ParsedRequest::rejected(Status::BadRequest);
    };
    let url = line[path_start..delimiter].to_vec();
    let (args, version) = match line[delimiter] {
        b' ' => (None, &line[delimiter + 1..]),
        b'?' | b'&' => {
            let query = &line[delimiter + 1..];
            match query.iter().position(|byte| *byte == b' ') {
                Some(space) => (Some(query[..space].to_vec()), &query[space + 1..]),
                None => (Some(query.to_vec()), &[][..]),
            }
        }
        _ => return ParsedRequest::rejected(Status::BadRequest),
    };
    let (http_version, mut keep_alive) = if version == b"HTTP/1.0" {
        (0x10, false)
    } else if version == b"HTTP/1.1" {
        (0x11, true)
    } else {
        (0, false)
    };
    let selected = route_index(requests, &url);
    let mut etag_match = ValidatorMatch::Unknown;
    let mut modified_match = ValidatorMatch::Unknown;
    for header in split_crlf(&data[line_end + 2..]) {
        let Some(separator) = header.iter().position(|byte| *byte == b':') else {
            continue;
        };
        let (name, value) = (&header[..separator], &header[separator + 1..]);
        let value = &value[value.iter().take_while(|byte| **byte == b' ').count()..];
        if let Some(request) = selected.and_then(|index| requests.get(index)) {
            if request.kind == RequestKind::File
                && etag_match == ValidatorMatch::Unknown
                && name.eq_ignore_ascii_case(b"If-None-Match")
            {
                if let Some(etag) = request.etag {
                    etag_match = match value.strip_prefix(b"\"") {
                        Some(candidate)
                            if candidate.len() >= 33
                                && candidate[..32] == etag
                                && candidate[32] == b'\"' =>
                        {
                            ValidatorMatch::Yes
                        }
                        Some(_) | None => ValidatorMatch::No,
                    };
                }
            }
            if request.kind == RequestKind::File
                && modified_match == ValidatorMatch::Unknown
                && name.eq_ignore_ascii_case(b"If-Modified-Since")
            {
                if let Some(modified) = &request.modified_text {
                    modified_match = if value == modified {
                        ValidatorMatch::Yes
                    } else {
                        ValidatorMatch::No
                    };
                }
            }
        }
        if name.eq_ignore_ascii_case(b"Connection") {
            if starts_ascii_case_insensitive(value, b"close") {
                keep_alive = false;
            }
            if starts_ascii_case_insensitive(value, b"keep-alive") {
                keep_alive = true;
            }
        }
    }
    ParsedRequest {
        method,
        url,
        args,
        http_version,
        keep_alive,
        status: Status::Ok,
        route_index: selected,
        etag_match,
        modified_match,
    }
}

fn split_crlf(mut data: &[u8]) -> impl Iterator<Item = &[u8]> {
    std::iter::from_fn(move || {
        let end = data.windows(2).position(|bytes| bytes == b"\r\n")?;
        let line = &data[..end];
        data = &data[end + 2..];
        Some(line)
    })
}

pub fn starts_ascii_case_insensitive(value: &[u8], expected: &[u8]) -> bool {
    value.len() >= expected.len() && value[..expected.len()].eq_ignore_ascii_case(expected)
}

pub fn header_prefix_len(line: &[u8], header: &[u8]) -> Option<usize> {
    (line.len() > header.len()
        && line[..header.len()].eq_ignore_ascii_case(header)
        && line[header.len()] == b':')
        .then_some(header.len() + 1)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resource {
    pub data: Vec<u8>,
    pub mime: Option<&'static [u8]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub status: Status,
    pub headers: Vec<u8>,
    pub body: Vec<u8>,
    pub keep_alive: bool,
}

pub fn prepare_response(
    parsed: &ParsedRequest,
    request: Option<&Request>,
    resource: Option<&Resource>,
) -> Response {
    let mut status = parsed.status;
    if status == Status::Ok && request.is_none() {
        status = Status::NotFound;
    }
    if status == Status::Ok {
        if parsed.etag_match == ValidatorMatch::Yes
            || parsed.etag_match == ValidatorMatch::Unknown
                && parsed.modified_match == ValidatorMatch::Yes
        {
            status = Status::NotModified;
        } else if request.is_some_and(|request| request.kind == RequestKind::Redirect) {
            status = Status::Found;
        } else if request.is_some_and(|request| request.kind == RequestKind::Cgi)
            && parsed.method == Method::Head
        {
            status = Status::MethodNotAllowed;
        }
    }
    let keep_alive = status == Status::Ok && parsed.keep_alive;
    let mut headers = Vec::with_capacity(512);
    push_line(&mut headers, status.line());
    push_line(&mut headers, b"Server: mfsgui");
    push_line(
        &mut headers,
        if keep_alive {
            b"Connection: keep-alive"
        } else {
            b"Connection: close"
        },
    );
    let mut body = Vec::new();
    if let Some(request) = request {
        if status == Status::Ok {
            let content =
                resource.map_or(request.data.as_slice(), |resource| resource.data.as_slice());
            let mime = request
                .extra
                .as_deref()
                .or_else(|| resource.and_then(|resource| resource.mime).or(request.mime));
            push_field(
                &mut headers,
                b"Content-Type: ",
                mime.unwrap_or(b"text/plain"),
            );
            push_field(
                &mut headers,
                b"Content-Length: ",
                content.len().to_string().as_bytes(),
            );
            if parsed.method == Method::Get {
                body.extend_from_slice(content);
            }
        }
        if status == Status::Found {
            let location = redirect_location(request, parsed.args.as_deref());
            push_field(&mut headers, b"Location: ", &location);
        } else {
            push_line(&mut headers, b"Cache-Control: public,max-age=0");
            if let Some(etag) = request.etag {
                let mut quoted = Vec::with_capacity(34);
                quoted.push(b'\"');
                quoted.extend_from_slice(&etag);
                quoted.push(b'\"');
                push_field(&mut headers, b"ETag: ", &quoted);
            }
            if let Some(modified) = request.modified_text.as_deref() {
                push_field(&mut headers, b"Last-Modified: ", modified);
            }
        }
    }
    push_line(&mut headers, b"");
    Response {
        status,
        headers,
        body,
        keep_alive,
    }
}

fn redirect_location(request: &Request, args: Option<&[u8]>) -> Vec<u8> {
    let mut location = Vec::with_capacity(request.target.len() + 2);
    location.push(b'/');
    location.extend_from_slice(&request.target);
    if let Some(args) = args {
        location.push(b'?');
        location.extend_from_slice(args);
    }
    if let Some(extra) = request.extra.as_deref() {
        location.push(if args.is_some() { b'&' } else { b'?' });
        location.extend_from_slice(extra);
    }
    location
}

fn push_line(output: &mut Vec<u8>, line: &[u8]) {
    output.extend_from_slice(line);
    output.extend_from_slice(b"\r\n");
}

fn push_field(output: &mut Vec<u8>, name: &[u8], value: &[u8]) {
    output.extend_from_slice(name);
    push_line(output, value);
}

pub fn resolve_directory_path(base: &Path, route: &[u8], url: &[u8]) -> Result<PathBuf, Status> {
    let suffix = url.get(route.len()..).ok_or(Status::Forbidden)?;
    let mut candidate = base.as_os_str().as_bytes().to_vec();
    candidate.extend_from_slice(suffix);
    let resolved = std::fs::canonicalize(PathBuf::from(OsString::from_vec(candidate)))
        .map_err(|_| Status::NotFound)?;
    let base = base.as_os_str().as_bytes();
    let resolved_bytes = resolved.as_os_str().as_bytes();
    if !resolved_bytes.starts_with(base) || resolved_bytes.get(base.len()) != Some(&b'/') {
        return Err(Status::Forbidden);
    }
    Ok(resolved)
}

pub fn query_string(args: Option<&[u8]>, extra: Option<&[u8]>) -> Vec<u8> {
    let mut query = b"QUERY_STRING=".to_vec();
    if let Some(args) = args {
        query.extend_from_slice(args);
        if extra.is_some() {
            query.push(b'&');
        }
    }
    if let Some(extra) = extra {
        query.extend_from_slice(extra);
    }
    query.truncate(1_023);
    query
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgiHeaders {
    pub headers: Vec<u8>,
    pub body_offset: usize,
}

pub fn prepare_cgi_headers(data: &[u8]) -> CgiHeaders {
    let mut headers = Vec::with_capacity(512);
    let mut position = 0;
    let mut status_written = false;
    let mut content_type = false;
    while position < data.len() {
        let Some(relative_end) = data[position..].iter().position(|byte| *byte == b'\n') else {
            break;
        };
        let next = position + relative_end + 1;
        let mut line = &data[position..position + relative_end];
        if line.last() == Some(&b'\r') {
            line = &line[..line.len() - 1];
        }
        position = next;
        if line.is_empty() {
            break;
        }
        if let Some(prefix) = header_prefix_len(line, b"Status") {
            let status = line[prefix..].trim_ascii_start();
            let mut status_line = b"HTTP/1.1 ".to_vec();
            status_line.extend_from_slice(status);
            push_line(&mut headers, &status_line);
            push_line(&mut headers, b"Server: mfsgui");
            push_line(&mut headers, b"Connection: close");
            status_written = true;
        } else {
            if !status_written {
                push_line(&mut headers, b"HTTP/1.1 200 OK");
                push_line(&mut headers, b"Server: mfsgui");
                push_line(&mut headers, b"Connection: close");
                status_written = true;
            }
            content_type |= header_prefix_len(line, b"Content-Type").is_some();
            push_line(&mut headers, line);
        }
    }
    if !content_type {
        push_line(&mut headers, b"Content-Type: text/plain");
    }
    push_line(&mut headers, b"");
    CgiHeaders {
        headers,
        body_offset: position,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn static_request() -> Request {
        Request {
            kind: RequestKind::File,
            route: Vec::new(),
            target: b"/srv/index.html".to_vec(),
            extra: None,
            mime: Some(b"text/html"),
            modified_seconds: 3,
            modified_text: Some(b"Wed, 01 Jan 2025 00:00:00 GMT".to_vec()),
            etag: Some(*b"900150983CD24FB0D6963F7D28E17F72"),
            data: b"abc".to_vec(),
        }
    }

    fn parse_ready(input: &[u8], requests: &[Request]) -> ParsedRequest {
        match RequestParser::new().feed(input, requests) {
            ParseProgress::Ready(parsed) => parsed,
            ParseProgress::Pending | ParseProgress::Closed => panic!("request did not complete"),
        }
    }

    #[test]
    fn config_line_preserves_c_grammar_and_route_order() {
        let root = Path::new("/srv/gui");
        let file = parse_config_line(b"f / index.html text/custom", root)
            .unwrap()
            .unwrap();
        assert_eq!(file.kind, RequestKind::File);
        assert_eq!(file.route, b"");
        assert_eq!(file.target, b"/srv/gui/index.html");
        assert_eq!(file.extra.as_deref(), Some(b"text/custom".as_slice()));

        let redirect = parse_config_line(b"R /old /new/path x=1", root)
            .unwrap()
            .unwrap();
        assert_eq!(redirect.kind, RequestKind::Redirect);
        assert_eq!(redirect.target, b"new/path");
        assert!(parse_config_line(b"redirect /old /new", root).is_err());
        assert_eq!(parse_config_line(b"# ignored", root).unwrap(), None);
    }

    #[test]
    fn fragmented_request_parses_get_head_bad_method_and_limit() {
        let requests = vec![static_request()];
        let mut parser = RequestParser::new();
        assert_eq!(
            parser.feed(b"GET / HTTP/1.", &requests),
            ParseProgress::Pending
        );
        assert_eq!(
            parser.feed(b"1\r\nConnection: close\r\n", &requests),
            ParseProgress::Pending
        );
        let ParseProgress::Ready(parsed) = parser.feed(b"\r\n", &requests) else {
            panic!("fragmented request did not complete");
        };
        assert_eq!(parsed.method, Method::Get);
        assert_eq!(parsed.route_index, Some(0));
        assert!(!parsed.keep_alive);

        let parsed = parse_ready(b"HEAD / HTTP/1.1\r\n\r\n", &requests);
        assert_eq!(parsed.method, Method::Head);

        let parsed = parse_ready(b"POST / HTTP/1.1", &requests);
        assert_eq!(parsed.status, Status::MethodNotAllowed);

        let mut parser = RequestParser::new();
        assert_eq!(
            parser.feed(&vec![b'x'; HTTP_HEADER_MAX], &requests),
            ParseProgress::Closed
        );
    }

    #[test]
    fn conditional_and_response_vectors_match_c() {
        let requests = vec![static_request()];
        let parsed = parse_ready(
            b"GET / HTTP/1.1\r\nIf-None-Match: \"900150983CD24FB0D6963F7D28E17F72\"\r\n\r\n",
            &requests,
        );
        let response = prepare_response(&parsed, Some(&requests[0]), None);
        assert_eq!(response.status, Status::NotModified);
        assert!(!response.keep_alive);
        assert!(
            response
                .headers
                .starts_with(b"HTTP/1.1 304 Not Modified\r\n")
        );
        assert!(
            !response
                .headers
                .windows(15)
                .any(|part| part == b"Content-Length:")
        );

        let parsed = parse_ready(
            b"GET / HTTP/1.1\r\nIf-Modified-Since: Wed, 01 Jan 2025 00:00:00 GMT\r\n\r\n",
            &requests,
        );
        assert_eq!(
            prepare_response(&parsed, Some(&requests[0]), None).status,
            Status::NotModified
        );

        let parsed = parse_ready(
            b"GET / HTTP/1.1\r\nIf-Modified-Since: Wed, 01 Jan 2025 00:00:00 GMT garbage\r\n\r\n",
            &requests,
        );
        assert_eq!(
            prepare_response(&parsed, Some(&requests[0]), None).status,
            Status::Ok
        );

        let parsed = parse_ready(b"HEAD / HTTP/1.1\r\n\r\n", &requests);
        let response = prepare_response(&parsed, Some(&requests[0]), None);
        assert_eq!(response.status, Status::Ok);
        assert!(
            response
                .headers
                .windows(17)
                .any(|part| part == b"Content-Length: 3")
        );
        assert!(response.body.is_empty());
    }

    #[test]
    fn config_and_path_vectors_preserve_bounds_and_containment() {
        assert_eq!(clamp_timeout(1), 10);
        assert_eq!(clamp_timeout(300), 300);
        assert_eq!(clamp_timeout(70_000), 65_535);
        assert_eq!(
            requests_path(Path::new("/srv/gui"), b"requests.cfg"),
            b"/srv/gui/requests.cfg"
        );
        assert_eq!(
            requests_path(Path::new("/srv/gui"), b"/etc/gui.cfg"),
            b"/etc/gui.cfg"
        );

        let root = Path::new("target/mfsgui-core-path-test");
        let _ = std::fs::remove_dir_all(root);
        std::fs::create_dir_all(root.join("assets")).unwrap();
        std::fs::write(root.join("assets/app.js"), b"ok").unwrap();
        std::fs::write(root.join("secret"), b"no").unwrap();
        let base = std::fs::canonicalize(root.join("assets")).unwrap();
        assert!(resolve_directory_path(&base, b"assets", b"assets/app.js").is_ok());
        assert_eq!(
            resolve_directory_path(&base, b"assets", b"assets/../secret"),
            Err(Status::Forbidden)
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cgi_header_vector_supplies_defaults_and_body_offset() {
        let parsed = prepare_cgi_headers(b"X-Test: yes\r\n\r\nbody");
        assert!(parsed.headers.starts_with(b"HTTP/1.1 200 OK\r\n"));
        assert!(
            parsed
                .headers
                .windows(24)
                .any(|part| part == b"Content-Type: text/plain")
        );
        assert_eq!(&b"X-Test: yes\r\n\r\nbody"[parsed.body_offset..], b"body");
    }
}
