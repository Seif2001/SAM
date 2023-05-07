extern crate cmake;

use cmake::Config;

fn main(){
    let dist = Config::new("systemcall").build();

    println!("cargo:rustc-link-search=native={}", dist.display());
    println!("cargo:rustc-link-lib=static=systemcall");

}