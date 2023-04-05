use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn read_binary_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn binary_data_to_chunks(binary_data: &[u8]) -> Vec<(String, &[u8])> {
    binary_data
        .chunks(16)
        .enumerate()
        .map(|(i, chunk)| {
            let address = format!("{:08x}", i * 16);
            (address, chunk)
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct MatchInfo {
    address: String,
    data0: String,
    data1: String,
}

fn compare_chunks(chunk0: &[u8], chunk1: &[u8], target_val1: u8, target_val2: u8) -> bool {
    chunk0.iter().zip(chunk1.iter()).any(|(&byte0, &byte1)| {
        (byte0 == target_val1 && byte1 == target_val2)
            || (byte0 == target_val2 && byte1 == target_val1)
    })
}

fn compare_files(data0: &[u8], data1: &[u8], target_val1: u8, target_val2: u8) -> Vec<MatchInfo> {
    let chunks0 = binary_data_to_chunks(data0);
    let chunks1 = binary_data_to_chunks(data1);

    let chunks_dict0: HashMap<_, _> = chunks0.into_iter().collect();
    let chunks_dict1: HashMap<_, _> = chunks1.into_iter().collect();

    let mut matches_info = vec![];

    for (address, chunk0) in &chunks_dict0 {
        if let Some(chunk1) = chunks_dict1.get(address) {
            if compare_chunks(chunk0, chunk1, target_val1, target_val2) {
                matches_info.push(MatchInfo {
                    address: address.clone(),
                    data0: chunk0
                        .iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<Vec<_>>()
                        .join(" "),
                    data1: chunk1
                        .iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<Vec<_>>()
                        .join(" "),
                });
            }
        }
    }

    matches_info
}

fn generate_json_output(matches_info: &[MatchInfo]) -> serde_json::Result<String> {
    serde_json::to_string_pretty(matches_info)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: compare_files fp0.bin fp1.bin v0 v1");
        return Ok(());
    }

    let fp0_path = &args[1];
    let fp1_path = &args[2];
    let v0 = args[3]
        .parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid value for v0"))?;
    let v1 = args[4]
        .parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid value for v1"))?;

    let data0 = read_binary_file(fp0_path)?;
    let data1 = read_binary_file(fp1_path)?;

    let target_val1 = v0 as u8;
    let target_val2 = v1 as u8;

    let matches_info = compare_files(&data0, &data1, target_val1, target_val2);
    let output_json = generate_json_output(&matches_info)?;
    println!("{}", output_json);

    Ok(())
}
