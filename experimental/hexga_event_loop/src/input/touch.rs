use super::*;

/// Represents a touch event
///
/// Every time the user touches the screen, a new [`TouchPhase::Started`] event with an unique
/// identifier for the finger is generated. When the finger is lifted, an [`TouchPhase::Ended`]
/// event is generated with the same finger id.
///
/// After a `Started` event has been emitted, there may be zero or more `Move`
/// events when the finger is moved or the touch pressure changes.
///
/// The finger id may be reused by the system after an `Ended` event. The user
/// should assume that a new `Started` event received with the same id has nothing
/// to do with the old finger and is a new finger.
///
/// A [`TouchPhase::Cancelled`] event is emitted when the system has canceled tracking this
/// touch, such as when the window loses focus, or on iOS if the user moves the
/// device against their face.
///
/// ## Platform-specific
///
/// - **Web:** Doesn't take into account CSS [`border`], [`padding`], or [`transform`].
/// - **macOS:** Unsupported.
///
/// [`border`]: https://developer.mozilla.org/en-US/docs/Web/CSS/border
/// [`padding`]: https://developer.mozilla.org/en-US/docs/Web/CSS/padding
/// [`transform`]: https://developer.mozilla.org/en-US/docs/Web/CSS/transform
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Touch {
    pub phase: TouchPhase,
    pub pos: Vec2,
    /*
    /// Describes how hard the screen was pressed. May be `None` if the platform
    /// does not support pressure sensitivity.
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **iOS** 9.0+, **Windows** 8+, **Web**, and **Android**.
    /// - **Android**: This will never be [None]. If the device doesn't support pressure
    ///   sensitivity, force will either be 0.0 or 1.0. Also see the
    ///   [android documentation](https://developer.android.com/reference/android/view/MotionEvent#AXIS_PRESSURE).
    pub force: Option<Force>,
    */
    /// Unique identifier of a finger.
    pub id: TouchID,
}

pub type TouchID = u64;