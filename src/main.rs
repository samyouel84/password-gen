use clap::{Parser, ValueEnum};
use rand::{Rng, thread_rng};
use std::error::Error;
use colored::*;
use arboard::Clipboard;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Type of password to generate
    #[arg(short, long, value_enum, default_value_t = PasswordType::Standard)]
    password_type: PasswordType,

    /// Number of passwords to generate
    #[arg(short = 'n', long, default_value_t = 1)]
    count: usize,

    /// Ensure at least one character from each required category
    #[arg(short = 'c', long, default_value_t = true)]
    complex: bool,

    /// Copy the last generated password to clipboard
    #[arg(long, default_value_t = false)]
    copy: bool,

    /// Use command-line mode instead of interactive mode
    #[arg(short = 'C', long, default_value_t = false)]
    cli_mode: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
enum PasswordType {
    /// Standard password (includes lowercase, uppercase, numbers, and special characters)
    Standard,
    /// Only alphabets (lowercase and uppercase)
    AlphabetsOnly,
    /// Only numbers
    NumbersOnly,
    /// Alphanumeric (no special characters)
    Alphanumeric,
}

struct PasswordStrength {
    description: &'static str,
    color: &'static str,
}

fn check_password_strength(password: &str) -> PasswordStrength {
    let length = password.len();
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_number = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    let mut score = 0;
    
    // Length criteria
    score += match length {
        0..=4 => 0,
        5..=7 => 1,
        8..=10 => 2,
        11..=14 => 3,
        _ => 4,
    };

    // Character type criteria
    if has_lowercase { score += 1; }
    if has_uppercase { score += 1; }
    if has_number { score += 1; }
    if has_special { score += 2; }

    match score {
        0..=2 => PasswordStrength { description: "Very Weak", color: "red" },
        3..=4 => PasswordStrength { description: "Weak", color: "yellow" },
        5..=6 => PasswordStrength { description: "Moderate", color: "blue" },
        7..=8 => PasswordStrength { description: "Strong", color: "green" },
        _ => PasswordStrength { description: "Very Strong", color: "bright green" },
    }
}

fn generate_complex_password(length: usize, password_type: PasswordType) -> String {
    let mut rng = thread_rng();
    let mut password = String::with_capacity(length);
    
    // Define character sets
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let special = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    // Ensure at least one character from each required set based on password type
    match password_type {
        PasswordType::Standard => {
            password.push(lowercase.chars().nth(rng.gen_range(0..lowercase.len())).unwrap());
            password.push(uppercase.chars().nth(rng.gen_range(0..uppercase.len())).unwrap());
            password.push(numbers.chars().nth(rng.gen_range(0..numbers.len())).unwrap());
            password.push(special.chars().nth(rng.gen_range(0..special.len())).unwrap());
        }
        PasswordType::AlphabetsOnly => {
            password.push(lowercase.chars().nth(rng.gen_range(0..lowercase.len())).unwrap());
            password.push(uppercase.chars().nth(rng.gen_range(0..uppercase.len())).unwrap());
        }
        PasswordType::NumbersOnly => {
            password.push(numbers.chars().nth(rng.gen_range(0..numbers.len())).unwrap());
        }
        PasswordType::Alphanumeric => {
            password.push(lowercase.chars().nth(rng.gen_range(0..lowercase.len())).unwrap());
            password.push(uppercase.chars().nth(rng.gen_range(0..uppercase.len())).unwrap());
            password.push(numbers.chars().nth(rng.gen_range(0..numbers.len())).unwrap());
        }
    }

    // Fill the rest of the password
    let chars = match password_type {
        PasswordType::Standard => {
            format!("{}{}{}{}", lowercase, uppercase, numbers, special)
        }
        PasswordType::AlphabetsOnly => {
            format!("{}{}", lowercase, uppercase)
        }
        PasswordType::NumbersOnly => numbers.to_string(),
        PasswordType::Alphanumeric => {
            format!("{}{}{}", lowercase, uppercase, numbers)
        }
    };

    while password.len() < length {
        password.push(chars.chars().nth(rng.gen_range(0..chars.len())).unwrap());
    }

    // Shuffle the password
    let mut password: Vec<char> = password.chars().collect();
    for i in (1..password.len()).rev() {
        let j = rng.gen_range(0..=i);
        password.swap(i, j);
    }

    password.iter().collect()
}

fn generate_password(length: usize, password_type: PasswordType, complex: bool) -> String {
    if complex {
        generate_complex_password(length, password_type)
    } else {
        let mut rng = thread_rng();
        let mut password = String::with_capacity(length);

        let chars = match password_type {
            PasswordType::Standard => {
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?"
            }
            PasswordType::AlphabetsOnly => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            PasswordType::NumbersOnly => "0123456789",
            PasswordType::Alphanumeric => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        };

        for _ in 0..length {
            password.push(chars.chars().nth(rng.gen_range(0..chars.len())).unwrap());
        }

        password
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text.to_owned())?;
    
    // Verify the clipboard content
    match clipboard.get_text() {
        Ok(contents) if contents == text => Ok(()),
        Ok(_) => Err("Failed to verify clipboard contents".into()),
        Err(e) => Err(format!("Failed to verify clipboard: {}", e).into()),
    }
}

fn interactive_mode() -> Result<(usize, PasswordType, usize, bool, bool), Box<dyn Error>> {
    let theme = ColorfulTheme::default();

    // Password type selection
    let password_types = vec!["Standard", "Alphabets Only", "Numbers Only", "Alphanumeric"];
    let password_type_selection = Select::with_theme(&theme)
        .with_prompt("Select password type")
        .default(0)
        .items(&password_types)
        .interact()?;

    let password_type = match password_type_selection {
        0 => PasswordType::Standard,
        1 => PasswordType::AlphabetsOnly,
        2 => PasswordType::NumbersOnly,
        3 => PasswordType::Alphanumeric,
        _ => PasswordType::Standard,
    };

    // Password length
    let length: usize = Input::with_theme(&theme)
        .with_prompt("Enter password length")
        .default(12)
        .validate_with(|input: &usize| {
            if *input >= 4 {
                Ok(())
            } else {
                Err("Length must be at least 4 characters")
            }
        })
        .interact()?;

    // Number of passwords
    let count: usize = Input::with_theme(&theme)
        .with_prompt("How many passwords to generate?")
        .default(1)
        .validate_with(|input: &usize| {
            if *input >= 1 {
                Ok(())
            } else {
                Err("Must generate at least 1 password")
            }
        })
        .interact()?;

    // Complexity
    let complex = Confirm::with_theme(&theme)
        .with_prompt("Enable password complexity requirements?")
        .default(true)
        .interact()?;

    // Clipboard
    let copy = Confirm::with_theme(&theme)
        .with_prompt("Copy the last generated password to clipboard?")
        .default(false)
        .interact()?;

    Ok((length, password_type, count, complex, copy))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    
    if !args.cli_mode {
        let (length, password_type, count, complex, copy) = interactive_mode()?;
        args.length = length;
        args.password_type = password_type;
        args.count = count;
        args.complex = complex;
        args.copy = copy;
    }
    
    println!("\nPassword Generation Settings:");
    println!("Type: {:?}", args.password_type);
    println!("Length: {}", args.length);
    println!("Count: {}", args.count);
    println!("Complex: {}", args.complex);
    println!("Copy to clipboard: {}", args.copy);
    println!("----------------------------------------");
    
    let mut last_password = String::new();
    
    for i in 0..args.count {
        let password = generate_password(args.length, args.password_type, args.complex);
        let strength = check_password_strength(&password);
        
        println!("Password {}: {}", i + 1, password);
        println!("Strength: {}", strength.description.color(strength.color));
        println!("----------------------------------------");
        
        last_password = password;
    }
    
    if args.copy {
        match copy_to_clipboard(&last_password) {
            Ok(_) => {
                println!("{}", "✓ Last password copied to clipboard!".green());
                println!("Password in clipboard: {}", last_password);
            }
            Err(e) => {
                eprintln!("{}: {}", "Failed to copy to clipboard".red(), e);
                eprintln!("You can manually copy this password: {}", last_password);
            }
        }
    }
    
    Ok(())
}
