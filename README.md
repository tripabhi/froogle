# Froogle

Froogle is a local search engine.

# Install
Froogle is only available in Homebrew, however, you can build it from source on any platform.

> NOTE: Froogle uses [poppler-rs](https://github.com/DMSrs/poppler-rs) for parsing PDF, which in turn depends on [libpoppler](https://poppler.freedesktop.org/) that **needs** to be present on your system to run Froogle.
## From source (any platform)
Froogle is a [Rust binary](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries), and therefore can be installed using the [Cargo](https://doc.rust-lang.org/cargo/) toolchain.
You can install Cargo by following instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Download libpoppler from [here](https://poppler.freedesktop.org/#:~:text=irc.libera.chat.-,Download,-The%20latest%20stable)

Once cargo and poppler are installed, you can install Froogle like this.

```console
git clone https://github.com/tripabhi/froogle.git
cd froogle
cargo install --path .

```


## MacOS
Because Froogle was requested to have PDF support, and the only available library with decent performance and a relatively simple API was [libpoppler](https://poppler.freedesktop.org/), you'll have to download poppler as a package. It is a large library and takes some time to download, and I apologize for having to depend on this library. I will fix this in future releases.

To install poppler on MacOS:
```console
brew install poppler
```


Froogle is available as Homebrew formula. To install Froogle -
```console
brew tap tripabhi/froogle
brew install froogle
```

# Usage
Use `-h` to see usage instruction.

