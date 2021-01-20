fn main() {
    println!("cargo:rustc-link-search=native=native");
    println!("cargo:rustc-link-lib=static=mysqlclient");
}
