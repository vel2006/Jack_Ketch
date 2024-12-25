use std::str;
use std::{fs, fs::File};
use std::io::{BufReader, Read, stdin, stdout, Write};

fn main()
{
    // Getting user input path
    let mut input_path = String::new();
    print!("[^] Please input full path to the file you wish to get the byte data of> ");
    let _ = stdout().flush();
    stdin().read_line(&mut input_path).expect("Did not enter a correct string");
    if let Some('\n')=input_path.chars().next_back()
    {
        input_path.pop();
    }
    if let Some('\r')=input_path.chars().next_back()
    {
        input_path.pop();
    }
    println!("");
    // Getting the file's contents in the form of u8 bytes
    let target_file_path: &str = &input_path;
    let mut read_buffer: Vec<u8> = Vec::new();
    let target_file: File = File::open(target_file_path).unwrap_or_else(|_| panic!("[!] Could not open file {}", &target_file_path));
    let mut reader: BufReader<File> = BufReader::new(target_file);
    let mut file_as_bytes: Vec<u8> = Vec::new();
    match reader.read_to_end(&mut read_buffer)
    {
        Ok(read_data) => {
            if read_data == 0
            {
                panic!("[!] Data for file {} was not read", &target_file_path);
            } else {
                let file_contents: Vec<u8> = read_buffer;
                for byte in file_contents.iter()
                {
                    file_as_bytes.push(*byte);
                }
            }
        }
        Err(error) => {
            panic!("[!] Could not read file {} with return error of: {}", &target_file_path, &error);
        }
    }
    // Converting the bytes to an XOR value for no detection
    let mut file_bytes_as_xor: Vec<u8> = Vec::new();
    let key_for_xor: u8 = 255;
    for byte in file_as_bytes.iter()
    {
        key_for_xor.push(*(byte ** &key_for_xor));
    }
    println!("{:?}", file_as_bytes);
}
