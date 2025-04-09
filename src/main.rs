use clap::{Arg, Command};
use rand::Rng;
use std::fs;
use std::path::Path;

fn is_first_run() -> bool {
    !Path::new("./config/crustagen/config.txt").exists()
}

fn mark_as_run() {
    fs::create_dir_all("./config/crustagen").unwrap_or_else(|_| {
        println!("Warning: Could not create config directory");
    });

    fs::write("./config/crustagen/config.txt", "initialized=true")
        .unwrap_or_else(|_| {
            println!("Warning: Could not write to config file");
        })
}

fn display_welcome_message() {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║                    CRUSTAGEN                       ║");
    println!("║                  Version 0.1.1                     ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!();
    println!("Welcome to crustagen, an open source password generator written in Rust.");
    println!("  - Generate a new password using the 'crustagen' command");
    println!("  - Specify password length with '--length' or '-l'");
    println!("  - Include special characters with '--special' or -'s'");
    println!("  - Turn on verbose mode using '--verbose' or '-d'");
    println!();
    println!("! Use the command HISTIGNORE='crustagen*' before running this application. It \n prevents outputs from being saved to bash history, which could introduce \n potential security liabilities. !");
}

fn main() {
    if is_first_run() {
        display_welcome_message();
        mark_as_run();

        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

    let matches = Command::new("crustagen")
    .version("0.1.1")
    .author("Glapsuidir")
    .about("Generates a random password based on user parameters.")
    .arg(
        Arg::new("length")
        .short('l')
        .long("length")
        .value_name("LENGTH")
        .help("Sets the length of the password")
        .value_parser(clap::value_parser!(usize))
        .default_value("12"),
    )
    .arg(
        Arg::new("special")
        .short('s')
        .long("special")
        .help("Include special characters in the new password")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(
        Arg::new("verbose")
        .short('d')
        .long("verbose")
        .help("Display verbose output")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(
        Arg::new("output")
        .short('o')
        .long("output")
        .help("Save the password to an encrypted file")
        .action(clap::ArgAction::SetTrue)
    )
    .get_matches();

    let length: usize = *matches
    .get_one::<usize>("length")
    .unwrap_or(&12);
    let include_special = matches.get_flag("special");
    let include_verbose = matches.get_flag("verbose");
    let save_output = matches.get_flag("output");

    let password = generate_password(length, include_special, include_verbose);

    println!("Generated password: {}", password);

    if save_output {
        println!("Once updated, output will save to a file.");
    }
    
}

fn generate_password(length: usize, include_special: bool, include_verbose: bool) -> String {
    let mut rng = rand::thread_rng();

    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let numbers = "0123456789";
    let specials = "!@#$%^&*()-_=+[]{}|;:,.<>?/";

    let mut charset = format!("{}{}", letters, numbers);
    if include_special {
        if include_verbose {
            println!("Adding special characters to the charset... Done.");
        }
        charset.push_str(specials);
    } else {
        if include_verbose {
            println!("Special characters are not present in the charset.");
        } 
    }

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}