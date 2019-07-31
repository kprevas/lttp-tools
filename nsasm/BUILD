load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "nsasm",
    srcs = ["src/lib.rs"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//nsasm/cargo:itertools",
    ],
)
