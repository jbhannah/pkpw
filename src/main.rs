use clap::Parser;
use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, thread_rng, Rng};

lazy_static! {
    /// All Pokémon names.
    static ref POKEMON: Vec<&'static str> =
        include_str!("../pokemon.txt").trim().split("\n").collect();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of Pokémon names to use in the generated password.
    #[clap(short = 'n', value_parser, default_value_t = 4)]
    count: usize,
}

/// Print a password of four random Pokémon names joined by a space character.
fn main() {
    let args = Args::parse();

    let password = pick(args.count, &mut thread_rng()).join(" ");

    println!("{}", password);
}

/// Pick a number of random Pokémon names using the given RNG.
fn pick<R: Rng + ?Sized>(count: usize, rng: &mut R) -> Vec<&str> {
    let mut pokemon = POKEMON.clone();
    pokemon.shuffle(rng);
    pokemon.into_iter().take(count).collect()
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;

    use super::*;

    /// Ensure that pick(4, …) returns a vector of four strings.
    #[test]
    fn test_pick() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(913);
        let pokemon = pick(4, &mut rng);

        assert_eq!(
            vec!["Shroomish", "Venusaur", "Froakie", "Tyranitar"],
            pokemon
        );
    }

    /// Ensure that pick() returns different results for different RNGs.
    #[test]
    fn test_pick_rand() {
        let mut rng_1 = rand::rngs::StdRng::seed_from_u64(913);
        let pokemon_1 = pick(4, &mut rng_1);

        let mut rng_2 = rand::rngs::StdRng::seed_from_u64(319);
        let pokemon_2 = pick(4, &mut rng_2);

        assert_ne!(pokemon_1, pokemon_2);
    }

    /// Ensure that all Pokémon names are loaded.
    #[test]
    fn test_pokemon() {
        assert_eq!(POKEMON.len(), 913);
    }
}
