
fn main() {
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-search=/opt/occlum/toolchains/gcc/x86_64-linux-musl/lib/");
}