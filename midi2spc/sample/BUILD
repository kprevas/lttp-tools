load("//:rules.bzl", "music_asm")

music_asm(
    name = "sample_music",
    manifest_json = "manifest.json",
    midis = glob(["*.mid"]),
    rom = "dummy.smc",
)

filegroup(
    name = "sample_files",
    srcs = glob(["**"]),
    visibility = ["//visibility:public"],
)