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
        name = "raze__ansi_term__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.11.0.crate",
        type = "tar.gz",
        sha256 = "ee49baf6cb617b853aa8d93bf420db2383fab46d314482ca2803b40d5fde979b",
        strip_prefix = "ansi_term-0.11.0",
        build_file = Label("//midi2spc/cargo/remote:ansi_term-0.11.0.BUILD")
    )

    _new_http_archive(
        name = "raze__atty__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.11.crate",
        type = "tar.gz",
        sha256 = "9a7d5b8723950951411ee34d271d99dddcc2035a16ab25310ea2c8cfd4369652",
        strip_prefix = "atty-0.2.11",
        build_file = Label("//midi2spc/cargo/remote:atty-0.2.11.BUILD")
    )

    _new_http_archive(
        name = "raze__bitflags__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.4.crate",
        type = "tar.gz",
        sha256 = "228047a76f468627ca71776ecdebd732a3423081fcf5125585bcd7c49886ce12",
        strip_prefix = "bitflags-1.0.4",
        build_file = Label("//midi2spc/cargo/remote:bitflags-1.0.4.BUILD")
    )

    _new_http_archive(
        name = "raze__byteorder__1_2_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-1.2.7.crate",
        type = "tar.gz",
        sha256 = "94f88df23a25417badc922ab0f5716cc1330e87f71ddd9203b3a3ccd9cedf75d",
        strip_prefix = "byteorder-1.2.7",
        build_file = Label("//midi2spc/cargo/remote:byteorder-1.2.7.BUILD")
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.6.crate",
        type = "tar.gz",
        sha256 = "082bb9b28e00d3c9d39cc03e64ce4cea0f1bb9b3fde493f0cbc008472d22bdf4",
        strip_prefix = "cfg-if-0.1.6",
        build_file = Label("//midi2spc/cargo/remote:cfg-if-0.1.6.BUILD")
    )

    _new_http_archive(
        name = "raze__clap__2_32_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-2.32.0.crate",
        type = "tar.gz",
        sha256 = "b957d88f4b6a63b9d70d5f454ac8011819c6efa7727858f458ab71c756ce2d3e",
        strip_prefix = "clap-2.32.0",
        build_file = Label("//midi2spc/cargo/remote:clap-2.32.0.BUILD")
    )

    _new_http_archive(
        name = "raze__either__1_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/either/either-1.5.0.crate",
        type = "tar.gz",
        sha256 = "3be565ca5c557d7f59e7cfcf1844f9e3033650c929c6566f511e8005f205c1d0",
        strip_prefix = "either-1.5.0",
        build_file = Label("//midi2spc/cargo/remote:either-1.5.0.BUILD")
    )

    _new_http_archive(
        name = "raze__ghakuf__0_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ghakuf/ghakuf-0.5.1.crate",
        type = "tar.gz",
        sha256 = "13e9c67552cb8e25aea67430d003303a8bc618f6b1f20f18c2f21d37752362f5",
        strip_prefix = "ghakuf-0.5.1",
        build_file = Label("//midi2spc/cargo/remote:ghakuf-0.5.1.BUILD")
    )

    _new_http_archive(
        name = "raze__itertools__0_7_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.7.11.crate",
        type = "tar.gz",
        sha256 = "0d47946d458e94a1b7bcabbf6521ea7c037062c81f534615abcad76e84d4970d",
        strip_prefix = "itertools-0.7.11",
        build_file = Label("//midi2spc/cargo/remote:itertools-0.7.11.BUILD")
    )

    _new_http_archive(
        name = "raze__itoa__0_4_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itoa/itoa-0.4.3.crate",
        type = "tar.gz",
        sha256 = "1306f3464951f30e30d12373d31c79fbd52d236e5e896fd92f96ec7babbbe60b",
        strip_prefix = "itoa-0.4.3",
        build_file = Label("//midi2spc/cargo/remote:itoa-0.4.3.BUILD")
    )

    _new_http_archive(
        name = "raze__kernel32_sys__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/kernel32-sys/kernel32-sys-0.2.2.crate",
        type = "tar.gz",
        sha256 = "7507624b29483431c0ba2d82aece8ca6cdba9382bff4ddd0f7490560c056098d",
        strip_prefix = "kernel32-sys-0.2.2",
        build_file = Label("//midi2spc/cargo/remote:kernel32-sys-0.2.2.BUILD")
    )

    _new_http_archive(
        name = "raze__libc__0_2_46",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.46.crate",
        type = "tar.gz",
        sha256 = "023a4cd09b2ff695f9734c1934145a315594b7986398496841c7031a5a1bbdbd",
        strip_prefix = "libc-0.2.46",
        build_file = Label("//midi2spc/cargo/remote:libc-0.2.46.BUILD")
    )

    _new_http_archive(
        name = "raze__log__0_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.3.9.crate",
        type = "tar.gz",
        sha256 = "e19e8d5c34a3e0e2223db8e060f9e8264aeeb5c5fc64a4ee9965c062211c024b",
        strip_prefix = "log-0.3.9",
        build_file = Label("//midi2spc/cargo/remote:log-0.3.9.BUILD")
    )

    _new_http_archive(
        name = "raze__log__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.6.crate",
        type = "tar.gz",
        sha256 = "c84ec4b527950aa83a329754b01dbe3f58361d1c5efacd1f6d68c494d08a17c6",
        strip_prefix = "log-0.4.6",
        build_file = Label("//midi2spc/cargo/remote:log-0.4.6.BUILD")
    )

    _new_http_archive(
        name = "raze__pbr__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pbr/pbr-1.0.1.crate",
        type = "tar.gz",
        sha256 = "deb73390ab68d81992bd994d145f697451bb0b54fd39738e72eef32458ad6907",
        strip_prefix = "pbr-1.0.1",
        build_file = Label("//midi2spc/cargo/remote:pbr-1.0.1.BUILD")
    )

    _new_http_archive(
        name = "raze__proc_macro2__0_4_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.4.24.crate",
        type = "tar.gz",
        sha256 = "77619697826f31a02ae974457af0b29b723e5619e113e9397b8b82c6bd253f09",
        strip_prefix = "proc-macro2-0.4.24",
        build_file = Label("//midi2spc/cargo/remote:proc-macro2-0.4.24.BUILD")
    )

    _new_http_archive(
        name = "raze__quote__0_6_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.6.10.crate",
        type = "tar.gz",
        sha256 = "53fa22a1994bd0f9372d7a816207d8a2677ad0325b073f5c5332760f0fb62b5c",
        strip_prefix = "quote-0.6.10",
        build_file = Label("//midi2spc/cargo/remote:quote-0.6.10.BUILD")
    )

    _new_http_archive(
        name = "raze__redox_syscall__0_1_50",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.50.crate",
        type = "tar.gz",
        sha256 = "52ee9a534dc1301776eff45b4fa92d2c39b1d8c3d3357e6eb593e0d795506fc2",
        strip_prefix = "redox_syscall-0.1.50",
        build_file = Label("//midi2spc/cargo/remote:redox_syscall-0.1.50.BUILD")
    )

    _new_http_archive(
        name = "raze__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        sha256 = "7e891cfe48e9100a70a3b6eb652fef28920c117d366339687bd5576160db0f76",
        strip_prefix = "redox_termios-0.1.1",
        build_file = Label("//midi2spc/cargo/remote:redox_termios-0.1.1.BUILD")
    )

    _new_http_archive(
        name = "raze__ryu__0_2_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ryu/ryu-0.2.7.crate",
        type = "tar.gz",
        sha256 = "eb9e9b8cde282a9fe6a42dd4681319bfb63f121b8a8ee9439c6f4107e58a46f7",
        strip_prefix = "ryu-0.2.7",
        build_file = Label("//midi2spc/cargo/remote:ryu-0.2.7.BUILD")
    )

    _new_http_archive(
        name = "raze__serde__1_0_84",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde/serde-1.0.84.crate",
        type = "tar.gz",
        sha256 = "0e732ed5a5592c17d961555e3b552985baf98d50ce418b7b655f31f6ba7eb1b7",
        strip_prefix = "serde-1.0.84",
        build_file = Label("//midi2spc/cargo/remote:serde-1.0.84.BUILD")
    )

    _new_http_archive(
        name = "raze__serde_derive__1_0_84",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_derive/serde_derive-1.0.84.crate",
        type = "tar.gz",
        sha256 = "b4d6115a3ca25c224e409185325afc16a0d5aaaabc15c42b09587d6f1ba39a5b",
        strip_prefix = "serde_derive-1.0.84",
        build_file = Label("//midi2spc/cargo/remote:serde_derive-1.0.84.BUILD")
    )

    _new_http_archive(
        name = "raze__serde_json__1_0_35",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_json/serde_json-1.0.35.crate",
        type = "tar.gz",
        sha256 = "dfb1277d4d0563e4593e0b8b5d23d744d277b55d2bc0bf1c38d0d8a6589d38aa",
        strip_prefix = "serde_json-1.0.35",
        build_file = Label("//midi2spc/cargo/remote:serde_json-1.0.35.BUILD")
    )

    _new_http_archive(
        name = "raze__simple_error__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/simple-error/simple-error-0.2.1.crate",
        type = "tar.gz",
        sha256 = "339844c9af2d844b9230bb28e8f819a7790cbf20a29b5cbd2b59916a03a1ef51",
        strip_prefix = "simple-error-0.2.1",
        build_file = Label("//midi2spc/cargo/remote:simple-error-0.2.1.BUILD")
    )

    _new_http_archive(
        name = "raze__strsim__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.7.0.crate",
        type = "tar.gz",
        sha256 = "bb4f380125926a99e52bc279241539c018323fab05ad6368b56f93d9369ff550",
        strip_prefix = "strsim-0.7.0",
        build_file = Label("//midi2spc/cargo/remote:strsim-0.7.0.BUILD")
    )

    _new_http_archive(
        name = "raze__syn__0_15_24",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.15.24.crate",
        type = "tar.gz",
        sha256 = "734ecc29cd36e8123850d9bf21dfd62ef8300aaa8f879aabaa899721808be37c",
        strip_prefix = "syn-0.15.24",
        build_file = Label("//midi2spc/cargo/remote:syn-0.15.24.BUILD")
    )

    _new_http_archive(
        name = "raze__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        sha256 = "689a3bdfaab439fd92bc87df5c4c78417d3cbe537487274e9b0b2dce76e92096",
        strip_prefix = "termion-1.5.1",
        build_file = Label("//midi2spc/cargo/remote:termion-1.5.1.BUILD")
    )

    _new_http_archive(
        name = "raze__textwrap__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.10.0.crate",
        type = "tar.gz",
        sha256 = "307686869c93e71f94da64286f9a9524c0f308a9e1c87a583de8e9c9039ad3f6",
        strip_prefix = "textwrap-0.10.0",
        build_file = Label("//midi2spc/cargo/remote:textwrap-0.10.0.BUILD")
    )

    _new_http_archive(
        name = "raze__time__0_1_42",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.42.crate",
        type = "tar.gz",
        sha256 = "db8dcfca086c1143c9270ac42a2bbd8a7ee477b78ac8e45b19abfb0cbede4b6f",
        strip_prefix = "time-0.1.42",
        build_file = Label("//midi2spc/cargo/remote:time-0.1.42.BUILD")
    )

    _new_http_archive(
        name = "raze__unicode_width__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.5.crate",
        type = "tar.gz",
        sha256 = "882386231c45df4700b275c7ff55b6f3698780a650026380e72dabe76fa46526",
        strip_prefix = "unicode-width-0.1.5",
        build_file = Label("//midi2spc/cargo/remote:unicode-width-0.1.5.BUILD")
    )

    _new_http_archive(
        name = "raze__unicode_xid__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.1.0.crate",
        type = "tar.gz",
        sha256 = "fc72304796d0818e357ead4e000d19c9c174ab23dc11093ac919054d20a6a7fc",
        strip_prefix = "unicode-xid-0.1.0",
        build_file = Label("//midi2spc/cargo/remote:unicode-xid-0.1.0.BUILD")
    )

    _new_http_archive(
        name = "raze__vec_map__0_8_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.1.crate",
        type = "tar.gz",
        sha256 = "05c78687fb1a80548ae3250346c3db86a80a7cdd77bda190189f2d0a0987c81a",
        strip_prefix = "vec_map-0.8.1",
        build_file = Label("//midi2spc/cargo/remote:vec_map-0.8.1.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.2.8.crate",
        type = "tar.gz",
        sha256 = "167dc9d6949a9b857f3451275e911c3f44255842c1f7a76f33c55103a909087a",
        strip_prefix = "winapi-0.2.8",
        build_file = Label("//midi2spc/cargo/remote:winapi-0.2.8.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.6.crate",
        type = "tar.gz",
        sha256 = "92c1eb33641e276cfa214a0522acad57be5c56b10cb348b3c5117db75f3ac4b0",
        strip_prefix = "winapi-0.3.6",
        build_file = Label("//midi2spc/cargo/remote:winapi-0.3.6.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi_build__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-build/winapi-build-0.1.1.crate",
        type = "tar.gz",
        sha256 = "2d315eee3b34aca4797b2da6b13ed88266e6d612562a0c46390af8299fc699bc",
        strip_prefix = "winapi-build-0.1.1",
        build_file = Label("//midi2spc/cargo/remote:winapi-build-0.1.1.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//midi2spc/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD")
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//midi2spc/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD")
    )

