# Krypt

Krypt is a swiss-army knife for encoding, decoding, encryption, decryption and hashing in the terminal.

## Usage

### Hashing

The hash operation provides a simple way for hashing data with many different algorithms.

**Supported hash algorithms:** blake2b, blake2s, md2, md4, md5,
sha1, sha224, sha256, sha384, sha512, keccak224, keccak256,
keccak384, keccak512, sha3_224, sha3_256, sha3_384, sha3_512, streebog256, streebog512

```sh
# hash stdin via sha256
$ echo -n "hello world" | krypt hash sha256
> b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
# hash a file via stdin
$ echo "hello world" > your_file
$ cat your_file | krypt hash md5
> 6f5902ac237024bdd0c176cb93063dc4
# output raw bytes instead of hex number
# e.g. to pipe the output into a hex dumper tool
$ echo "foobar" | krypt hash -r keccak512 | xxd
> 00000000: b11b 8496 0a36 c230 4db6 8e10 8059 a0ca  .....6.0M....Y..
> 00000010: 8c68 8acb 6599 bc39 342a 8227 ef2f e945  .h..e..94*.'./.E
> 00000020: 62a1 f138 1aee e9c7 7772 c1ea 63bb fbb5  b..8....wr..c...
> 00000030: bea1 15a5 7260 a72c 01e0 aaaa 17ce 806a  ....r`.,.......j
```

### Encodig

The encode operation provides a simple way for encoding and decoding data with
different algorithms.

**Supported data formats:** hex, base32, base32hex, base64, base64url

```sh
# encode simple text with base64
# the '-n' removes the trailing newline
$ echo -n "foo" | krypt encode base64
> Zm9v
# decode the result
$ echo -n "Zm9v" | krypt encode -d base64
> foo
# convert an ascii string to hexadecimal number
$ echo "bar" | krypt encode hex
> 6261720a
# convert an image to base64
$ krypt -i your_image.png encode bsae64
```
