fn main() {
    link_webui();
}

#[cfg(target_os = "macos")]
fn link_webui() {
    println!("cargo:rustc-link-search=webui-macos-clang-arm64/");
    println!("cargo:rustc-link-lib=static=webui-2-static");
}
