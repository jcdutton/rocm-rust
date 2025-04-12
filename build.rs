//use bindgen;

fn main() {
    // Link against the rocBLAS library
    println!("cargo:rustc-link-search=native=/opt/rocm/lib"); // Path to ROCm libraries
    println!("cargo:rustc-link-lib=dylib=rocblas");
    println!("cargo:rustc-link-lib=dylib=amdhip64");
    // Generate the bindings for the HIP C++ API.
    //let rocm_include_path = "/opt/rocm/include/";
    //let bindings = bindgen::Builder::default()
    //    .header(format!("{}/hip/hip_runtime_api.h", rocm_include_path))
    //    .clang_arg(format!("-D__HIP_PLATFORM_AMD__ -I{}", rocm_include_path))
    //    .generate()
    //    .expect("Unable to generate bindings");


    // Write the bindings to a Rust file
    //bindings
    //    .write_to_file("src/hip_bindings.rs")
    //    .expect("Couldn't write bindings!");
}

