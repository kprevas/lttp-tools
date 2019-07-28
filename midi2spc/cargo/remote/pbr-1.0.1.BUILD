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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "lib" with type "test" omitted
# Unsupported target "max_refresh_rate" with type "example" omitted
# Unsupported target "multi" with type "example" omitted
# Unsupported target "npm_bar" with type "example" omitted

rust_library(
    name = "pbr",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__libc__0_2_46//:libc",
        "@raze__time__0_1_42//:time",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.0.1",
    crate_features = [
    ],
)

# Unsupported target "simple" with type "example" omitted
