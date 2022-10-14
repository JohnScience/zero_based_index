#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test, doctest, feature = "std")), no_std)]
#![cfg_attr(any(doc, test, doctest, feature = "unchecked_math"), feature(unchecked_math))]
#![cfg_attr(any(doc, test, doctest, feature = "const_inherent_unchecked_arith"), feature(const_inherent_unchecked_arith))]

#[cfg(feature = "const_inherent_unchecked_arith")]
use const_fn::const_fn;
#[cfg(doc)]
use include_display_mode_tex::include_display_mode_tex;

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) offering some utility methods
/// for [zero-based indices](https://en.wikipedia.org/wiki/Zero-based_numbering)
pub struct ZBI<T>(pub T);

impl ZBI<usize> {
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
    #[cfg_attr(doc, doc = include_display_mode_tex!("./tex/to_len.tex"))]
    ///
    /// # Examples
    ///
    /// ## Base case
    /// ```
    /// use zero_based_index::ZBI;
    ///
    /// let zbi = ZBI(2usize);
    /// assert_eq!(zbi.to_len(), Some(3));
    /// ```
    ///
    /// ## Corner case
    ///
    /// ```
    /// use zero_based_index::ZBI;
    ///
    /// let zbi = ZBI(usize::MAX);
    /// assert_eq!(zbi.to_len(), None);
    /// ```
    pub const fn to_len(&self) -> Option<usize> {
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
    #[cfg_attr(doc, doc = include_display_mode_tex!("./tex/to_len.tex"))]
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
    /// use zero_based_index::ZBI;
    ///
    /// let zbi = ZBI(2usize);
    /// // <Proof of the necessary level of rigor that the overflow will never happen>
    /// assert_eq!(unsafe { zbi.to_len_unchecked() }, 3);
    /// ```
    ///
    /// ## Corner case
    ///
    /// The following is [**UB (Undefined Behavior)**](https://en.wikipedia.org/wiki/Undefined_behavior):
    ///
    /// ```no_run
    /// use zero_based_index::ZBI;
    ///
    /// let zbi = ZBI(usize::MAX);
    /// // Undefined Behavior can even summon nasal demons that will take your sinful soul.
    /// assert_eq!(unsafe { zbi.to_len_unchecked() }, 666);
    /// ```    
    ///
    /// *This function is available only if `zero_based_index` is built with the `"unchecked_math"` feature.*
    #[cfg(any(doc, test, doctest, feature = "unchecked_math"))]
    #[cfg_attr(feature = "const_inherent_unchecked_arith", const_fn)]
    pub unsafe fn to_len_unchecked(&self) -> usize {
        self.0.unchecked_add(1)
    }
}

/// Convenience trait for working with zero-based indices, notably of type `usize`
pub trait AsZBI: Sized {
    fn as_zbi(self) -> ZBI<Self>;
}

impl AsZBI for usize {
    fn as_zbi(self) -> ZBI<Self> {
        ZBI(self)
    }
}
