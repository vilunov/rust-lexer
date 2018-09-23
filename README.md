# Rust Lexer

This is the delivery for the second homework assignment of Compilers Construction of Innopolis University, Fall 2018

## Usage

- Place the source of your program in file `in.txt`
- Run the tokenizer using by one of instructions below
- Find the list of tokens in file `out.txt`

##  Running in Docker

Requirements:
- Docker >= 18.05
- Docker Compose >= 1.22

**Building:**
```sh
docker-compose build
```

**Running:**
```sh
docker-compose run --rm rust-lexer
```


## Running with Cargo

Requirements:
- Cargo >= 1.28
- rustc >= 1.28


**Building and running:**
```sh
cargo run
```

**Running tests:**
```sh
cargo test
```

## Documentation

Requirements:
- Cargo >= 1.28
- rustdoc >= 1.29

**Building:**
```sh
cargo doc
```

The documentation index will be located at `target/doc/rust_lexer/index.html`
