use cmake;

fn main() {
    let dst_halton = cmake::Config::new("libhalton")
        .build();
    println!("cargo:rustc-link-search=native={}", dst_halton.display());
    println!("cargo:rustc-link-lib=static=halton");

    let dst_sobol = cmake::Config::new("libsobol")
        .build();
    println!("cargo:rustc-link-search=native={}", dst_sobol.display());
    println!("cargo:rustc-link-lib=static=sobol");

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
}