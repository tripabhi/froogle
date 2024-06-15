# Froogle

Froogle is a local search engine.

# Install
Froogle is only available in Homebrew, however, you can build it from source on any platform.

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

