load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library", "rust_test")

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
        "//nsasm",
    ],
)

rust_test(
    name = "midi2spc_unit_tests",
    size = "small",
    crate = ":midi2spc",
)

rust_test(
    name = "midi2spc_integration_tests",
    size = "medium",
    srcs = ["tests/test.rs"],
    data = ["//midi2spc/sample:sample_files"],
    deps = [":midi2spc"],
)
