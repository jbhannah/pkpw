use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, thread_rng, Rng};

lazy_static! {
    /// All Pokémon names.
    static ref POKEMON: Vec<&'static str> =
        include_str!("../pokemon.txt").trim().split("\n").collect();
}

/// Print a password of four random Pokémon names joined by a space character.
fn main() {
    let password = pick(&mut thread_rng()).join(" ");

    println!("{}", password);
}

/// Pick four random Pokémon names using the given RNG.
fn pick<R: Rng + ?Sized>(rng: &mut R) -> Vec<&str> {
    let mut pokemon = POKEMON.clone();
    pokemon.shuffle(rng);
    pokemon.into_iter().take(4).collect()
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;

    use super::*;

    /// Ensure that pick() returns a vector of four strings.
    #[test]
    fn test_pick() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(913);
        let pokemon = pick(&mut rng);

        assert_eq!(
            vec!["Shroomish", "Venusaur", "Froakie", "Tyranitar"],
            pokemon
        );
    }

    /// Ensure that pick() returns different results for different RNGs.
    #[test]
    fn test_pick_rand() {
        let mut rng_1 = rand::rngs::StdRng::seed_from_u64(913);
        let pokemon_1 = pick(&mut rng_1);

        let mut rng_2 = rand::rngs::StdRng::seed_from_u64(319);
        let pokemon_2 = pick(&mut rng_2);

        assert_ne!(pokemon_1, pokemon_2);
    }

    /// Ensure that all Pokémon names are loaded.
    #[test]
    fn test_pokemon() {
        assert_eq!(POKEMON.len(), 913);
    }
}
