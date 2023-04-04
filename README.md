# bin_int_compare

bin_int_compare is a Rust utility that compares two binary files based on hexadecimal representations of input integers. The script takes two binary files and two integers as input and returns the output in JSON format.

## Features

-   Reads binary files and converts them to hexadecimal representation
-   Compares addresses based on given integer values
-   Outputs comparison results in JSON format

## build Instructions

To build and install the bin_int_compare tool, make sure you have Rust installed on your machine. Then, follow these steps:

Clone the repository:

```sh
git clone https://github.com/bad3r/bin_int_compare.git
```

Change to the bin_int_compare directory:

```sh
cd bin_int_compare
```

Build the project:

```sh
cargo build --release
```

The binary will be generated in the `target/release` directory.

## Usage

To use the bin_int_compare tool, run the following command:

```sh
bin_int_compare file0.dat file1.dat int1 int2
```

Replace file0.dat and file1.dat with the paths to your binary files, and int1 and int with the integer values you want to compare.

The output will be a JSON array containing objects with the following structure:

```json
{
    "address": "00000001",
    "data0": "0400 05ca 0000 0d4s 034c 340c 50e5 3040",
    "data1": "00c2 056a 580b dfe0 0000 0055 a627 e540"
}
```

Each object in the array represents a match found between the two files, with:

-   `address`: The hexadecimal address offset where the match occurred.
-   `data0`: The line from the first file containing `int0` in hexadecimal format.
-   `data1`: The line from the second file containing `int1` in hexadecimal format.

## Test case

The provided [test.bin.c](./test_bin.c) file can be used to generate two testing binary files.
Compile the file using the following command:

```sh
$ gcc -o 0x4d.bin -DBYTE_VALUE=77 test_bin.c
$ gcc -o 0x21.bin -DBYTE_VALUE=33 test_bin.c
```

This will generate two binary files, `0x4d.bin` and `0x21.bin`, with the same contents except for the byte value at address `0x00000001`, which is set to 77 (`0x4D`) in `0x4d.bin` and 33 (`0x21`) in `0x21.bin`.

### Expected output

```json
❯ cargo run --release -- ./0x4d.bin ./0x21.bin 77 33 | jq .
[
  {
    "data1": "10 c7 45 fc 21 00 00 00 8b 45 fc 89 c6 48 8d 05",
    "data0": "10 c7 45 fc 4d 00 00 00 8b 45 fc 89 c6 48 8d 05",
    "address": "00001140"
  }
]
```

## Technical Breakdown

The `bin_int_compare` performs the following steps:

1. Reads the binary files using the `read_binary_file` function, which reads the contents of the files into `Vec<u8>` buffers.
2. Converts the binary data into hexadecimal representation using the `binary_to_hex` function, which processes the binary data in chunks and creates a vector of tuples containing the address offsets and the corresponding hexadecimal lines.
3. Compares the files using the `compare_files` function, which takes the paths to the two files and the integer values to compare as arguments. The function creates hashmaps from the vectors of tuples generated in step 2 and iterates through the first hashmap, checking if the lines contain the given integer values (converted to hexadecimal). If a match is found, the relevant information is stored in a vector.
4. Outputs the comparison results in JSON format using the `serde_json` crate.

The main function handles command-line arguments, error handling, and output formatting. The tool uses Rust's efficient memory handling and processing capabilities to provide fast file reading and comparison.

## License

`bin_int_compare` is released under the MIT License. Please see the [LICENSE](./LICENSE) file for more information.
