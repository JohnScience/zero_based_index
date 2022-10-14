[![crates.io](https://img.shields.io/crates/v/zero_based_index.svg)][`zero_based_index`]
[![crates.io](https://img.shields.io/crates/d/zero_based_index.svg)][`zero_based_index`]
[![crates.io](https://img.shields.io/github/workflow/status/JohnScience/zero_based_index/Stable)][`zero_based_index`]

[Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) offering some utility methods
for [zero-based indices](https://en.wikipedia.org/wiki/Zero-based_numbering)

In order to keep the lengths of method names reasonable, several abbreviations have been used, namely
* [**len**](https://www.abbreviations.com/term/92110) for length;
* [**int**](https://www.abbreviations.com/term/542972) for integer.
* [**zbi**](https://www.abbreviations.com/term/2520505) for zero-based index.

# Examples

## Base case
```
use zero_based_index::ZBI;

let zbi = ZBI(2usize);
assert_eq!(zbi.to_len(), Some(3));
```

## Base case with `zero_based_index::AsZBI`
```
use zero_based_index::{ZBI, AsZBI};

let zbi = 2.as_zbi();
assert_eq!(zbi.to_len(), Some(3));
```

## Corner case
```
use zero_based_index::ZBI;

let zbi = ZBI(usize::MAX);
assert_eq!(zbi.to_len(), None);
```

# Features

* Check the list of feature flags [here](https://docs.rs/crate/zero_based_index/latest/features).
* Learn about features in general [here](https://dev.to/rimutaka/cargo-features-explained-with-examples-194g).

[`zero_based_index`]: https://crates.io/crates/zero_based_index

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
