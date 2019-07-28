"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//midi2spc/cargo", which limits external
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


# Unsupported target "example" with type "example" omitted

rust_library(
    name = "ghakuf",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__byteorder__1_2_7//:byteorder",
        "@raze__log__0_3_9//:log",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.1",
    crate_features = [
    ],
)

# Unsupported target "test" with type "test" omitted
