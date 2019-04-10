# Filter 454 Reads

Filters 454 reads similarly to [Hurwitz & Sullivan, 2013](https://doi.org/10.1371/journal.pone.0057355).

- Remove sequences with ambiguous bases.
- Remove sequences more than 2 standard deviations from the mean read length.

## Installation

### Precompiled binaries

This program is written in Rust.  If you aren't familiar with compiling Rust programs and you are on Linux or Mac, you can just download the precompiled binaries and use those instead!

### From source

Get the source code

```
git clone https://github.com/mooreryan/filter_454.git
```

cd into the directory

```
cd filter_454
```

Build the executable program with `cargo`.  This command will use 8 CPUs.  Change `--jobs` flag to the number of CPUs you want to use.

```
cargo build --release --jobs 8
```

The binary will be `target/release/filter_454`.  You can now add the binary to somewhere on your path.  I like to put a symlink to the executable file in `~/bin`.  You can do it like this.

```
ln -s $PWD/target/release/filter_454 $HOME/bin/filter_454
```

## Usage

Here is an example command

```
filter_454 seqs.fa > seqs.filtered.fa
```