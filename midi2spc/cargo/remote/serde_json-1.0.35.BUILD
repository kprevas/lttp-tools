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



rust_library(
    name = "serde_json",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__itoa__0_4_3//:itoa",
        "@raze__ryu__0_2_7//:ryu",
        "@raze__serde__1_0_84//:serde",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.0.35",
    crate_features = [
        "default",
    ],
)

