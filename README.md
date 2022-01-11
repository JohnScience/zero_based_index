[Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) offering some utility methods
for [zero-based indices](https://en.wikipedia.org/wiki/Zero-based_numbering)

In order to keep the lengths of method names reasonable, several abbreviations have been used, namely
* [**len**](https://www.abbreviations.com/term/92110) for length;
* [**intvl**](https://www.abbreviations.com/term/563139) for interval;
* [**int**](https://www.abbreviations.com/term/542972) for integer.

# Examples

## Base case
```
use zero_based_index::ZeroBasedIndex;

let zbi = ZeroBasedIndex::<usize>(2);
assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), Some(3));
```

## Base case with `zero_based_index::AsZeroBasedIndex`
```
use zero_based_index::{ZeroBasedIndex, AsZeroBasedIndex};

let zbi = 2.as_zero_based_index();
assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), Some(3));
```

## Corner case
```
use zero_based_index::ZeroBasedIndex;

let zbi = ZeroBasedIndex::<usize>(usize::MAX);
assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), None);
```

# Unchecked math
