# Krypt

Krypt is a swiss-army knife for encoding, decoding, and hashing in the terminal.

## Installation

### Using cargo

```
cargo install krypt
```

### Building from source

**Requirements:**
  - Rust (1.41 stable)
  - cargo

```
git clone git@github.com:Stupremee/krypt
cd krypt
# Just build the binary
cargo build --release
# Install krypt
cargo install --path .
```

## Usage

### Hashing

```sh
# hash stdin input and output hex representation
$ echo -n "hello world" | krypt -f hex hash sha256
> b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
# output raw bytes to
# -f raw is optional because the default format is raw
$ echo -n "hello world" | krypt -f raw hash sha256 | xxd
> 00000000: b94d 27b9 934d 3e08 a52e 52d7 da7d abfa  .M'..M>...R..}..
> 00000010: c484 efe3 7a53 80ee 9088 f7ac e2ef cde9  ....zS..........
# output in hexdump format using hexyl
$ echo -n "hello world" | krypt -f hexdump hash sha256
> ┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
> │00000000│ b9 4d 27 b9 93 4d 3e 08 ┊ a5 2e 52 d7 da 7d ab fa │×M'××M>•┊×.R××}××│
> │00000010│ c4 84 ef e3 7a 53 80 ee ┊ 90 88 f7 ac e2 ef cd e9 │××××zS××┊××××××××│
> └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
# use file as input
$ echo -n "hello world" > your_file
$ krypt -i your_file -f hex hash sha256
> b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
# hash raw bytes
$ echo -n "\xff" | krypt -f hex hash sha256
> a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89
# use hex encoded data as input
$ echo "0xff" | krypt --hex-input -f hex hash sha256
> a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89
# works also with files
$ echo "0xff" > your_file
$ krypt --hex-input -i your_file -f hex hash sha256
> a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89
```

### Encoding

```sh
# encode string to bsae64
$ echo "hello!" | krypt encode base64
> aGVsbG8hCg==
# decode data
$ echo "aGVsbG8hCg==" | krypt encode -d base64
> hello!
# encode raw bytes (the 0A is the newline character)
$ echo "\xaa\xbb\xcc" > foo
$ krypt -i foo encode hex
> AABBCC0A
```

## License

This project is licensed under the [GPLv3](https://github.com/Stupremee/krypt/blob/master/LICENSE) license.
