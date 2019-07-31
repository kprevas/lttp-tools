use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufWriter, Write}
};
use itertools::Itertools;

pub fn write_asm(
    data: &Vec<(&str, Vec<u8>)>,
    output_asm_path: &str,
    asm_module: &str,
    start_addr: &str,
    line_len: usize,
) -> Result<(), Box<Error>> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_asm_path)?;
    let mut writer = BufWriter::new(&file);
    writeln!(&mut writer, "        .module {}", asm_module)?;
    writeln!(&mut writer)?;
    writeln!(&mut writer, "        .org ${}", start_addr)?;
    writeln!(&mut writer)?;
    for (asm_label, label_data) in data {
        writeln!(&mut writer, "{}:", asm_label)?;
        for line in &label_data.iter().chunks(line_len) {
            writeln!(
                &mut writer,
                "        .db {}",
                line.map(|byte| format!("${:02X}", byte)).join(", ")
            )?;
        }
    }
    Ok(())
}
