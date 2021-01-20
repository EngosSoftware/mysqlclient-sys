extern crate pkg_config;

fn main() {
    pkg_config::Config::new().statik(true).probe_library("mysqlclient").unwrap();
    println!("cargo:rustc-link-lib=static=mysqlclient");
}
