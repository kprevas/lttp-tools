"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__aho_corasick__0_7_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.7.4.crate",
        type = "tar.gz",
        sha256 = "36b7aa1ccb7d7ea3f437cf025a2ab1c47cc6c1bc9fc84918ff449def12f5e282",
        strip_prefix = "aho-corasick-0.7.4",
        build_file = Label("//textconvert/cargo/remote:aho-corasick-0.7.4.BUILD")
    )

    _new_http_archive(
        name = "raze__ansi_term__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.11.0.crate",
        type = "tar.gz",
        sha256 = "ee49baf6cb617b853aa8d93bf420db2383fab46d314482ca2803b40d5fde979b",
        strip_prefix = "ansi_term-0.11.0",
        build_file = Label("//textconvert/cargo/remote:ansi_term-0.11.0.BUILD")
    )

    _new_http_archive(
        name = "raze__atty__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.11.crate",
        type = "tar.gz",
        sha256 = "9a7d5b8723950951411ee34d271d99dddcc2035a16ab25310ea2c8cfd4369652",
        strip_prefix = "atty-0.2.11",
        build_file = Label("//textconvert/cargo/remote:atty-0.2.11.BUILD")
    )

    _new_http_archive(
        name = "raze__bimap__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bimap/bimap-0.3.1.crate",
        type = "tar.gz",
        sha256 = "446177e5c72eca392a27926194ceb51a6d3f6a855ef56a490fca096e93f9971f",
        strip_prefix = "bimap-0.3.1",
        build_file = Label("//textconvert/cargo/remote:bimap-0.3.1.BUILD")
    )

    _new_http_archive(
        name = "raze__bitflags__1_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.1.0.crate",
        type = "tar.gz",
        sha256 = "3d155346769a6855b86399e9bc3814ab343cd3d62c7e985113d46a0ec3c281fd",
        strip_prefix = "bitflags-1.1.0",
        build_file = Label("//textconvert/cargo/remote:bitflags-1.1.0.BUILD")
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.9.crate",
        type = "tar.gz",
        sha256 = "b486ce3ccf7ffd79fdeb678eac06a9e6c09fc88d33836340becb8fffe87c5e33",
        strip_prefix = "cfg-if-0.1.9",
        build_file = Label("//textconvert/cargo/remote:cfg-if-0.1.9.BUILD")
    )

    _new_http_archive(
        name = "raze__clap__2_33_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-2.33.0.crate",
        type = "tar.gz",
        sha256 = "5067f5bb2d80ef5d68b4c87db81601f0b75bca627bc2ef76b141d7b846a3c6d9",
        strip_prefix = "clap-2.33.0",
        build_file = Label("//textconvert/cargo/remote:clap-2.33.0.BUILD")
    )

    _new_http_archive(
        name = "raze__either__1_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/either/either-1.5.2.crate",
        type = "tar.gz",
        sha256 = "5527cfe0d098f36e3f8839852688e63c8fff1c90b2b405aef730615f9a7bcf7b",
        strip_prefix = "either-1.5.2",
        build_file = Label("//textconvert/cargo/remote:either-1.5.2.BUILD")
    )

    _new_http_archive(
        name = "raze__itertools__0_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.8.0.crate",
        type = "tar.gz",
        sha256 = "5b8467d9c1cebe26feb08c640139247fac215782d35371ade9a2136ed6085358",
        strip_prefix = "itertools-0.8.0",
        build_file = Label("//textconvert/cargo/remote:itertools-0.8.0.BUILD")
    )

    _new_http_archive(
        name = "raze__itoa__0_4_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itoa/itoa-0.4.4.crate",
        type = "tar.gz",
        sha256 = "501266b7edd0174f8530248f87f99c88fbe60ca4ef3dd486835b8d8d53136f7f",
        strip_prefix = "itoa-0.4.4",
        build_file = Label("//textconvert/cargo/remote:itoa-0.4.4.BUILD")
    )

    _new_http_archive(
        name = "raze__lazy_static__1_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.3.0.crate",
        type = "tar.gz",
        sha256 = "bc5729f27f159ddd61f4df6228e827e86643d4d3e7c32183cb30a1c08f604a14",
        strip_prefix = "lazy_static-1.3.0",
        build_file = Label("//textconvert/cargo/remote:lazy_static-1.3.0.BUILD")
    )

    _new_http_archive(
        name = "raze__libc__0_2_58",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.58.crate",
        type = "tar.gz",
        sha256 = "6281b86796ba5e4366000be6e9e18bf35580adf9e63fbe2294aadb587613a319",
        strip_prefix = "libc-0.2.58",
        build_file = Label("//textconvert/cargo/remote:libc-0.2.58.BUILD")
    )

    _new_http_archive(
        name = "raze__memchr__2_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.2.0.crate",
        type = "tar.gz",
        sha256 = "2efc7bc57c883d4a4d6e3246905283d8dae951bb3bd32f49d6ef297f546e1c39",
        strip_prefix = "memchr-2.2.0",
        build_file = Label("//textconvert/cargo/remote:memchr-2.2.0.BUILD")
    )

    _new_http_archive(
        name = "raze__numtoa__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/numtoa/numtoa-0.1.0.crate",
        type = "tar.gz",
        sha256 = "b8f8bdf33df195859076e54ab11ee78a1b208382d3a26ec40d142ffc1ecc49ef",
        strip_prefix = "numtoa-0.1.0",
        build_file = Label("//textconvert/cargo/remote:numtoa-0.1.0.BUILD")
    )

    _new_http_archive(
        name = "raze__redox_syscall__0_1_54",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.54.crate",
        type = "tar.gz",
        sha256 = "12229c14a0f65c4f1cb046a3b52047cdd9da1f4b30f8a39c5063c8bae515e252",
        strip_prefix = "redox_syscall-0.1.54",
        build_file = Label("//textconvert/cargo/remote:redox_syscall-0.1.54.BUILD")
    )

    _new_http_archive(
        name = "raze__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        sha256 = "7e891cfe48e9100a70a3b6eb652fef28920c117d366339687bd5576160db0f76",
        strip_prefix = "redox_termios-0.1.1",
        build_file = Label("//textconvert/cargo/remote:redox_termios-0.1.1.BUILD")
    )

    _new_http_archive(
        name = "raze__regex__1_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-1.1.8.crate",
        type = "tar.gz",
        sha256 = "1325e8a57b7da4cbcb38b3957112f729990bad0a18420e7e250ef6b1d9a15763",
        strip_prefix = "regex-1.1.8",
        build_file = Label("//textconvert/cargo/remote:regex-1.1.8.BUILD")
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_6_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.6.7.crate",
        type = "tar.gz",
        sha256 = "9d76410686f9e3a17f06128962e0ecc5755870bb890c34820c7af7f1db2e1d48",
        strip_prefix = "regex-syntax-0.6.7",
        build_file = Label("//textconvert/cargo/remote:regex-syntax-0.6.7.BUILD")
    )

    _new_http_archive(
        name = "raze__ryu__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ryu/ryu-1.0.0.crate",
        type = "tar.gz",
        sha256 = "c92464b447c0ee8c4fb3824ecc8383b81717b9f1e74ba2e72540aef7b9f82997",
        strip_prefix = "ryu-1.0.0",
        build_file = Label("//textconvert/cargo/remote:ryu-1.0.0.BUILD")
    )

    _new_http_archive(
        name = "raze__serde__1_0_94",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde/serde-1.0.94.crate",
        type = "tar.gz",
        sha256 = "076a696fdea89c19d3baed462576b8f6d663064414b5c793642da8dfeb99475b",
        strip_prefix = "serde-1.0.94",
        build_file = Label("//textconvert/cargo/remote:serde-1.0.94.BUILD")
    )

    _new_http_archive(
        name = "raze__serde_json__1_0_40",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_json/serde_json-1.0.40.crate",
        type = "tar.gz",
        sha256 = "051c49229f282f7c6f3813f8286cc1e3323e8051823fce42c7ea80fe13521704",
        strip_prefix = "serde_json-1.0.40",
        build_file = Label("//textconvert/cargo/remote:serde_json-1.0.40.BUILD")
    )

    _new_http_archive(
        name = "raze__simple_error__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/simple-error/simple-error-0.2.0.crate",
        type = "tar.gz",
        sha256 = "30dec844a00c634b23676fa33f9833610148e13fbd679ec5ecce11d25fb1d213",
        strip_prefix = "simple-error-0.2.0",
        build_file = Label("//textconvert/cargo/remote:simple-error-0.2.0.BUILD")
    )

    _new_http_archive(
        name = "raze__strsim__0_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.8.0.crate",
        type = "tar.gz",
        sha256 = "8ea5119cdb4c55b55d432abb513a0429384878c15dde60cc77b1c99de1a95a6a",
        strip_prefix = "strsim-0.8.0",
        build_file = Label("//textconvert/cargo/remote:strsim-0.8.0.BUILD")
    )

    _new_http_archive(
        name = "raze__termion__1_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.3.crate",
        type = "tar.gz",
        sha256 = "6a8fb22f7cde82c8220e5aeacb3258ed7ce996142c77cba193f203515e26c330",
        strip_prefix = "termion-1.5.3",
        build_file = Label("//textconvert/cargo/remote:termion-1.5.3.BUILD")
    )

    _new_http_archive(
        name = "raze__textwrap__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.11.0.crate",
        type = "tar.gz",
        sha256 = "d326610f408c7a4eb6f51c37c330e496b08506c9457c9d34287ecc38809fb060",
        strip_prefix = "textwrap-0.11.0",
        build_file = Label("//textconvert/cargo/remote:textwrap-0.11.0.BUILD")
    )

    _new_http_archive(
        name = "raze__thread_local__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.6.crate",
        type = "tar.gz",
        sha256 = "c6b53e329000edc2b34dbe8545fd20e55a333362d0a321909685a19bd28c3f1b",
        strip_prefix = "thread_local-0.3.6",
        build_file = Label("//textconvert/cargo/remote:thread_local-0.3.6.BUILD")
    )

    _new_http_archive(
        name = "raze__ucd_util__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.3.crate",
        type = "tar.gz",
        sha256 = "535c204ee4d8434478593480b8f86ab45ec9aae0e83c568ca81abf0fd0e88f86",
        strip_prefix = "ucd-util-0.1.3",
        build_file = Label("//textconvert/cargo/remote:ucd-util-0.1.3.BUILD")
    )

    _new_http_archive(
        name = "raze__unicode_width__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.5.crate",
        type = "tar.gz",
        sha256 = "882386231c45df4700b275c7ff55b6f3698780a650026380e72dabe76fa46526",
        strip_prefix = "unicode-width-0.1.5",
        build_file = Label("//textconvert/cargo/remote:unicode-width-0.1.5.BUILD")
    )

    _new_http_archive(
        name = "raze__utf8_ranges__1_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.3.crate",
        type = "tar.gz",
        sha256 = "9d50aa7650df78abf942826607c62468ce18d9019673d4a2ebe1865dbb96ffde",
        strip_prefix = "utf8-ranges-1.0.3",
        build_file = Label("//textconvert/cargo/remote:utf8-ranges-1.0.3.BUILD")
    )

    _new_http_archive(
        name = "raze__vec_map__0_8_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.1.crate",
        type = "tar.gz",
        sha256 = "05c78687fb1a80548ae3250346c3db86a80a7cdd77bda190189f2d0a0987c81a",
        strip_prefix = "vec_map-0.8.1",
        build_file = Label("//textconvert/cargo/remote:vec_map-0.8.1.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi__0_3_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.7.crate",
        type = "tar.gz",
        sha256 = "f10e386af2b13e47c89e7236a7a14a086791a2b88ebad6df9bf42040195cf770",
        strip_prefix = "winapi-0.3.7",
        build_file = Label("//textconvert/cargo/remote:winapi-0.3.7.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//textconvert/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//textconvert/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD")
    )

