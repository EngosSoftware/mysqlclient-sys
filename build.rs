fn main() {
    println!("cargo:rustc-link-lib=static=mysqlclient");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
}
