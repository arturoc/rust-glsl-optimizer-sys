extern crate cmake;

fn main() {
    let dst = cmake::Config::new("glsl-optimizer")
        .build_target("")
        .build();
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=mesa");
    println!("cargo:rustc-link-lib=static=glcpp-library");
    println!("cargo:rustc-link-lib=static=glsl_optimizer");
}
