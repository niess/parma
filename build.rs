fn main() {
    cc::Build::new()
        .cpp(true) 
        .file("src/PARMA/subroutines.cpp")
        .compile("parma-cpp");

    println!("cargo::rerun-if-changed=src/PARMA/subroutines.cpp");
}
