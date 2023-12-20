fn main() {
    println!("cargo:rustc-link-search=webui-linux-clang-x64/");
    println!("cargo:rustc-link-lib=static=webui-2-static");
}
