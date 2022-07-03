use lazy_static::lazy_static;

pub const POKEMON_COUNT: usize = 913;

lazy_static! {
    /// Array of all Pokémon names, in English and ASCII-normalized (e.g.
    /// Nidoran♀ -> Nidoran-F, Flabébé -> Flabebe) for easier keyboard entry.
    pub static ref POKEMON: [&'static str; POKEMON_COUNT] =
        include_str!("./pokemon.txt").trim().split("\n").collect::<Vec<&str>>().try_into().unwrap();
}
