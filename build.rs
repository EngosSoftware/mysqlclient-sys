extern crate pkg_config;

use std::env;

fn main() {
    if pkg_config::probe_library("mysqlclient").is_ok() {
        // ok
    } else if let Ok(path) = env::var("MYSQLCLIENT_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", path);
    }
    println!("cargo:rustc-link-lib=static=mysqlclient");
}
