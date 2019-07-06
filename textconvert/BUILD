load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "lttp_textconvert",
    srcs = ["src/main.rs"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//textconvert/cargo:bimap",
        "//textconvert/cargo:clap",
        "//textconvert/cargo:itertools",
        "//textconvert/cargo:regex",
        "//textconvert/cargo:serde_json",
        "//textconvert/cargo:simple_error",
        "//textconvert/cargo:textwrap",
    ],
)
