/// A script that compares two binary files based on hexadecimal representations of input integers.
/// The script takes two binary files and two integers as input, and returns the output in JSON format.
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};

/// Read the binary file at the given file path and return its contents as a Vec<u8>.
fn read_binary_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Convert the binary data into a vector of tuples containing the address and the corresponding
/// hexadecimal data.
fn binary_to_hex(binary_data: &[u8]) -> Vec<(String, String)> {
    binary_data
        .chunks(16)
        .enumerate()
        .map(|(i, chunk)| {
            let address = format!("{:08x}", i * 16);
            let hex_ln = chunk
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<_>>()
                .join(" ");
            (address, hex_ln)
        })
        .collect()
}

/// Compare two hexadecimal strings and check if they contain the target hexadecimal values.
fn compare_bytes(hex_str1: &str, hex_str2: &str, target_hex1: &str, target_hex2: &str) -> bool {
    hex_str1.contains(target_hex1) && hex_str2.contains(target_hex2)
}

/// Compare two binary files based on the hexadecimal representations of the input integers.
fn compare_files(
    fp0_path: &str,
    fp1_path: &str,
    v0: u32,
    v1: u32,
) -> io::Result<Vec<HashMap<String, String>>> {
    let v0_hex = format!("{:02x}", v0);
    let v1_hex = format!("{:02x}", v1);

    let fp0_data = read_binary_file(fp0_path)?;
    let fp1_data = read_binary_file(fp1_path)?;

    let fp0_hex = binary_to_hex(&fp0_data);
    let fp1_hex = binary_to_hex(&fp1_data);

    let fp0_dict: HashMap<_, _> = fp0_hex.into_iter().collect();
    let fp1_dict: HashMap<_, _> = fp1_hex.into_iter().collect();

    let mut matches = vec![];

    for (address, ln0) in &fp0_dict {
        if let Some(ln1) = fp1_dict.get(address) {
            if compare_bytes(ln0, ln1, &v0_hex, &v1_hex) {
                let mut match_info = HashMap::new();
                match_info.insert("address".to_string(), address.clone());
                match_info.insert("data0".to_string(), ln0.clone());
                match_info.insert("data1".to_string(), ln1.clone());
                matches.push(match_info);
            }
        }
    }

    Ok(matches)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: compare_files fp0.bin fp1.bin v0 v1");
        return Ok(());
    }

    let fp0_path = &args[1];
    let fp1_path = &args[2];
    let v0 = args[3].parse::<u32>().expect("Invalid value for v0");
    let v1 = args[4].parse::<u32>().expect("Invalid value for v1");

    let matches = compare_files(fp0_path, fp1_path, v0, v1)?;
    let output_json = serde_json::to_string_pretty(&matches).expect("Failed to serialize output");
    println!("{}", output_json);

    Ok(())
}
