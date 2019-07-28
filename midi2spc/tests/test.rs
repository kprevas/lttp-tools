extern crate midi2spc;
extern crate rand;

use midi2spc::rom::DEFAULT_BANK_BASE_ADDRS;
use midi2spc::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::path::PathBuf;

fn sample_path(filename: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("sample");
    path_buf.push(filename);
    path_buf
}

fn copy_dummy_rom() -> PathBuf {
    let dummy_path = sample_path("dummy.smc");
    let copy_name: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    let copy_path = dummy_path.parent().unwrap().join(copy_name + ".smc");
    fs::copy(dummy_path, copy_path.clone()).unwrap();
    copy_path
}

#[test]
fn test_file_select() {
    write_file_select(
        sample_path("adagio-for-strings.mid").to_str().unwrap(),
        copy_dummy_rom().to_str().unwrap(),
        DEFAULT_BANK_BASE_ADDRS,
        true,
        false,
    )
    .unwrap();
}

#[test]
fn test_all_overworld() {
    write_all_overworld(
        sample_path("adagio-for-strings.mid").to_str().unwrap(),
        copy_dummy_rom().to_str().unwrap(),
        DEFAULT_BANK_BASE_ADDRS,
        true,
        false,
    )
    .unwrap();
}

#[test]
fn test_build_rom() {
    build_rom(
        sample_path("manifest.json").to_str().unwrap(),
        copy_dummy_rom().to_str().unwrap(),
        DEFAULT_BANK_BASE_ADDRS,
        true,
        false,
    )
    .unwrap();
}
