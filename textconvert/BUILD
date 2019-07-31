load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "lttp_textconvert",
    srcs = ["src/main.rs"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//nsasm",
        "//textconvert/cargo:bimap",
        "//textconvert/cargo:clap",
        "//textconvert/cargo:env_logger",
        "//textconvert/cargo:itertools",
        "//textconvert/cargo:log",
        "//textconvert/cargo:regex",
        "//textconvert/cargo:serde_json",
        "//textconvert/cargo:simple_error",
        "//textconvert/cargo:unicode_segmentation",
    ],
)
