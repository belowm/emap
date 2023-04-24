[![cargo](https://github.com/yegor256/emap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/emap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/emap.svg)](https://crates.io/crates/emap)
[![codecov](https://codecov.io/gh/yegor256/emap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/emap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/emap)](https://hitsofcode.com/view/github/yegor256/emap)
![Lines of code](https://img.shields.io/tokei/lines/github/yegor256/emap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/emap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/emap)](https://docs.rs/emap/latest/emap/)

It is an alternative on-heap implementation of a Rust map with keys of type `usize`
and a limited capacity.
The map works much faster, see the [benchmarking results](#benchmark) below.
It is faster because it allocates space for all keys at once and then the cost
of `get()` is _O(1)_. Obviously, with this design, the cost of `iter()` increases because the iterator
has to jump through empty keys. However, there
is a supplementary function `next_key()`, which returns the next available key in the map. 
It is recommended to use it in order to ensure sequential order of the keys, which
will guarantee _O(1)_ cost of `next()` in iterators.

First, add this to `Cargo.toml`:

```toml
[dependencies]
emap = "0.0.2"
```

Then, use it like a standard hash map... well, almost:

```rust
use emap::Map;
let mut m : Map<&str> = Map::with_capacity(100); // allocation on heap
m.insert(m.next_key(), "foo");
m.insert(m.next_key(), "bar");
assert_eq!(2, m.len());
```

If more than 100 keys will be added to the map, it will panic. 
The map doesn't increase its size automatically.

Read [the API documentation](https://docs.rs/emap/latest/emap/). 
The struct
[`emap::Map`](https://docs.rs/emap/latest/emap/struct.Map.html) is designed as closely similar to 
[`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `emap::Map` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions 
([`benchmark.rs`](https://github.com/yegor256/emap/blob/master/tests/benchmark.rs)) 
to them and measured how fast they performed. In the following table, 
the numbers over 1.0 indicate performance gain, 
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 1 | 10 | 100 | 1000 | 10000 |
| --- | --: | --: | --: | --: | --: |
| `emap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `hashbrown::HashMap` | 32.00 | 14.05 | 9.00 | 8.59 | 8.82 |
| `indexmap::IndexMap` | 36.00 | 27.91 | 22.64 | 21.61 | 22.01 |
| `linear_map::LinearMap` | 7.00 | 4.73 | 29.85 | 241.30 | 2K |
| `linked_hash_map::LinkedHashMap` | 59.00 | 34.77 | 30.12 | 28.29 | 28.40 |
| `litemap::LiteMap` | 12.00 | 7.55 | 12.16 | 34.10 | 472.24 |
| `nohash_hasher::BuildNoHashHasher` | 23.00 | 16.73 | 8.55 | 7.83 | 7.66 |
| `rustc_hash::FxHashMap` | 25.00 | 13.09 | 8.64 | 8.07 | 8.12 |
| `std::collections::BTreeMap` | 44.00 | 24.77 | 23.37 | 44.26 | 49.86 |
| `std::collections::HashMap` | 43.00 | 25.23 | 21.80 | 20.63 | 21.03 |
| `tinymap::array_map::ArrayMap` | 3.00 | 16.23 | 116.53 | 1K | 9K |

The experiment was performed on 23-04-2023.
 There were 100 repetition cycles.
 The entire benchmark took 108s.

<!-- benchmark -->

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
$ cargo test -vv
```

If everything goes well, fork repository, make changes, 
send us a [pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also, 
run `cargo fmt` and `cargo clippy`.
