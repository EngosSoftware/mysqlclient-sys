fn main() {
    println!("-lmysqlclient -lssl -lcrypto -lcrypt");
    println!("cargo:rustc-link-lib=static=mysqlclient");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=crypt");
}
