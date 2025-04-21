# Jack_Ketch

## Note:

### Jack_Ketch isnot designed to be used in a corporate or illegal setting, in the event it is, I am not held responsible and all actions will fall onto the user / deployer of this software. I as the sole author, will track down anyone who uses this software or it's name in vain or for illegal purposes along with bringing to the fullest justice that any law holds.

## Description:

Jack_Ketch is a script designed to do either encryption or hashing on windows systems. The main pourpose of this script is to be used for research on Anti-viruses detection, protection measures against malware / hackers, and learn reverse engenieering. A detailed description on how Jack_Ketch works and it's different versions can be seen in each version's documentation. There are different versions based on the level of skill needed to write the file and execute correctly. The versions go from lowest to highest numbers after "Jack_Ketch", meaning version one would be "Jack_Ketch_V1", so on and so forth. Each version builds off of the last one's flaws and advancements.

## Versions:

### Version 1:

Version one is the most basic version. It starts in the current directory it is inside of, with no root / starting directory changing. It is programed in Python and is honestly, the best first test for seeing on an Anti-virus will scan python files, which in my experience seem to bypass the most commonly used Anti-viruses. This file instead of having the contents of files be reversable, version one decides to hash the files using the MD4 algorithm. While not the most secure, MD4 is the most used hashing algorithm on windows devices due to how windows holds the passwords, for more information on windows password security and MD4 hashing, watch "The Shocking Ease of Cracking Windows 11 Passwords" by Enderman.

### Version 2:

Version two is a more advanced version that is going to be displayed in a YouTube video covering MalwareBytes for a second time. This version is programed in Rust (I know, I know, I love rust though :3) and changes the directory it runs inside of to be within C:\ (root directory for Windows) and will do an XOR encryption on any file not inside of it's black list. Once it collects the file's contents it will do the XOR encryption on it, then create a new file and delete the one it got the data from. I am aware that XOR is a very easy encryption to break, and that's the point. It is not designed to be used as real malware but rather studdied to see how malware works and how an Anti-virus solution handles malware.

### Version 3:

The spesific file covered in this README / documentation is JackKetchV3.rs, not it's python counterpart JackKetchV3.py. The third version is an advancement from Version 2, this one uses multi threading over multi processing, this is due to the file being more "sneeky" (if you can call it that) through using less system resorces for the random data to be put inside of files over a basic XOR encoding. This version (as just mentioned) uses random data over XOR encoding for the file's data, meaning that no matter the original file's data there is no way to easily reverse the operation besides using backups. This script also get's prompted to the drive path to start in like Version 2, except that this one is not designed to be recursive. Instead of recursively itterating through the file system, this version gets all of the items in a directory, starts to handle the files while it goes into another directory. The full file is under review for publication due to the possible damage that could be created using it, the full release will likely never happen due to the damage it could cause, if it is to be published the file will be put within the 'Version 3' folder.
