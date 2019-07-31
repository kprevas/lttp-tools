load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_binary(
    name = "midi2spc_bin",
    srcs = ["src/main.rs"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        ":midi2spc",
        "//midi2spc/cargo:clap",
    ],
)

rust_library(
    name = "midi2spc",
    srcs = glob(
        ["src/**/*.rs"],
        exclude = ["src/main.rs"],
    ),
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//midi2spc/cargo:byteorder",
        "//midi2spc/cargo:clap",
        "//midi2spc/cargo:ghakuf",
        "//midi2spc/cargo:itertools",
        "//midi2spc/cargo:pbr",
        "//midi2spc/cargo:serde",
        "//midi2spc/cargo:serde_derive",
        "//midi2spc/cargo:serde_json",
        "//midi2spc/cargo:simple_error",
    ],
)
