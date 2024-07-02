fn main() {
    napi_build_ohos::setup();
    println!("cargo:rustc-link-lib=static=clang_rt.builtins");
}
