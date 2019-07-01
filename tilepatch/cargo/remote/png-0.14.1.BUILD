"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//tilepatch/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "png",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__1_1_0//:bitflags",
        "@raze__deflate__0_7_19//:deflate",
        "@raze__inflate__0_4_5//:inflate",
        "@raze__num_iter__0_1_39//:num_iter",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.14.1",
    crate_features = [
        "default",
        "deflate",
        "png-encoding",
    ],
)

# Unsupported target "pngcheck" with type "example" omitted
# Unsupported target "show" with type "example" omitted
