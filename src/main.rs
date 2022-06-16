use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, thread_rng, Rng};

fn main() {
    let password = pick(&mut thread_rng()).join(" ");

    println!("{}", password);
}

fn pick<R: Rng + ?Sized>(rng: &mut R) -> Vec<&str> {
    let mut pokemon = POKEMON.clone();
    pokemon.shuffle(rng);
    pokemon.into_iter().take(4).collect()
}

lazy_static! {
    pub static ref POKEMON: Vec<&'static str> =
        include_str!("../pokemon.txt").trim().split("\n").collect();
}
