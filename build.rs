use std::env;

fn main() {
    if let Ok(path) = env::var("MYSQLCLIENT_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", path);
    }
    println!("cargo:rustc-link-lib=static=mysqlclient");
}
