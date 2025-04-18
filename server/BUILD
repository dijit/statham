load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("@crate_server//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

package(default_visibility = ["//visibility:public"])

exports_files([
    "Cargo.toml",
    "Cargo.lock",
])

rust_binary(
    name = "server",
    srcs = [
        "src/db/mod.rs",
        "src/main.rs",
    ],
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True),
    #deps = [
    #    "//cargo:fern",
    #    "//cargo:log",
    #    "//cargo:rocket",
    #    "//cargo:rocket_sync_db_pools",
    #],
)

load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

pkg_tar(
    name = "tar",
    srcs = [":server"],
    visibility = ["//visibility:private"]
)
oci_image(
    name = "image",
    base = "@docker_lib_debian",
    tars = [":tar"],
    entrypoint = ["/server"],
    visibility = ["//visibility:private"]
)

oci_load(
    name = "image_load",
    image = ":image",
    repo_tags = ["statham_server:latest"],
)
