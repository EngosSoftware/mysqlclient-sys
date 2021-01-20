extern crate pkg_config;

use std::env;

fn main() {
    pkg_config::probe_library("mysqlclient").unwrap();
    pkg_config::probe_library("libssl").unwrap();
    pkg_config::probe_library("libcrypto").unwrap();
    println!("cargo:rustc-link-lib=static=mysqlclient");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
}
