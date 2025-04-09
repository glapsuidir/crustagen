# Crustagen 🦀🔐

## Description

Crustagen is a password generation program written in Rust. It makes use of command line arguments to supply information to the program. It was created using VSCode and the cargo tool. In coming versions, I hope to implement saving passwords to an encrypted file, passphrase generation from a wordlist, an interactive TUI mode, exporting to JSON for APIs, and custom character sets.

## How to Install and Run Crustagen

There are currently two ways to install Crustagen, with a method to install using a PPA being currently in development.

### Curl

1. Input the below command into your terminal emulator of choice

```bash
curl -sSL https://raw.githubusercontent.com/glapsuidir/crustagen/master/install.sh | bash
```

2. The install script will start automatically
3. Check to ensure crustagen is present in $PATH by running ‘echo $PATH’

### Manual Compilation

1. Navigate to github.com/glapsuidir/crustagen
2. Download the binary or source code
3. Navigate to the directory the files were downloaded in and extract if necessary
4. If compiling from source, use cargo build from the project directory
5. Run the command below to add the binary to your PATH variable

```bash
cp crustagen /usr/local/bin
```

## How to Use Crustagen

Crustagen is run and used from the command line. After installing and adding binary to your PATH variable, you can run it using ‘cargo’. Otherwise, locate the crustagen binary, and run it directly (such as /home/skywalker/crustagen).

### Flags

- -d or --verbose: Controls verbose mode to show more information on command execution
- -l or --length: Manually sets length of password (default 12)
- -s or --special: Adds special characters to the charset used to generate the password
- -v or --version: Displays the installed version number
- -h or --help: Displays help information

## Credits

This project was created as a supplementary learning method in tandem with *The Rust Programming Language*, which can be found at https://doc.rust-lang.org/book/. 

## License

Copyright (c) 2025 Glapsuidir

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
