load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("@crate_server//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_clippy", "rust_test", "rustfmt_test")

package(default_visibility = ["//visibility:public"])

exports_files([
    "Cargo.toml",
    "Cargo.lock",
])

platform(
    name = "linux_x86_64",
    constraint_values = [
        "@platforms//cpu:x86_64",
        "@platforms//os:linux",
    ],
)

platform(
    name = "darwin_aarch64",
    constraint_values = [
        "@platforms//cpu:aarch64",
        "@platforms//os:osx",
    ],
)

rust_binary(
    name = "server",
    srcs = [
        "src/db/mod.rs",
        "src/main.rs",
    ],
    # for others, use compile_data
    # this file is needed at runtime I think
    compile_data = [":Rocket.toml"],
    data = [":Rocket.toml"],
    #deps = [
    #    "//cargo:fern",
    #    "//cargo:log",
    #    "//cargo:rocket",
    #    "//cargo:rocket_sync_db_pools",
    #],
    #platform = "linux_x86_64",
    platform = "darwin_aarch64",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True),
)

rust_clippy(
    name = "server_clippy",
    testonly = True,
    deps = [
        ":server",
    ],
)

rustfmt_test(
    name = "server_rustfmt",
    targets = [":server"],
)

pkg_tar(
    name = "tar",
    srcs = [":server"],
    visibility = ["//visibility:private"],
)

oci_image(
    name = "image_debian",
    base = "@docker_lib_debian",
    entrypoint = ["/server"],
    tars = [":tar"],
    visibility = ["//visibility:private"],
)

oci_image(
    name = "image_distroless",
    base = "@distroless_base",
    entrypoint = ["/server"],
    tars = [":tar"],
    visibility = ["//visibility:private"],
)

oci_load(
    name = "image_load",
    image = ":image_distroless",
    repo_tags = ["statham_server:latest"],
)
