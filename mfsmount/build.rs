fn main() {
    println!("cargo:rustc-link-lib=fuse3");
    println!("cargo:rustc-link-lib=z");
}
