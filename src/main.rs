use std::vec::IntoIter;

use arboard::Clipboard;
use clap::Parser;
use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, thread_rng, Rng};

const DIGITS: &'static [&'static str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const SPECIAL: &'static [&'static str] = &[
    "~", "`", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "-", "+", "=", "{", "}", "[",
    "]", "|", ":", ";", "<", ",", ">", ".", "?", "/",
];

lazy_static! {
    /// All Pokémon names.
    static ref POKEMON: Vec<&'static str> =
        include_str!("../pokemon.txt").trim().split("\n").collect();
}

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
    let mut pokemon = shuffle(&mut rng);

    let picked = match args.length {
        Some(len) => length(&mut pokemon, len),
        None => pick(&mut pokemon, args.count),
    };

    let mut rng = thread_rng();
    let password = match args.separator.as_str() {
        "digit" => join(picked, DIGITS, &mut rng),
        "special" => join(picked, SPECIAL, &mut rng),
        sep => picked.join(sep),
    };

    if args.copy {
        Clipboard::new()
            .expect("could not access OS clipboard")
            .set_text(password)
            .expect("could not set OS clipboard");
    } else {
        print!("{}", password);

        if atty::is(atty::Stream::Stdout) {
            println!();
        }
    }
}

/// Shuffle the list of Pokémon using the given RNG and return them as an
/// iterator.
fn shuffle<R: Rng + ?Sized>(rng: &mut R) -> IntoIter<&str> {
    let mut pokemon = POKEMON.clone();
    pokemon.shuffle(rng);
    pokemon.into_iter()
}

/// Pick a number of random Pokémon names from the given iterator.
fn pick<'a>(pokemon: &mut IntoIter<&'a str>, count: usize) -> Vec<&'a str> {
    pokemon.take(count).collect()
}

/// Take Pokémon names from the given iterator until the resulting password
/// would meet the required minimum length.
fn length<'a>(pokemon: &mut IntoIter<&'a str>, length: usize) -> Vec<&'a str> {
    let mut picked: Vec<&str> = vec![];

    while picked
        .clone()
        .into_iter()
        .fold(0, |len, name| len + name.len() + 1)
        < length + 1
    {
        picked.push(pokemon.next().expect("no unique names left"));
    }

    picked
}

/// Join the collection of items with random selections from the set of possible
/// separators.
fn join<R: Rng + ?Sized>(picked: Vec<&str>, separators: &[&str], rng: &mut R) -> String {
    picked
        .into_iter()
        .map(|name| name.to_owned())
        .reduce(|password, next| {
            let i = rng.gen::<usize>() % separators.len();
            format!("{}{}{}", password, separators[i], next)
        })
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;

    use super::*;

    /// Ensure that the command runs successfully.
    #[test]
    fn test_command() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.assert().success();
    }

    /// Ensure that the count and length arguments are mutually exclusive.
    #[test]
    fn test_command_length_excl_pick() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("-l 40").arg("-n 4");
        cmd.assert().failure();
    }

    /// Ensure that join() joins the vector of strings with random elements from
    /// the slice of separators.
    #[test]
    fn test_join() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(913);
        let mut rng_1 = rng.clone();

        let mut pokemon = shuffle(&mut rng);
        let picked = pick(&mut pokemon, 4);

        assert_eq!(
            "Shroomish7Venusaur0Froakie2Tyranitar",
            join(picked, DIGITS, &mut rng_1)
        );
    }

    /// Ensure that length(…, 40) returns a vector of five strings which create
    /// a password with a length greater than 40.
    #[test]
    fn test_length() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(913);
        let mut pokemon = shuffle(&mut rng);

        assert_eq!(
            vec!["Shroomish", "Venusaur", "Froakie", "Tyranitar", "Wingull"],
            length(&mut pokemon, 40)
        );
    }

    /// Ensure that pick(…, 4) returns a vector of four strings.
    #[test]
    fn test_pick() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(913);
        let mut pokemon = shuffle(&mut rng);

        assert_eq!(
            vec!["Shroomish", "Venusaur", "Froakie", "Tyranitar"],
            pick(&mut pokemon, 4)
        );
    }

    /// Ensure that pick() returns different results for different RNGs.
    #[test]
    fn test_pick_rand() {
        let mut rng_1 = rand::rngs::StdRng::seed_from_u64(913);
        let mut pokemon_1 = shuffle(&mut rng_1);

        let mut rng_2 = rand::rngs::StdRng::seed_from_u64(319);
        let mut pokemon_2 = shuffle(&mut rng_2);

        assert_ne!(pick(&mut pokemon_1, 4), pick(&mut pokemon_2, 4));
    }

    /// Ensure that all Pokémon names are loaded.
    #[test]
    fn test_pokemon() {
        assert_eq!(POKEMON.len(), 913);
    }
}
