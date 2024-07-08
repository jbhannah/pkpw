use std::io::{stdout, IsTerminal};

use arboard::Clipboard;
use clap::Parser;
use pkpw::generate;
use rand::thread_rng;

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
}

/// Generate a password of four random Pokémon names joined by a separator.
fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();
    let password = generate(args.length, args.count, &args.separator, &mut rng);

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
}
