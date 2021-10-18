// this file originally here: 
//   https://rust-lang.github.io/rust-bindgen/tutorial-3.html

extern crate bindgen;

fn main() {
    
    let src = [
        "src/cpp/InvalidEnvelopeSizeException.cpp",    
        "src/cpp/Envelope.cpp",
        "src/cpp/EnvelopeComparator.cpp",
        "src/cpp/EnvelopesContainer.cpp",
        "src/cpp/EnvelopesAnalysis.cpp",
    ];
    
    println!("cargo:rerun-if-changed=src/wrapper.hpp");

    cc::Build::new()
        .cpp(true)    
        .files(src.iter())
        //.file("src/mymath.cpp")
        .compile("cpp_envelopes_analysis");
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/cpp_envelopes_analysis.rs")
        .expect("Couldn't write bindings!");

    

    
}