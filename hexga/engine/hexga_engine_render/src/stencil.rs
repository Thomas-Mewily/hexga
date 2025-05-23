use super::*;



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilState {
    pub front: StencilFaceState,
    pub back: StencilFaceState,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StencilFaceState {
    /// Operation to use when stencil test fails
    pub fail_op: StencilOp,

    /// Operation to use when stencil test passes, but depth test fails
    pub depth_fail_op: StencilOp,

    /// Operation to use when both stencil and depth test pass,
    /// or when stencil pass and no depth or depth disabled
    pub pass_op: StencilOp,

    /// Used for stencil testing with test_ref and test_mask: if (test_ref & test_mask) *test_func* (*stencil* && test_mask)
    /// Default is Always, which means "always pass"
    pub test_func: StencilCompareFunc,

    /// Default value: 0
    pub test_ref: i32,

    /// Default value: all 1s
    pub test_mask: u32,

    /// Specifies a bit mask to enable or disable writing of individual bits in the stencil planes
    /// Default value: all 1s
    pub write_mask: u32,
}

/// Operations performed on current stencil value when comparison test passes or fails.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StencilOp {
    /// Default value
    Keep,
    Zero,
    Replace,
    IncrementClamp,
    DecrementClamp,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

/// Depth and stencil compare function
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StencilCompareFunc {
    /// Default value
    Always,
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
}

