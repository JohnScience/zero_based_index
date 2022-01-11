//! [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) offering some utility methods
//! for [zero-based indices](https://en.wikipedia.org/wiki/Zero-based_numbering)
//!
//! In order to keep the lengths of method names reasonable, several abbreviations have been used, namely
//! * [**len**](https://www.abbreviations.com/term/92110) for length;
//! * [**intvl**](https://www.abbreviations.com/term/563139) for interval;
//! * [**int**](https://www.abbreviations.com/term/542972) for integer.
//!
//! # Examples
//!
//! ## Base case
//! ```
//! use zero_based_index::ZeroBasedIndex;
//!
//! let zbi = ZeroBasedIndex::<usize>(2);
//! assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), Some(3));
//! ```
//!
//! ## Corner case
//! ```
//! use zero_based_index::ZeroBasedIndex;
//!
//! let zbi = ZeroBasedIndex::<usize>(usize::MAX);
//! assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), None);
//! ```
#![cfg_attr(not(any(doc, test, doctest, feature = "std")), no_std)]
#![cfg_attr(any(doc, test, doctest, feature = "unchecked_math"), feature(unchecked_math))]

use include_display_mode_tex::include_display_mode_tex;

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) offering some utility methods
/// for [zero-based indices](https://en.wikipedia.org/wiki/Zero-based_numbering)
pub struct ZeroBasedIndex<T>(pub T);

impl ZeroBasedIndex<usize> {
    /// If the [cardinality](https://en.wikipedia.org/wiki/Cardinality) of the
    /// [closed](https://en.wikipedia.org/wiki/Interval_(mathematics)#Classification_of_intervals)
    /// [integer interval](https://en.wikipedia.org/wiki/Interval_(mathematics)#Integer_intervals)
    /// from 0 to the stored value fits into [usize][core::primitive::usize], returns that cardinality
    /// as `Some` variant of [Option][core::option::Option]<[usize][core::primitive::usize]>. Otherwise (when
    /// happens [integer overflow](https://en.wikipedia.org/wiki/Integer_overflow)),
    /// the returned value is `None`.
    ///
    /// Using mathematical notation,
    ///
    #[doc = include_display_mode_tex!("./tex/try_get_len_of_closed_int_intvl_from_0.tex")]
    ///
    /// # Examples
    ///
    /// ## Base case
    /// ```
    /// use zero_based_index::ZeroBasedIndex;
    ///
    /// let zbi = ZeroBasedIndex::<usize>(2);
    /// assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), Some(3));
    /// ```
    ///
    /// ## Corner case
    ///
    /// ```
    /// use zero_based_index::ZeroBasedIndex;
    ///
    /// let zbi = ZeroBasedIndex::<usize>(usize::MAX);
    /// assert_eq!(zbi.try_get_len_of_closed_int_intvl_from_0(), None);
    /// ```
    pub const fn try_get_len_of_closed_int_intvl_from_0(&self) -> Option<usize> {
        // [0..r] = {0} ∪ (0..r]
        // |[0..r]| = |{0}| + |(0..r]|
        // |{0}| = 1
        // |(0..r]| = |[1..r]| = r
        // |[0..r]| = r + 1
        self.0.checked_add(1)
    }

    /// Assuming the [cardinality](https://en.wikipedia.org/wiki/Cardinality) of the
    /// [closed](https://en.wikipedia.org/wiki/Interval_(mathematics)#Classification_of_intervals)
    /// [integer interval](https://en.wikipedia.org/wiki/Interval_(mathematics)#Integer_intervals)
    /// from 0 to the stored value fits into [usize][core::primitive::usize], returns that cardinality
    /// as [usize][core::primitive::usize].
    ///
    /// Using mathematical notation,
    ///
    #[doc = include_display_mode_tex!("./tex/try_get_len_of_closed_int_intvl_from_0.tex")]
    ///
    /// # Safety
    ///
    /// This results in undefined behavior when intuitively (or, rather, naively)
    /// `self.0 + 1 > usize::MAX`,
    /// i.e. when [checked_add][core::primitive::usize::checked_add] would return `None`
    /// for `(self.0, 1)`.
    ///
    /// # Examples
    ///
    /// ## Base case
    /// ```
    /// use zero_based_index::ZeroBasedIndex;
    ///
    /// let zbi = ZeroBasedIndex::<usize>(2);
    /// // <Proof of the necessary level of rigor that the overflow will never happen>
    /// assert_eq!(unsafe { zbi.get_len_of_closed_int_intvl_from_0_ignoring_overflow() }, Some(3));
    /// ```
    ///
    /// ## Corner case
    ///
    /// The following is [**UB (Undefined Behavior)**](https://en.wikipedia.org/wiki/Undefined_behavior):
    ///
    /// ```no_run
    /// use zero_based_index::ZeroBasedIndex;
    ///
    /// let zbi = ZeroBasedIndex::<usize>(usize::MAX);
    /// assert_eq!(unsafe { zbi.get_len_of_closed_int_intvl_from_0_ignoring_overflow() }, 214127usize);
    /// ```    
    ///
    /// *This function is available only if `zero_based_index` is built with the `"unchecked_math"` feature.*
    #[cfg(any(doc, test, doctest, feature = "unchecked_math"))]
    pub unsafe fn get_len_of_closed_int_intvl_from_0_ignoring_overflow(&self) -> usize {
        self.0.unchecked_add(1)
    }
}
