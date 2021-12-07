load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])
# srcs = glob(["**/*.rs"]),

rust_binary(
    name = "statham_server",
    srcs = [
        "src/db/mod.rs",
        "src/main.rs",
    ],
    deps = [
        "//cargo:fern",
        "//cargo:log",
        "//cargo:rocket",
        "//cargo:rocket_sync_db_pools",
    ],
)

"""
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
container_image(
    name = "statham_server_container",
    base = "@base//image",
    entrypoint = ["/server"],
    files = [":statham_server"],
)
"""
