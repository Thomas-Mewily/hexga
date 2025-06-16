use super::*;


/// Pixel arithmetic description for blending operations.
/// Will be used in an equation:
/// `equation(sfactor * source_color, dfactor * destination_color)`
/// Where source_color is the new pixel color and destination color is color from the destination buffer.
///
/// Example:
///```
///# use hexga_engine_graphics::blend::{BlendState, BlendFactor, BlendValue, BlendEquation};
///BlendState::new(
///    BlendEquation::Add,
///    BlendFactor::Value(BlendValue::SourceAlpha),
///    BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
///);
///```
/// This will be `source_color * source_color.a + destination_color * (1 - source_color.a)`
/// Wich is quite common set up for alpha blending.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendState {
    pub equation: BlendEquation,
    pub sfactor: BlendFactor,
    pub dfactor: BlendFactor,
}

impl BlendState{
    pub fn new(
        equation: BlendEquation,
        sfactor: BlendFactor,
        dfactor: BlendFactor,
    ) -> Self {
        Self {
            equation,
            sfactor,
            dfactor,
        }
    }
}

/// Specifies how incoming RGBA values (source) and the RGBA in framebuffer (destination)
/// are combined.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendEquation {
    /// Adds source and destination. Source and destination are multiplied
    /// by blending parameters before addition.
    Add,
    /// Subtracts destination from source. Source and destination are
    /// multiplied by blending parameters before subtraction.
    Subtract,
    /// Subtracts source from destination. Source and destination are
    /// multiplied by blending parameters before subtraction.
    ReverseSubtract,
}

/// Blend factors.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendFactor {
    Zero,
    One,
    Value(BlendValue),
    OneMinusValue(BlendValue),
    SourceAlphaSaturate,
}

/// Blend values.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendValue
{
    SourceColor,
    SourceAlpha,
    DestinationColor,
    DestinationAlpha,
}