# pkpw

[![Crates.io](https://img.shields.io/crates/v/pkpw)](https://crates.io/crates/pkpw)
[![.github/workflows/cargo.yml](https://github.com/jbhannah/pkpw/actions/workflows/cargo.yml/badge.svg)](https://github.com/jbhannah/pkpw/actions/workflows/cargo.yml)

What if [`correct horse battery staple`][xkcd], but Pokémon.

## Installation

```sh
cargo install pkpw
```

## Usage

### CLI

```console
$ pkpw -h
pkpw 1.2.4
Jesse Brooklyn Hannah <jesse@jbhannah.net>
What if correct horse battery staple, but Pokémon.

USAGE:
    pkpw [OPTIONS]

OPTIONS:
    -c, --copy                     Copy the generated value to the clipboard instead of displaying
                                   it
    -h, --help                     Print help information
    -l, --length <LENGTH>          Minimum length of the generated password
    -n, --count <COUNT>            Number of Pokémon names to use in the generated password
                                   [default: 4]
    -s, --separator <SEPARATOR>    Separator between Pokémon names in the generated password; either
                                   a single character, "digit" for random digits, or "special" for
                                   random special characters [default: " "]
    -V, --version                  Print version information
```

### Library

```rust
use pkpw::generate;
use rand::thread_rng;

let mut rng = thread_rng();
let password = generate(None, 4, " ", &mut rng);
```

## But is it secure?

**Disclaimer:** These are just estimates, I have a physics degree but I'm not
a combinatorics or cryptography expert.

[Password entropy][] is calculated using the pool size $R$ and password length
$L$ used to generate a password:

$$
E = log_2(R^L)
$$

where a brute-force attack will need an average of $2^{E-1}$ guesses to
crack a password with $E$ bits of entropy.

By default, `pkpw` chooses **4** Pokémon names from the pool of [**1025** known
Pokémon][names], resulting in an entropy of

$$
E = log_2(1025^4) \approx 40.01
$$

bits. A dictionary attack that knows to use the 1025 known Pokémon names as the
pool of values would take $1.104 \times 10^{12}$ guesses on average to correctly
guess a password, or about **34 years, 11 months, and 21 days** at 1000 guesses
per second.

At an average length of about 7.67 characters per Pokémon name, passwords
generated using the default settings have an average length of about **34**
characters (4 Pokémon names, plus one space separating each name for a total of
3 spaces). A brute-force attack that uses a pool of **95** standard US keyboard
characters (alphanumeric, special characters, and space) would be working
against

$$
E = log_2(95^{34}) \approx 222.375
$$

bits of entropy, taking an average of $1.748 \times 10^{67}$ guesses, or
**$5.540 \times 10^{56}$ years**, to correctly guess a password.

## Copyright

All Pokémon names are ™ and © The Pokémon Company, Inc. Everything else in
this project is © Jesse Brooklyn Hannah and released under the terms of the
[MIT License][].

[xkcd]: https://xkcd.com/936/
[password entropy]: https://www.omnicalculator.com/other/password-entropy
[names]: https://github.com/jbhannah/pkpw/blob/trunk/src/pokemon/pokemon.txt
[mit license]: https://github.com/jbhannah/pkpw/blob/trunk/LICENSE
