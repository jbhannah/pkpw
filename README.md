# pkpw

What if [`correct horse battery staple`][xkcd], but Pokémon.

## Installation

```sh
cargo install pkpw
```

## Usage

```sh
pkpw
```

## But is it secure?

**Disclaimer:** These are just estimates, I have a physics degree but I'm not
a combinatorics or cryptography expert.

[Password entropy][] is calculated using the pool size $R$ and password length
$L$ used to generate a password:

$$
E = log_2(R^L)
$$

where a brute-force attack will need an average of $ 2^{E-1} $ guesses to
crack a password with $E$ bits of entropy.

By default, `pkpw` chooses **4** Pokémon names from the pool of [**913** known
Pokémon][names], resulting in an entropy of

$$
E = log_2(913^4) \approx 39.338
$$

bits. A brute-force attack that knows to use the 913 known Pokémon names as
the pool of values would take $ 3.474 \times 10^{11} $ guesses on average to
correctly guess a password, or a bit more than **11 years** at 1000 guesses
per second.

At an average length of about 7.5 characters per Pokémon name, passwords
generated using the default settings have an average length of about **33**
characters (4 Pokémon names, plus one space separating each name for a total
of 3 spaces). A brute-force attack that uses a pool of **95** standard US
keyboard characters (alphanumeric, special characters, and space) would be
working against

$$
E = log_2(95^{33}) \approx 216.805
$$

bits of entropy, taking an average of $ 9.201 \times 10^{64} $ guesses, or
**$ 2.916 \times 10^{54} $ years**, to correctly guess a password.

## Copyright

All Pokémon names are ™ and © The Pokémon Company, Inc. Everything else in
this project is © Jesse Brooklyn Hannah and released under the terms of the
[MIT License](LICENSE).

[xkcd]: https://xkcd.com/936/
[password entropy]: https://www.omnicalculator.com/other/password-entropy
[names]: pokemon.txt
