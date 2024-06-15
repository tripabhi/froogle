# Froogle

Froogle is a local search engine. It uses [TF-IDF](https://en.wikipedia.org/wiki/Tf%E2%80%93idf) as a metric to rank the search results.



# Rust crate
Froogle is available on [crates.io](https://crates.io/crates/froogle). If you have the [Cargo](https://doc.rust-lang.org/cargo/) toolchain setup, you can install froogle using `cargo install`.
```console
cargo install froogle
```

# Install
Platform-specific binaries for Froogle can be installed from the [Releases](https://github.com/tripabhi/froogle/releases) page.

## MacOS
Froogle is available as Homebrew formula. To install Froogle -
```console
brew tap tripabhi/froogle
brew install froogle
```

## From source (any platform)
Froogle is a [Rust binary](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries), and therefore can be installed using the [Cargo](https://doc.rust-lang.org/cargo/) toolchain.
You can install Cargo by following instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).


Once cargo is installed, you can install Froogle like this.

```console
git clone https://github.com/tripabhi/froogle.git
cd froogle
cargo install --path .

```

# Usage
Use `-h` to see usage instruction.


# References
- Term Frequency - Inverse Document Frequency [Wikipedia](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)
- Snowball English Stemmers [Website](https://snowballstem.org/)
