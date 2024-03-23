//extern crate cc;

fn main() {
    let mut build = cc::Build::new();
    build.compiler("clang");
    build.file("filesystem/*");
    build.no_default_flags(true);
    build.flag("-O2");
    build.compile("fs");
}