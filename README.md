
# Random 5 words

You're at a loss for words for Wordle? I'm in the same boat.

I'm a newcomer to Rust and Wordle addict. This small project's goal is to discover Rust's features, such as the file system, command line arguments, randomisation and parallelisation.

## Prerequisite

This binary uses words list from '/usr/share/dict/words'. Make sure this file is available on build system.
```bash
/usr/bin/file '/usr/share/dict/words'
```

## Usage

```bash
$ rand-5words
casks
cedar
robin
spire
stead

$ rand-5words 7
teasy
rabic
chirp
ploat
mahar
carve
sloke
```

Use `cargo run` or `cargo run -- <number_of_output>` for quick result

To execute release build
```bash
cargo build --release

./target/release/rand-5words
./target/release/rand-5words <number_of_output>
```
