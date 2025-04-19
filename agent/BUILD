#load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("@crate_index_cargo_local//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_clippy", "rust_test")

package(default_visibility = ["//visibility:public"])

exports_files([
    "Cargo.toml",
    "Cargo.lock",
])

rust_binary(
    name = "agent",
    srcs = glob(["**/*.rs"]),
    compile_data = glob(["src/words/*.txt"]),
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True),
    #deps = [
    #    "//crate:rand",
    #    "//crate:fern",
    #    "//crate:log",
    #] +
    #select({ "@platforms//os:windows":
    #   [
    #        "//crate:widestring",
    #        "//crate:scopeguard",
    #        "//crate:winapi",
    #        "//crate:winres",
    #       ],
    #       "//conditions:default": []}
    #),
)

"""
rust_clippy(
    name = "agent_clippy",
    testonly = True,
    deps = [
        ":agent",
    ],
)
"""

rust_test(
    name = "agent_test",
    crate = ":agent",
)
