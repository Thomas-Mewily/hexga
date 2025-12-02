
pub type BitResult<T=()> = Result<T, BitError>;

/// The things that can go wrong when casting between [`Pod`] data forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitError {
    /// You tried to cast a reference into a reference to a type with a higher
    /// alignment requirement but the input reference wasn't aligned.
    TargetAlignmentGreaterAndInputNotAligned,
    /// If the element size of a slice changes, then the output slice changes
    /// length accordingly. If the output slice wouldn't be a whole number of
    /// elements, then the conversion fails.
    OutputSliceWouldHaveSlop,
    /// When casting an individual `T`, `&T`, or `&mut T` value the
    /// source size and destination size must be an exact match.
    SizeMismatch,
    /// For this type of cast the alignments must be exactly the same and they
    /// were not so now you're sad.
    ///
    /// This error is generated **only** by operations that cast allocated types
    /// (such as `Box` and `Vec`), because in that case the alignment must stay
    /// exact.
    AlignmentMismatch,

    /// When casting to a [`BitPattern`] type, it is possible that the
    /// original data contains an invalid bit pattern. If so, the cast will
    /// fail and this error will be returned. Will never happen on casts
    /// between [`Pod`] types.
    ///
    /// [`Pod`]: crate::Pod
    InvalidBitPattern,
}