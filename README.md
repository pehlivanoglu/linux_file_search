## Prerequisites
Cargo and Rust

## 1. Build/Compile
Make sure Cargo and Rust is installed. Then,

0. Open Linux terminal. (Ctrl+Alt+T)
1. Go to main directory (which is /linux_file_search).
2. Type ``` cargo build --release``` to terminal.

## 2. Install

### Installing Options
#### Setup Modes:
1. Minimal: Includes files from: /home, /bin, /usr, /root. ```--setup_mode minimal```
2. Standard: Includes files from: /home, /bin, /usr, /var, /cdrom, /etc, /media, /sbin, /srv, /root ```--setup_mode standar```
3. Maximal: Includes files from every directory except (excludes): /proc, /run, /lost+found, /tmp, /dev ```--setup_mode maximal```
4. Default: Includes files from directories listed with ``` --include ``` parameter. Example usage: ```--setup_mode default --include /home/user/Desktop```


#### Include Option:
Includes files from directories listed with ``` --include ``` parameter.
Example usage: ```--include /home/user/Downloads /var /home/user/Desktop```

#### Add Hidden Directories and Files Flag:
To add hidden directories and hidden files.
Usage: ```--add_hidden```


### Install with Database Setup Parameters
Type ```sudo ./target/release/setup --setup_mode <choose setup mode from installing options section> --include <desired dirs to add to database>```.
If you want to add hidden directories and hidden files, also add ``` --add_hidden ``` flag.

### Or, Install without Setup Mode (Minimal setup mode)
Type ```sudo ./target/release/setup```.
If you want to add hidden directories and hidden files, also add ``` --add_hidden ``` flag.

### Install Search Binary
Type ``` sudo cp ./target/release/search /usr/local/bin/  ```.
Then, type ``` sudo chmod +x /usr/local/bin/search```

## Usage
Examples:
1. To copy a file named test.txt (whose name is known but path is unknown) to current working directory:
```sudo search "cp [test.txt] ./" ```
If the program finds multiple test.txt files, it lists the all files named test.txt, then allows you to choose one.
![Alt text](md_images/list.png)



2. To write the content of a file named Makefile (whose name is known but path is unknown) to another file named XYZ (whose name is known but path is unknown):
   ```sudo search "cat [Makefile] > [XYZ]" ```
   If the program finds multiple Makefile and XYZ files, it lists the all files named Makefile and XYZ, then allows you to choose one.
   ![Alt text](md_images/list1.png)
   ![Alt text](md_images/list2.png)

3. To compile and execute a C++ source file named test.cpp (whose name is known but path is unknown):
   ```sudo search "g++ [test.cpp] -o compile_test" ```
   ```sudo search "[compile_test]"```
   ![Alt text](md_images/list3.png)

all binaries must be executed as superuser

keeps special characters in a single database named _.db ( #, *, &, etc.)
## generated files:
daemon files at /tmp
config file at /etc/file_search/
db files at /var/lib/file_search/

## how to daemonize
put */2 * * * * /home/pehlivanoglu/CS350/project/linux_file_search/target/release/update
to crontab -e

