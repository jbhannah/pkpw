use rand::rng;
use wasm_bindgen::prelude::*;

use crate::generate;

#[wasm_bindgen]
pub fn generate_password(
    len: Option<usize>,
    count: Option<usize>,
    separator: Option<String>,
) -> String {
    let mut rng = rng();

    generate(
        len,
        count.unwrap_or(4),
        &(separator.unwrap_or(" ".to_string())),
        &mut rng,
    )
}
