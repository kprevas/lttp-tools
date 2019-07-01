load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "lttp_tilepatch",
    srcs = ["src/main.rs"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//tilepatch/cargo:clap",
        "//tilepatch/cargo:env_logger",
        "//tilepatch/cargo:itertools",
        "//tilepatch/cargo:log",
        "//tilepatch/cargo:png",
        "//tilepatch/cargo:prefix_tree",
        "//tilepatch/cargo:simple_error",
        "//tilepatch/cargo:termion",
    ],
)
