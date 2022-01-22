load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "statham_agent",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo:fern",
        "//cargo:log",
        "//cargo:widestring",
        "//cargo:winapi",
        "//cargo:winres",
    ],
)
