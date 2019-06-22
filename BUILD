load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "lttp_tilepatch",
    srcs = ["src/main.rs"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:clap",
        "//cargo:env_logger",
        "//cargo:itertools",
        "//cargo:log",
        "//cargo:png",
        "//cargo:prefix_tree",
        "//cargo:simple_error",
        "//cargo:termion",
    ],
)
