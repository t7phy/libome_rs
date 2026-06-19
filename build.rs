fn main() {
    let src = "native/libome/src";

    let cpp_files = [
        "ome/AqqQNSEven.cpp",
        "ome/AqqQNSOdd.cpp",
        "ome/AQqPS.cpp",
        "ome/AQqPSs.cpp",
        "ome/AqqQPS.cpp",
        "ome/AqgQ.cpp",
        "ome/AgqQ.cpp",
        "ome/AggQ.cpp",
        "ome/AQg.cpp",
        "ome/polAqqQNSEven.cpp",
        "ome/polAqqQNSOdd.cpp",
        "ome/polAQqPS.cpp",
        "ome/polAQqPSs.cpp",
        "ome/polAqqQPS.cpp",
        "ome/polAqgQ.cpp",
        "ome/polAgqQ.cpp",
        "ome/polAggQ.cpp",
        "ome/polAQg.cpp",
    ];

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include(src)
        .files(cpp_files.iter().map(|f| format!("{src}/{f}")));

    if std::env::var("CARGO_FEATURE_MELLIN").is_ok() {
        let gsl = pkg_config::probe_library("gsl").expect(
            "GSL not found, but feature \"mellin\" was enabled. \
             Install libgsl-dev on Debian/Ubuntu, gsl via Homebrew on macOS, \
             or disable the \"mellin\" feature.",
        );
        for inc in &gsl.include_paths {
            build.include(inc);
        }
        build.file(format!("{src}/ome/integration_engine_gsl.cpp"));
        build.file("src/gsl_shim.cpp");
    }

    build.compile("ome");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/gsl_shim.cpp");
    println!("cargo:rerun-if-changed={src}/ome");
    println!("cargo:rerun-if-changed={src}/ome/ome.h");
    println!("cargo:rerun-if-changed={src}/ome/mellin.h");
    println!("cargo:rerun-if-changed={src}/ome/integration_engine_gsl.h");
}
