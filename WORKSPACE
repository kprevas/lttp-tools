workspace(name = "lttp_tools")

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

git_repository(
    name = "io_bazel_rules_rust",
    commit = "c06ab748bd23281d2018899f9539c1bc18467af7",
    shallow_since = "1563543940 +0200",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "bazel_skylib",
    sha256 = "eb5c57e4c12e68c0c20bc774bfbc60a568e800d025557bc4ea022c6479acc867",
    strip_prefix = "bazel-skylib-0.6.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/0.6.0.tar.gz",
)
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("//tilepatch/cargo:crates.bzl", tilepatch_fetch_remote_crates = "raze_fetch_remote_crates")

tilepatch_fetch_remote_crates()

load("//textconvert/cargo:crates.bzl", textconvert_fetch_remote_crates = "raze_fetch_remote_crates")

textconvert_fetch_remote_crates()
