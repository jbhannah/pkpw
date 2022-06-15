use rand::{prelude::SliceRandom, thread_rng};

fn main() {
    let mut pokemon = include_str!("../pokemon.txt")
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    pokemon.shuffle(&mut thread_rng());

    let password = pokemon.into_iter().take(4).collect::<Vec<&str>>().join(" ");

    println!("{}", password);
}
