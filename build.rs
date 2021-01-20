extern crate pkg_config;

fn main() {
    pkg_config::Config::new().statik(true).probe("mysqlclient").unwrap();
    println!("cargo:rustc-link-lib=static=mysqlclient");
    println!("cargo:rustc-link-lib=static=pthread");
    println!("cargo:rustc-link-lib=static=rt");
    println!("cargo:rustc-link-lib=static=dl");
    println!("cargo:rustc-link-lib=static=resolv");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
}
