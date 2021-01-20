extern crate pkg_config;

fn main() {
    pkg_config::Config::new().statik(true).probe("mysqlclient").unwrap();
    println!("cargo:rustc-link-lib=static=mysqlclient");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=crypt");
}
