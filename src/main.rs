use clap::{Arg, Command};
use rand::Rng;

fn main() {
    let matches = Command::new("crustagen")
    .version("1.0")
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
    .get_matches();

    let length: usize = *matches
    .get_one::<usize>("length")
    .unwrap_or(&12);
    let include_special = matches.get_flag("special");
    let include_verbose = matches.get_flag("verbose");

    let password = generate_password(length, include_special);

    println!("Generated password: {}", password);
    
}

fn generate_password(length: usize, include_special: bool) -> String {
    let mut rng = rand::thread_rng();

    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let numbers = "0123456789";
    let specials = "!@#$%^&*()-_=+[]{}|;:,.<>?/";

    let mut charset = format!("{}{}", letters, numbers);
    if include_special {
        println!("Adding special characters to the charset.");
        charset.push_str(specials);
    } else {
        println!("Special characters are NOT included in this charset.");
    }

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}