const WEBUI_VERSION: &str = "2.4.2";
const WEBUI_RELEASE_BASE_URL: &str = "https://github.com/webui-dev/webui/releases/download";

fn main() {
    link_webui();
}

#[cfg(target_os = "macos")]
fn link_webui() {
    println!("cargo:rustc-link-search=webui-macos-clang-arm64/");
    println!("cargo:rustc-link-lib=static=webui-2-static");
}

#[cfg(target_os = "linux")]
fn link_webui() {
    println!("cargo:rustc-link-search=webui-linux-clang-x64/");
    println!("cargo:rustc-link-lib=static=webui-2-static");
}
