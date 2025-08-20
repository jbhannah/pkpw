#![cfg(not(target_arch = "wasm32"))]

use std::io::{stdout, IsTerminal};

use arboard::Clipboard;
use clap::Parser;
use pkpw::generate;
use rand::rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Copy the generated value to the clipboard instead of displaying it.
    #[clap(short = 'c', long = "copy")]
    copy: bool,

    /// Number of Pokémon names to use in the generated password.
    #[clap(
        short = 'n',
        long = "count",
        value_parser,
        default_value_t = 4,
        conflicts_with = "length"
    )]
    count: usize,

    /// Minimum length of the generated password.
    #[clap(short = 'l', long = "length", value_parser, required = false)]
    length: Option<usize>,

    /// Separator between Pokémon names in the generated password; either a
    /// single character, "digit" for random digits, or "special" for random
    /// special characters.
    #[clap(short = 's', long = "separator", value_parser, default_value = " ")]
    separator: String,

    /// Number of random digits to append to the end of the password.
    /// If provided without a value, defaults to 4.
    #[clap(short = 'a', long = "append-numbers", value_parser, num_args(0..=1), default_missing_value = "4")]
    append_numbers: Option<usize>,
}

/// Generate a password of four random Pokémon names joined by a separator.
fn main() {
    let args = Args::parse();
    let mut rng = rng();
    let append_numbers = args.append_numbers.unwrap_or(0);
    let password = generate(args.length, args.count, &args.separator, append_numbers, &mut rng);

    if args.copy {
        Clipboard::new()
            .expect("could not access OS clipboard")
            .set_text(password)
            .expect("could not set OS clipboard");
    } else {
        print!("{}", password);

        if stdout().is_terminal() {
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;

    fn cmd() -> Command {
        assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    /// Ensure that the command runs successfully.
    #[test]
    fn test_command() {
        cmd().assert().success();
    }

    /// Ensure that the count and length arguments are mutually exclusive.
    #[test]
    fn test_command_length_excl_pick() {
        let mut cmd = cmd();
        cmd.arg("-l 40").arg("-n 4");
        cmd.assert().failure();
    }

    /// Ensure that the append-numbers option works with explicit value.
    #[test]
    fn test_command_append_numbers() {
        let mut cmd = cmd();
        cmd.arg("--append-numbers").arg("3");
        let output = cmd.assert().success();
        let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
        
        // Check that the last 3 characters are digits
        let chars: Vec<char> = stdout.trim().chars().collect();
        let last_three = &chars[chars.len() - 3..];
        
        for &ch in last_three {
            assert!(ch.is_ascii_digit());
        }
    }

    /// Ensure that the append-numbers option works without explicit value (defaults to 4).
    #[test]
    fn test_command_append_numbers_default() {
        let mut cmd = cmd();
        cmd.arg("-a");
        let output = cmd.assert().success();
        let stdout = std::str::from_utf8(&output.get_output().stdout).unwrap();
        
        // Check that the last 4 characters are digits
        let chars: Vec<char> = stdout.trim().chars().collect();
        let last_four = &chars[chars.len() - 4..];
        
        for &ch in last_four {
            assert!(ch.is_ascii_digit());
        }
    }
}
