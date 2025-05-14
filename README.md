# Certain encryption decryptor

decryptor written in Rust for certain encryption which uses rijndael algorithm

## Usage

1. Build the project and install dependencies:
```
cargo build --release
```

2. Run the program with the file you want to decrypt:
```
certain_encryption_decryptor <input_file>
```

Example:
```
cargo run --release -- secret.enc
```

The decrypted result will be output as `secret.enc.dec`.
