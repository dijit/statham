load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    #We are going to call the target/binary summation
    name = "hello",
    #The list of src files it needs (just main.rs)
    srcs = ["main.rs"],
    #Any libraries/crates it depends on, for now we'll leave this blank
    deps = [],
    #The crate_root file, this would default to main.rs but we put it in for clarity
    crate_root = "main.rs",
)