use super::*;

pub(crate) type WinitKeyNative = winit::keyboard::NativeKey;
pub(crate) type WinitKeyName = winit::keyboard::NamedKey;
pub(crate) type WinitKey<Str> = winit::keyboard::Key<Str>;

// This file contains a substantial portion of the UI Events Specification by the W3C. In
// particular, the variant names within `Key` and `KeyCode` and their documentation are modified
// versions of contents of the aforementioned specification.
//
// The original documents are:
//
// ### For `Key`
// UI Events KeyboardEvent key Values
// https://www.w3.org/TR/2017/CR-uievents-key-20170601/
// Copyright © 2017 W3C® (MIT, ERCIM, Keio, Beihang).
//
// ### For `KeyCode`
// UI Events KeyboardEvent code Values
// https://www.w3.org/TR/2017/CR-uievents-code-20170601/
// Copyright © 2017 W3C® (MIT, ERCIM, Keio, Beihang).
//
// These documents were used under the terms of the following license. This W3C license as well as
// the W3C short notice apply to the `Key` and `KeyCode` enums and their variants and the
// documentation attached to their variants.

// --------- BEGINNING OF W3C LICENSE --------------------------------------------------------------
//
// License
//
// By obtaining and/or copying this work, you (the licensee) agree that you have read, understood,
// and will comply with the following terms and conditions.
//
// Permission to copy, modify, and distribute this work, with or without modification, for any
// purpose and without fee or royalty is hereby granted, provided that you include the following on
// ALL copies of the work or portions thereof, including modifications:
//
// - The full text of this NOTICE in a location viewable to users of the redistributed or derivative
//   work.
// - Any pre-existing intellectual property disclaimers, notices, or terms and conditions. If none
//   exist, the W3C Software and Document Short Notice should be included.
// - Notice of any changes or modifications, through a copyright statement on the new code or
//   document such as "This software or document includes material copied from or derived from
//   [title and URI of the W3C document]. Copyright © [YEAR] W3C® (MIT, ERCIM, Keio, Beihang)."
//
// Disclaimers
//
// THIS WORK IS PROVIDED "AS IS," AND COPYRIGHT HOLDERS MAKE NO REPRESENTATIONS OR WARRANTIES,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY OR FITNESS FOR
// ANY PARTICULAR PURPOSE OR THAT THE USE OF THE SOFTWARE OR DOCUMENT WILL NOT INFRINGE ANY THIRD
// PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
//
// COPYRIGHT HOLDERS WILL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL OR CONSEQUENTIAL DAMAGES
// ARISING OUT OF ANY USE OF THE SOFTWARE OR DOCUMENT.
//
// The name and trademarks of copyright holders may NOT be used in advertising or publicity
// pertaining to the work without specific, written prior permission. Title to copyright in this
// work will at all times remain with copyright holders.
//
// --------- END OF W3C LICENSE --------------------------------------------------------------------

// --------- BEGINNING OF W3C SHORT NOTICE ---------------------------------------------------------
//
// winit: https://github.com/rust-windowing/winit
//
// Copyright © 2021 World Wide Web Consortium, (Massachusetts Institute of Technology, European
// Research Consortium for Informatics and Mathematics, Keio University, Beihang). All Rights
// Reserved. This work is distributed under the W3C® Software License [1] in the hope that it will
// be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.
//
// [1] http://www.w3.org/Consortium/Legal/copyright-software
//
// --------- END OF W3C SHORT NOTICE ---------------------------------------------------------------

/// Contains the platform-native logical key identifier
///
/// Exactly what that means differs from platform to platform, but the values are to some degree
/// tied to the currently active keyboard layout. The same key on the same keyboard may also report
/// different values on different platforms, which is one of the reasons this is a per-platform
/// enum.
///
/// This enum is primarily used to store raw keysym when Winit doesn't map a given native logical
/// key identifier to a meaningful [`Key`] variant. This lets you use [`Key`], and let the user
/// define keybinds which work in the presence of identifiers we haven't mapped for you yet.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NativeKey {
    Unidentified,
    /// An Android "keycode", which is similar to a "virtual-key code" on Windows.
    Android(u32),
    /// A macOS "scancode". There does not appear to be any direct analogue to either keysyms or
    /// "virtual-key" codes in macOS, so we report the scancode instead.
    MacOS(u16),
    /// A Windows "virtual-key code".
    Windows(u16),
    /// An XKB "keysym".
    Xkb(u32),
    /// A "key value string".
    Web(KeyText),
}

impl From<WinitKeyNative> for NativeKey
{
    fn from(value: WinitKeyNative) -> Self {
        match value
        {
            winit::keyboard::NativeKey::Unidentified => NativeKey::Unidentified,
            winit::keyboard::NativeKey::Android(v) => NativeKey::Android(v),
            winit::keyboard::NativeKey::MacOS(v) => NativeKey::MacOS(v),
            winit::keyboard::NativeKey::Windows(v) => NativeKey::Windows(v),
            winit::keyboard::NativeKey::Xkb(v) => NativeKey::Xkb(v),
            winit::keyboard::NativeKey::Web(v) => NativeKey::Web(v),
        }
    }
}


impl Debug for NativeKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use NativeKey::{Android, MacOS, Unidentified, Web, Windows, Xkb};
        let mut debug_tuple;
        match self {
            Unidentified => {
                debug_tuple = f.debug_tuple("Unidentified");
            },
            Android(code) => {
                debug_tuple = f.debug_tuple("Android");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            MacOS(code) => {
                debug_tuple = f.debug_tuple("MacOS");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            Windows(code) => {
                debug_tuple = f.debug_tuple("Windows");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            Xkb(code) => {
                debug_tuple = f.debug_tuple("Xkb");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            Web(code) => {
                debug_tuple = f.debug_tuple("Web");
                debug_tuple.field(code);
            },
        }
        debug_tuple.finish()
    }
}

impl From<NativeKeyCode> for NativeKey {
    #[inline]
    fn from(code: NativeKeyCode) -> Self {
        match code {
            NativeKeyCode::Unidentified => NativeKey::Unidentified,
            NativeKeyCode::Android(x) => NativeKey::Android(x),
            NativeKeyCode::MacOS(x) => NativeKey::MacOS(x),
            NativeKeyCode::Windows(x) => NativeKey::Windows(x),
            NativeKeyCode::Xkb(x) => NativeKey::Xkb(x),
        }
    }
}

impl PartialEq<NativeKey> for NativeKeyCode {
    #[allow(clippy::cmp_owned)] // uses less code than direct match; target is stack allocated
    #[inline]
    fn eq(&self, rhs: &NativeKey) -> bool {
        NativeKey::from(*self) == *rhs
    }
}


impl PartialEq<NativeKeyCode> for NativeKey {
    #[inline]
    fn eq(&self, rhs: &NativeKeyCode) -> bool {
        rhs == self
    }
}

/// A [`Key::Named`] value
///
/// This mostly conforms to the UI Events Specification's [`KeyboardEvent.key`] with a few
/// exceptions:
/// - The `Super` variant here, is named `Meta` in the aforementioned specification. (There's
///   another key which the specification calls `Super`. That does not exist here.)
/// - The `Space` variant here, can be identified by the character it generates in the
///   specification.
///
/// [`KeyboardEvent.key`]: https://w3c.github.io/uievents-key/
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NamedKey 
{
    /// `Unknow` / error value, used if the convertion from Winit NamedKey was not handled 
    /// (because Winit NamedKey is also non exhaustive)
    Unknow,

    /// The `Alt` (Alternative) key.
    ///
    /// This key enables the alternate modifier function for interpreting concurrent or subsequent
    /// keyboard input. This key value is also used for the Apple <kbd>Option</kbd> key.
    Alt,
    /// The Alternate Graphics (<kbd>AltGr</kbd> or <kbd>AltGraph</kbd>) key.
    ///
    /// This key is used enable the ISO Level 3 shift modifier (the standard `Shift` key is the
    /// level 2 modifier).
    AltGraph,
    /// The `Caps Lock` (Capital) key.
    ///
    /// Toggle capital character lock function for interpreting subsequent keyboard input event.
    CapsLock,
    /// The `Control` or `Ctrl` key.
    ///
    /// Used to enable control modifier function for interpreting concurrent or subsequent keyboard
    /// input.
    Control,
    /// The Function switch `Fn` key. Activating this key simultaneously with another key changes
    /// that key’s value to an alternate character or function. This key is often handled directly
    /// in the keyboard hardware and does not usually generate key events.
    Fn,
    /// The Function-Lock (`FnLock` or `F-Lock`) key. Activating this key switches the mode of the
    /// keyboard to changes some keys' values to an alternate character or function. This key is
    /// often handled directly in the keyboard hardware and does not usually generate key events.
    FnLock,
    /// The `NumLock` or Number Lock key. Used to toggle numpad mode function for interpreting
    /// subsequent keyboard input.
    NumLock,
    /// Toggle between scrolling and cursor movement modes.
    ScrollLock,
    /// Used to enable shift modifier function for interpreting concurrent or subsequent keyboard
    /// input.
    Shift,
    /// The Symbol modifier key (used on some virtual keyboards).
    Symbol,
    SymbolLock,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    /// Used to enable "super" modifier function for interpreting concurrent or subsequent keyboard
    /// input. This key value is used for the "Windows Logo" key and the Apple `Command` or `⌘`
    /// key.
    ///
    /// Note: In some contexts (e.g. the Web) this is referred to as the "Meta" key.
    Super,
    /// The `Enter` or `↵` key. Used to activate current selection or accept current input. This
    /// key value is also used for the `Return` (Macintosh numpad) key. This key value is also
    /// used for the Android `KEYCODE_DPAD_CENTER`.
    Enter,
    /// The Horizontal Tabulation `Tab` key.
    Tab,
    /// Used in text to insert a space between words. Usually located below the character keys.
    Space,
    /// Navigate or traverse downward. (`KEYCODE_DPAD_DOWN`)
    ArrowDown,
    /// Navigate or traverse leftward. (`KEYCODE_DPAD_LEFT`)
    ArrowLeft,
    /// Navigate or traverse rightward. (`KEYCODE_DPAD_RIGHT`)
    ArrowRight,
    /// Navigate or traverse upward. (`KEYCODE_DPAD_UP`)
    ArrowUp,
    /// The End key, used with keyboard entry to go to the end of content (`KEYCODE_MOVE_END`).
    End,
    /// The Home key, used with keyboard entry, to go to start of content (`KEYCODE_MOVE_HOME`).
    /// For the mobile phone `Home` key (which goes to the phone’s main screen), use [`GoHome`].
    ///
    /// [`GoHome`]: Self::GoHome
    Home,
    /// Scroll down or display next page of content.
    PageDown,
    /// Scroll up or display previous page of content.
    PageUp,
    /// Used to remove the character to the left of the cursor. This key value is also used for
    /// the key labeled `Delete` on MacOS keyboards.
    Backspace,
    /// Remove the currently selected input.
    Clear,
    /// Copy the current selection. (`APPCOMMAND_COPY`)
    Copy,
    /// The Cursor Select key.
    CrSel,
    /// Cut the current selection. (`APPCOMMAND_CUT`)
    Cut,
    /// Used to delete the character to the right of the cursor. This key value is also used for
    /// the key labeled `Delete` on MacOS keyboards when `Fn` is active.
    Delete,
    /// The Erase to End of Field key. This key deletes all characters from the current cursor
    /// position to the end of the current field.
    EraseEof,
    /// The Extend Selection (Exsel) key.
    ExSel,
    /// Toggle between text modes for insertion or overtyping.
    /// (`KEYCODE_INSERT`)
    Insert,
    /// The Paste key. (`APPCOMMAND_PASTE`)
    Paste,
    /// Redo the last action. (`APPCOMMAND_REDO`)
    Redo,
    /// Undo the last action. (`APPCOMMAND_UNDO`)
    Undo,
    /// The Accept (Commit, OK) key. Accept current option or input method sequence conversion.
    Accept,
    /// Redo or repeat an action.
    Again,
    /// The Attention (Attn) key.
    Attn,
    Cancel,
    /// Show the application’s context menu.
    /// This key is commonly found between the right `Super` key and the right `Control` key.
    ContextMenu,
    /// The `Esc` key. This key was originally used to initiate an escape sequence, but is
    /// now more generally used to exit or "escape" the current context, such as closing a dialog
    /// or exiting full screen mode.
    Escape,
    Execute,
    /// Open the Find dialog. (`APPCOMMAND_FIND`)
    Find,
    /// Open a help dialog or toggle display of help information. (`APPCOMMAND_HELP`,
    /// `KEYCODE_HELP`)
    Help,
    /// Pause the current state or application (as appropriate).
    ///
    /// Note: Do not use this value for the `Pause` button on media controllers. Use `"MediaPause"`
    /// instead.
    Pause,
    /// Play or resume the current state or application (as appropriate).
    ///
    /// Note: Do not use this value for the `Play` button on media controllers. Use `"MediaPlay"`
    /// instead.
    Play,
    /// The properties (Props) key.
    Props,
    Select,
    /// The ZoomIn key. (`KEYCODE_ZOOM_IN`)
    ZoomIn,
    /// The ZoomOut key. (`KEYCODE_ZOOM_OUT`)
    ZoomOut,
    /// The Brightness Down key. Typically controls the display brightness.
    /// (`KEYCODE_BRIGHTNESS_DOWN`)
    BrightnessDown,
    /// The Brightness Up key. Typically controls the display brightness. (`KEYCODE_BRIGHTNESS_UP`)
    BrightnessUp,
    /// Toggle removable media to eject (open) and insert (close) state. (`KEYCODE_MEDIA_EJECT`)
    Eject,
    LogOff,
    /// Toggle power state. (`KEYCODE_POWER`)
    /// Note: Note: Some devices might not expose this key to the operating environment.
    Power,
    /// The `PowerOff` key. Sometime called `PowerDown`.
    PowerOff,
    /// Initiate print-screen function.
    PrintScreen,
    /// The Hibernate key. This key saves the current state of the computer to disk so that it can
    /// be restored. The computer will then shutdown.
    Hibernate,
    /// The Standby key. This key turns off the display and places the computer into a low-power
    /// mode without completely shutting down. It is sometimes labelled `Suspend` or `Sleep` key.
    /// (`KEYCODE_SLEEP`)
    Standby,
    /// The WakeUp key. (`KEYCODE_WAKEUP`)
    WakeUp,
    /// Initiate the multi-candidate mode.
    AllCandidates,
    Alphanumeric,
    /// Initiate the Code Input mode to allow characters to be entered by
    /// their code points.
    CodeInput,
    /// The Compose key, also known as "Multi_key" on the X Window System. This key acts in a
    /// manner similar to a dead key, triggering a mode where subsequent key presses are combined
    /// to produce a different character.
    Compose,
    /// Convert the current input method sequence.
    Convert,
    /// The Final Mode `Final` key used on some Asian keyboards, to enable the final mode for IMEs.
    FinalMode,
    /// Switch to the first character group. (ISO/IEC 9995)
    GroupFirst,
    /// Switch to the last character group. (ISO/IEC 9995)
    GroupLast,
    /// Switch to the next character group. (ISO/IEC 9995)
    GroupNext,
    /// Switch to the previous character group. (ISO/IEC 9995)
    GroupPrevious,
    /// Toggle between or cycle through input modes of IMEs.
    ModeChange,
    NextCandidate,
    /// Accept current input method sequence without
    /// conversion in IMEs.
    NonConvert,
    PreviousCandidate,
    Process,
    SingleCandidate,
    /// Toggle between Hangul and English modes.
    HangulMode,
    HanjaMode,
    JunjaMode,
    /// The Eisu key. This key may close the IME, but its purpose is defined by the current IME.
    /// (`KEYCODE_EISU`)
    Eisu,
    /// The (Half-Width) Characters key.
    Hankaku,
    /// The Hiragana (Japanese Kana characters) key.
    Hiragana,
    /// The Hiragana/Katakana toggle key. (`KEYCODE_KATAKANA_HIRAGANA`)
    HiraganaKatakana,
    /// The Kana Mode (Kana Lock) key. This key is used to enter hiragana mode (typically from
    /// romaji mode).
    KanaMode,
    /// The Kanji (Japanese name for ideographic characters of Chinese origin) Mode key. This key
    /// is typically used to switch to a hiragana keyboard for the purpose of converting input
    /// into kanji. (`KEYCODE_KANA`)
    KanjiMode,
    /// The Katakana (Japanese Kana characters) key.
    Katakana,
    /// The Roman characters function key.
    Romaji,
    /// The Zenkaku (Full-Width) Characters key.
    Zenkaku,
    /// The Zenkaku/Hankaku (full-width/half-width) toggle key. (`KEYCODE_ZENKAKU_HANKAKU`)
    ZenkakuHankaku,
    /// General purpose virtual function key, as index 1.
    Soft1,
    /// General purpose virtual function key, as index 2.
    Soft2,
    /// General purpose virtual function key, as index 3.
    Soft3,
    /// General purpose virtual function key, as index 4.
    Soft4,
    /// Select next (numerically or logically) lower channel. (`APPCOMMAND_MEDIA_CHANNEL_DOWN`,
    /// `KEYCODE_CHANNEL_DOWN`)
    ChannelDown,
    /// Select next (numerically or logically) higher channel. (`APPCOMMAND_MEDIA_CHANNEL_UP`,
    /// `KEYCODE_CHANNEL_UP`)
    ChannelUp,
    /// Close the current document or message (Note: This doesn’t close the application).
    /// (`APPCOMMAND_CLOSE`)
    Close,
    /// Open an editor to forward the current message. (`APPCOMMAND_FORWARD_MAIL`)
    MailForward,
    /// Open an editor to reply to the current message. (`APPCOMMAND_REPLY_TO_MAIL`)
    MailReply,
    /// Send the current message. (`APPCOMMAND_SEND_MAIL`)
    MailSend,
    /// Close the current media, for example to close a CD or DVD tray. (`KEYCODE_MEDIA_CLOSE`)
    MediaClose,
    /// Initiate or continue forward playback at faster than normal speed, or increase speed if
    /// already fast forwarding. (`APPCOMMAND_MEDIA_FAST_FORWARD`, `KEYCODE_MEDIA_FAST_FORWARD`)
    MediaFastForward,
    /// Pause the currently playing media. (`APPCOMMAND_MEDIA_PAUSE`, `KEYCODE_MEDIA_PAUSE`)
    ///
    /// Note: Media controller devices should use this value rather than `"Pause"` for their pause
    /// keys.
    MediaPause,
    /// Initiate or continue media playback at normal speed, if not currently playing at normal
    /// speed. (`APPCOMMAND_MEDIA_PLAY`, `KEYCODE_MEDIA_PLAY`)
    MediaPlay,
    /// Toggle media between play and pause states. (`APPCOMMAND_MEDIA_PLAY_PAUSE`,
    /// `KEYCODE_MEDIA_PLAY_PAUSE`)
    MediaPlayPause,
    /// Initiate or resume recording of currently selected media. (`APPCOMMAND_MEDIA_RECORD`,
    /// `KEYCODE_MEDIA_RECORD`)
    MediaRecord,
    /// Initiate or continue reverse playback at faster than normal speed, or increase speed if
    /// already rewinding. (`APPCOMMAND_MEDIA_REWIND`, `KEYCODE_MEDIA_REWIND`)
    MediaRewind,
    /// Stop media playing, pausing, forwarding, rewinding, or recording, if not already stopped.
    /// (`APPCOMMAND_MEDIA_STOP`, `KEYCODE_MEDIA_STOP`)
    MediaStop,
    /// Seek to next media or program track. (`APPCOMMAND_MEDIA_NEXTTRACK`, `KEYCODE_MEDIA_NEXT`)
    MediaTrackNext,
    /// Seek to previous media or program track. (`APPCOMMAND_MEDIA_PREVIOUSTRACK`,
    /// `KEYCODE_MEDIA_PREVIOUS`)
    MediaTrackPrevious,
    /// Open a new document or message. (`APPCOMMAND_NEW`)
    New,
    /// Open an existing document or message. (`APPCOMMAND_OPEN`)
    Open,
    /// Print the current document or message. (`APPCOMMAND_PRINT`)
    Print,
    /// Save the current document or message. (`APPCOMMAND_SAVE`)
    Save,
    /// Spellcheck the current document or selection. (`APPCOMMAND_SPELL_CHECK`)
    SpellCheck,
    /// The `11` key found on media numpads that
    /// have buttons from `1` ... `12`.
    Key11,
    /// The `12` key found on media numpads that
    /// have buttons from `1` ... `12`.
    Key12,
    /// Adjust audio balance leftward. (`VK_AUDIO_BALANCE_LEFT`)
    AudioBalanceLeft,
    /// Adjust audio balance rightward. (`VK_AUDIO_BALANCE_RIGHT`)
    AudioBalanceRight,
    /// Decrease audio bass boost or cycle down through bass boost states. (`APPCOMMAND_BASS_DOWN`,
    /// `VK_BASS_BOOST_DOWN`)
    AudioBassBoostDown,
    /// Toggle bass boost on/off. (`APPCOMMAND_BASS_BOOST`)
    AudioBassBoostToggle,
    /// Increase audio bass boost or cycle up through bass boost states. (`APPCOMMAND_BASS_UP`,
    /// `VK_BASS_BOOST_UP`)
    AudioBassBoostUp,
    /// Adjust audio fader towards front. (`VK_FADER_FRONT`)
    AudioFaderFront,
    /// Adjust audio fader towards rear. (`VK_FADER_REAR`)
    AudioFaderRear,
    /// Advance surround audio mode to next available mode. (`VK_SURROUND_MODE_NEXT`)
    AudioSurroundModeNext,
    /// Decrease treble. (`APPCOMMAND_TREBLE_DOWN`)
    AudioTrebleDown,
    /// Increase treble. (`APPCOMMAND_TREBLE_UP`)
    AudioTrebleUp,
    /// Decrease audio volume. (`APPCOMMAND_VOLUME_DOWN`, `KEYCODE_VOLUME_DOWN`)
    AudioVolumeDown,
    /// Increase audio volume. (`APPCOMMAND_VOLUME_UP`, `KEYCODE_VOLUME_UP`)
    AudioVolumeUp,
    /// Toggle between muted state and prior volume level. (`APPCOMMAND_VOLUME_MUTE`,
    /// `KEYCODE_VOLUME_MUTE`)
    AudioVolumeMute,
    /// Toggle the microphone on/off. (`APPCOMMAND_MIC_ON_OFF_TOGGLE`)
    MicrophoneToggle,
    /// Decrease microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_DOWN`)
    MicrophoneVolumeDown,
    /// Increase microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_UP`)
    MicrophoneVolumeUp,
    /// Mute the microphone. (`APPCOMMAND_MICROPHONE_VOLUME_MUTE`, `KEYCODE_MUTE`)
    MicrophoneVolumeMute,
    /// Show correction list when a word is incorrectly identified. (`APPCOMMAND_CORRECTION_LIST`)
    SpeechCorrectionList,
    /// Toggle between dictation mode and command/control mode.
    /// (`APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE`)
    SpeechInputToggle,
    /// The first generic "LaunchApplication" key. This is commonly associated with launching "My
    /// Computer", and may have a computer symbol on the key. (`APPCOMMAND_LAUNCH_APP1`)
    LaunchApplication1,
    /// The second generic "LaunchApplication" key. This is commonly associated with launching
    /// "Calculator", and may have a calculator symbol on the key. (`APPCOMMAND_LAUNCH_APP2`,
    /// `KEYCODE_CALCULATOR`)
    LaunchApplication2,
    /// The "Calendar" key. (`KEYCODE_CALENDAR`)
    LaunchCalendar,
    /// The "Contacts" key. (`KEYCODE_CONTACTS`)
    LaunchContacts,
    /// The "Mail" key. (`APPCOMMAND_LAUNCH_MAIL`)
    LaunchMail,
    /// The "Media Player" key. (`APPCOMMAND_LAUNCH_MEDIA_SELECT`)
    LaunchMediaPlayer,
    LaunchMusicPlayer,
    LaunchPhone,
    LaunchScreenSaver,
    LaunchSpreadsheet,
    LaunchWebBrowser,
    LaunchWebCam,
    LaunchWordProcessor,
    /// Navigate to previous content or page in current history. (`APPCOMMAND_BROWSER_BACKWARD`)
    BrowserBack,
    /// Open the list of browser favorites. (`APPCOMMAND_BROWSER_FAVORITES`)
    BrowserFavorites,
    /// Navigate to next content or page in current history. (`APPCOMMAND_BROWSER_FORWARD`)
    BrowserForward,
    /// Go to the user’s preferred home page. (`APPCOMMAND_BROWSER_HOME`)
    BrowserHome,
    /// Refresh the current page or content. (`APPCOMMAND_BROWSER_REFRESH`)
    BrowserRefresh,
    /// Call up the user’s preferred search page. (`APPCOMMAND_BROWSER_SEARCH`)
    BrowserSearch,
    /// Stop loading the current page or content. (`APPCOMMAND_BROWSER_STOP`)
    BrowserStop,
    /// The Application switch key, which provides a list of recent apps to switch between.
    /// (`KEYCODE_APP_SWITCH`)
    AppSwitch,
    /// The Call key. (`KEYCODE_CALL`)
    Call,
    /// The Camera key. (`KEYCODE_CAMERA`)
    Camera,
    /// The Camera focus key. (`KEYCODE_FOCUS`)
    CameraFocus,
    /// The End Call key. (`KEYCODE_ENDCALL`)
    EndCall,
    /// The Back key. (`KEYCODE_BACK`)
    GoBack,
    /// The Home key, which goes to the phone’s main screen. (`KEYCODE_HOME`)
    GoHome,
    /// The Headset Hook key. (`KEYCODE_HEADSETHOOK`)
    HeadsetHook,
    LastNumberRedial,
    /// The Notification key. (`KEYCODE_NOTIFICATION`)
    Notification,
    /// Toggle between manner mode state: silent, vibrate, ring, ... (`KEYCODE_MANNER_MODE`)
    MannerMode,
    VoiceDial,
    /// Switch to viewing TV. (`KEYCODE_TV`)
    TV,
    /// TV 3D Mode. (`KEYCODE_3D_MODE`)
    TV3DMode,
    /// Toggle between antenna and cable input. (`KEYCODE_TV_ANTENNA_CABLE`)
    TVAntennaCable,
    /// Audio description. (`KEYCODE_TV_AUDIO_DESCRIPTION`)
    TVAudioDescription,
    /// Audio description mixing volume down. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN`)
    TVAudioDescriptionMixDown,
    /// Audio description mixing volume up. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP`)
    TVAudioDescriptionMixUp,
    /// Contents menu. (`KEYCODE_TV_CONTENTS_MENU`)
    TVContentsMenu,
    /// Contents menu. (`KEYCODE_TV_DATA_SERVICE`)
    TVDataService,
    /// Switch the input mode on an external TV. (`KEYCODE_TV_INPUT`)
    TVInput,
    /// Switch to component input #1. (`KEYCODE_TV_INPUT_COMPONENT_1`)
    TVInputComponent1,
    /// Switch to component input #2. (`KEYCODE_TV_INPUT_COMPONENT_2`)
    TVInputComponent2,
    /// Switch to composite input #1. (`KEYCODE_TV_INPUT_COMPOSITE_1`)
    TVInputComposite1,
    /// Switch to composite input #2. (`KEYCODE_TV_INPUT_COMPOSITE_2`)
    TVInputComposite2,
    /// Switch to HDMI input #1. (`KEYCODE_TV_INPUT_HDMI_1`)
    TVInputHDMI1,
    /// Switch to HDMI input #2. (`KEYCODE_TV_INPUT_HDMI_2`)
    TVInputHDMI2,
    /// Switch to HDMI input #3. (`KEYCODE_TV_INPUT_HDMI_3`)
    TVInputHDMI3,
    /// Switch to HDMI input #4. (`KEYCODE_TV_INPUT_HDMI_4`)
    TVInputHDMI4,
    /// Switch to VGA input #1. (`KEYCODE_TV_INPUT_VGA_1`)
    TVInputVGA1,
    /// Media context menu. (`KEYCODE_TV_MEDIA_CONTEXT_MENU`)
    TVMediaContext,
    /// Toggle network. (`KEYCODE_TV_NETWORK`)
    TVNetwork,
    /// Number entry. (`KEYCODE_TV_NUMBER_ENTRY`)
    TVNumberEntry,
    /// Toggle the power on an external TV. (`KEYCODE_TV_POWER`)
    TVPower,
    /// Radio. (`KEYCODE_TV_RADIO_SERVICE`)
    TVRadioService,
    /// Satellite. (`KEYCODE_TV_SATELLITE`)
    TVSatellite,
    /// Broadcast Satellite. (`KEYCODE_TV_SATELLITE_BS`)
    TVSatelliteBS,
    /// Communication Satellite. (`KEYCODE_TV_SATELLITE_CS`)
    TVSatelliteCS,
    /// Toggle between available satellites. (`KEYCODE_TV_SATELLITE_SERVICE`)
    TVSatelliteToggle,
    /// Analog Terrestrial. (`KEYCODE_TV_TERRESTRIAL_ANALOG`)
    TVTerrestrialAnalog,
    /// Digital Terrestrial. (`KEYCODE_TV_TERRESTRIAL_DIGITAL`)
    TVTerrestrialDigital,
    /// Timer programming. (`KEYCODE_TV_TIMER_PROGRAMMING`)
    TVTimer,
    /// Switch the input mode on an external AVR (audio/video receiver). (`KEYCODE_AVR_INPUT`)
    AVRInput,
    /// Toggle the power on an external AVR (audio/video receiver). (`KEYCODE_AVR_POWER`)
    AVRPower,
    /// General purpose color-coded media function key, as index 0 (red). (`VK_COLORED_KEY_0`,
    /// `KEYCODE_PROG_RED`)
    ColorF0Red,
    /// General purpose color-coded media function key, as index 1 (green). (`VK_COLORED_KEY_1`,
    /// `KEYCODE_PROG_GREEN`)
    ColorF1Green,
    /// General purpose color-coded media function key, as index 2 (yellow). (`VK_COLORED_KEY_2`,
    /// `KEYCODE_PROG_YELLOW`)
    ColorF2Yellow,
    /// General purpose color-coded media function key, as index 3 (blue). (`VK_COLORED_KEY_3`,
    /// `KEYCODE_PROG_BLUE`)
    ColorF3Blue,
    /// General purpose color-coded media function key, as index 4 (grey). (`VK_COLORED_KEY_4`)
    ColorF4Grey,
    /// General purpose color-coded media function key, as index 5 (brown). (`VK_COLORED_KEY_5`)
    ColorF5Brown,
    /// Toggle the display of Closed Captions. (`VK_CC`, `KEYCODE_CAPTIONS`)
    ClosedCaptionToggle,
    /// Adjust brightness of device, by toggling between or cycling through states. (`VK_DIMMER`)
    Dimmer,
    /// Swap video sources. (`VK_DISPLAY_SWAP`)
    DisplaySwap,
    /// Select Digital Video Recorder. (`KEYCODE_DVR`)
    DVR,
    /// Exit the current application. (`VK_EXIT`)
    Exit,
    /// Clear program or content stored as favorite 0. (`VK_CLEAR_FAVORITE_0`)
    FavoriteClear0,
    /// Clear program or content stored as favorite 1. (`VK_CLEAR_FAVORITE_1`)
    FavoriteClear1,
    /// Clear program or content stored as favorite 2. (`VK_CLEAR_FAVORITE_2`)
    FavoriteClear2,
    /// Clear program or content stored as favorite 3. (`VK_CLEAR_FAVORITE_3`)
    FavoriteClear3,
    /// Select (recall) program or content stored as favorite 0. (`VK_RECALL_FAVORITE_0`)
    FavoriteRecall0,
    /// Select (recall) program or content stored as favorite 1. (`VK_RECALL_FAVORITE_1`)
    FavoriteRecall1,
    /// Select (recall) program or content stored as favorite 2. (`VK_RECALL_FAVORITE_2`)
    FavoriteRecall2,
    /// Select (recall) program or content stored as favorite 3. (`VK_RECALL_FAVORITE_3`)
    FavoriteRecall3,
    /// Store current program or content as favorite 0. (`VK_STORE_FAVORITE_0`)
    FavoriteStore0,
    /// Store current program or content as favorite 1. (`VK_STORE_FAVORITE_1`)
    FavoriteStore1,
    /// Store current program or content as favorite 2. (`VK_STORE_FAVORITE_2`)
    FavoriteStore2,
    /// Store current program or content as favorite 3. (`VK_STORE_FAVORITE_3`)
    FavoriteStore3,
    /// Toggle display of program or content guide. (`VK_GUIDE`, `KEYCODE_GUIDE`)
    Guide,
    /// If guide is active and displayed, then display next day’s content. (`VK_NEXT_DAY`)
    GuideNextDay,
    /// If guide is active and displayed, then display previous day’s content. (`VK_PREV_DAY`)
    GuidePreviousDay,
    /// Toggle display of information about currently selected context or media. (`VK_INFO`,
    /// `KEYCODE_INFO`)
    Info,
    /// Toggle instant replay. (`VK_INSTANT_REPLAY`)
    InstantReplay,
    /// Launch linked content, if available and appropriate. (`VK_LINK`)
    Link,
    /// List the current program. (`VK_LIST`)
    ListProgram,
    /// Toggle display listing of currently available live content or programs. (`VK_LIVE`)
    LiveContent,
    /// Lock or unlock current content or program. (`VK_LOCK`)
    Lock,
    /// Show a list of media applications: audio/video players and image viewers. (`VK_APPS`)
    ///
    /// Note: Do not confuse this key value with the Windows' `VK_APPS` / `VK_CONTEXT_MENU` key,
    /// which is encoded as `"ContextMenu"`.
    MediaApps,
    /// Audio track key. (`KEYCODE_MEDIA_AUDIO_TRACK`)
    MediaAudioTrack,
    /// Select previously selected channel or media. (`VK_LAST`, `KEYCODE_LAST_CHANNEL`)
    MediaLast,
    /// Skip backward to next content or program. (`KEYCODE_MEDIA_SKIP_BACKWARD`)
    MediaSkipBackward,
    /// Skip forward to next content or program. (`VK_SKIP`, `KEYCODE_MEDIA_SKIP_FORWARD`)
    MediaSkipForward,
    /// Step backward to next content or program. (`KEYCODE_MEDIA_STEP_BACKWARD`)
    MediaStepBackward,
    /// Step forward to next content or program. (`KEYCODE_MEDIA_STEP_FORWARD`)
    MediaStepForward,
    /// Media top menu. (`KEYCODE_MEDIA_TOP_MENU`)
    MediaTopMenu,
    /// Navigate in. (`KEYCODE_NAVIGATE_IN`)
    NavigateIn,
    /// Navigate to next key. (`KEYCODE_NAVIGATE_NEXT`)
    NavigateNext,
    /// Navigate out. (`KEYCODE_NAVIGATE_OUT`)
    NavigateOut,
    /// Navigate to previous key. (`KEYCODE_NAVIGATE_PREVIOUS`)
    NavigatePrevious,
    /// Cycle to next favorite channel (in favorites list). (`VK_NEXT_FAVORITE_CHANNEL`)
    NextFavoriteChannel,
    /// Cycle to next user profile (if there are multiple user profiles). (`VK_USER`)
    NextUserProfile,
    /// Access on-demand content or programs. (`VK_ON_DEMAND`)
    OnDemand,
    /// Pairing key to pair devices. (`KEYCODE_PAIRING`)
    Pairing,
    /// Move picture-in-picture window down. (`VK_PINP_DOWN`)
    PinPDown,
    /// Move picture-in-picture window. (`VK_PINP_MOVE`)
    PinPMove,
    /// Toggle display of picture-in-picture window. (`VK_PINP_TOGGLE`)
    PinPToggle,
    /// Move picture-in-picture window up. (`VK_PINP_UP`)
    PinPUp,
    /// Decrease media playback speed. (`VK_PLAY_SPEED_DOWN`)
    PlaySpeedDown,
    /// Reset playback to normal speed. (`VK_PLAY_SPEED_RESET`)
    PlaySpeedReset,
    /// Increase media playback speed. (`VK_PLAY_SPEED_UP`)
    PlaySpeedUp,
    /// Toggle random media or content shuffle mode. (`VK_RANDOM_TOGGLE`)
    RandomToggle,
    /// Not a physical key, but this key code is sent when the remote control battery is low.
    /// (`VK_RC_LOW_BATTERY`)
    RcLowBattery,
    /// Toggle or cycle between media recording speeds. (`VK_RECORD_SPEED_NEXT`)
    RecordSpeedNext,
    /// Toggle RF (radio frequency) input bypass mode (pass RF input directly to the RF output).
    /// (`VK_RF_BYPASS`)
    RfBypass,
    /// Toggle scan channels mode. (`VK_SCAN_CHANNELS_TOGGLE`)
    ScanChannelsToggle,
    /// Advance display screen mode to next available mode. (`VK_SCREEN_MODE_NEXT`)
    ScreenModeNext,
    /// Toggle display of device settings screen. (`VK_SETTINGS`, `KEYCODE_SETTINGS`)
    Settings,
    /// Toggle split screen mode. (`VK_SPLIT_SCREEN_TOGGLE`)
    SplitScreenToggle,
    /// Switch the input mode on an external STB (set top box). (`KEYCODE_STB_INPUT`)
    STBInput,
    /// Toggle the power on an external STB (set top box). (`KEYCODE_STB_POWER`)
    STBPower,
    /// Toggle display of subtitles, if available. (`VK_SUBTITLE`)
    Subtitle,
    /// Toggle display of teletext, if available (`VK_TELETEXT`, `KEYCODE_TV_TELETEXT`).
    Teletext,
    /// Advance video mode to next available mode. (`VK_VIDEO_MODE_NEXT`)
    VideoModeNext,
    /// Cause device to identify itself in some manner, e.g., audibly or visibly. (`VK_WINK`)
    Wink,
    /// Toggle between full-screen and scaled content, or alter magnification level. (`VK_ZOOM`,
    /// `KEYCODE_TV_ZOOM_MODE`)
    ZoomToggle,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
}

impl From<WinitKeyName> for NamedKey
{
    fn from(value: WinitKeyName) -> Self {
        match value
        {
            winit::keyboard::NamedKey::Alt => NamedKey::Alt,
            winit::keyboard::NamedKey::AltGraph => NamedKey::AltGraph,
            winit::keyboard::NamedKey::CapsLock => NamedKey::CapsLock,
            winit::keyboard::NamedKey::Control => NamedKey::Control,
            winit::keyboard::NamedKey::Fn => NamedKey::Fn,
            winit::keyboard::NamedKey::FnLock => NamedKey::FnLock,
            winit::keyboard::NamedKey::NumLock => NamedKey::NumLock,
            winit::keyboard::NamedKey::ScrollLock => NamedKey::ScrollLock,
            winit::keyboard::NamedKey::Shift => NamedKey::Shift,
            winit::keyboard::NamedKey::Symbol => NamedKey::Symbol,
            winit::keyboard::NamedKey::SymbolLock => NamedKey::SymbolLock,
            winit::keyboard::NamedKey::Meta => NamedKey::Meta,
            winit::keyboard::NamedKey::Hyper => NamedKey::Hyper,
            winit::keyboard::NamedKey::Super => NamedKey::Super,
            winit::keyboard::NamedKey::Enter => NamedKey::Enter,
            winit::keyboard::NamedKey::Tab => NamedKey::Tab,
            winit::keyboard::NamedKey::Space => NamedKey::Space,
            winit::keyboard::NamedKey::ArrowDown => NamedKey::ArrowDown,
            winit::keyboard::NamedKey::ArrowLeft => NamedKey::ArrowLeft,
            winit::keyboard::NamedKey::ArrowRight => NamedKey::ArrowRight,
            winit::keyboard::NamedKey::ArrowUp => NamedKey::ArrowUp,
            winit::keyboard::NamedKey::End => NamedKey::End,
            winit::keyboard::NamedKey::Home => NamedKey::Home,
            winit::keyboard::NamedKey::PageDown => NamedKey::PageDown,
            winit::keyboard::NamedKey::PageUp => NamedKey::PageUp,
            winit::keyboard::NamedKey::Backspace => NamedKey::Backspace,
            winit::keyboard::NamedKey::Clear => NamedKey::Clear,
            winit::keyboard::NamedKey::Copy => NamedKey::Copy,
            winit::keyboard::NamedKey::CrSel => NamedKey::CrSel,
            winit::keyboard::NamedKey::Cut => NamedKey::Cut,
            winit::keyboard::NamedKey::Delete => NamedKey::Delete,
            winit::keyboard::NamedKey::EraseEof => NamedKey::EraseEof,
            winit::keyboard::NamedKey::ExSel => NamedKey::ExSel,
            winit::keyboard::NamedKey::Insert => NamedKey::Insert,
            winit::keyboard::NamedKey::Paste => NamedKey::Paste,
            winit::keyboard::NamedKey::Redo => NamedKey::Redo,
            winit::keyboard::NamedKey::Undo => NamedKey::Undo,
            winit::keyboard::NamedKey::Accept => NamedKey::Accept,
            winit::keyboard::NamedKey::Again => NamedKey::Again,
            winit::keyboard::NamedKey::Attn => NamedKey::Attn,
            winit::keyboard::NamedKey::Cancel => NamedKey::Cancel,
            winit::keyboard::NamedKey::ContextMenu => NamedKey::ContextMenu,
            winit::keyboard::NamedKey::Escape => NamedKey::Escape,
            winit::keyboard::NamedKey::Execute => NamedKey::Execute,
            winit::keyboard::NamedKey::Find => NamedKey::Find,
            winit::keyboard::NamedKey::Help => NamedKey::Help,
            winit::keyboard::NamedKey::Pause => NamedKey::Pause,
            winit::keyboard::NamedKey::Play => NamedKey::Play,
            winit::keyboard::NamedKey::Props => NamedKey::Props,
            winit::keyboard::NamedKey::Select => NamedKey::Select,
            winit::keyboard::NamedKey::ZoomIn => NamedKey::ZoomIn,
            winit::keyboard::NamedKey::ZoomOut => NamedKey::ZoomOut,
            winit::keyboard::NamedKey::BrightnessDown => NamedKey::BrightnessDown,
            winit::keyboard::NamedKey::BrightnessUp => NamedKey::BrightnessUp,
            winit::keyboard::NamedKey::Eject => NamedKey::Eject,
            winit::keyboard::NamedKey::LogOff => NamedKey::LogOff,
            winit::keyboard::NamedKey::Power => NamedKey::Power,
            winit::keyboard::NamedKey::PowerOff => NamedKey::PowerOff,
            winit::keyboard::NamedKey::PrintScreen => NamedKey::PrintScreen,
            winit::keyboard::NamedKey::Hibernate => NamedKey::Hibernate,
            winit::keyboard::NamedKey::Standby => NamedKey::Standby,
            winit::keyboard::NamedKey::WakeUp => NamedKey::WakeUp,
            winit::keyboard::NamedKey::AllCandidates => NamedKey::AllCandidates,
            winit::keyboard::NamedKey::Alphanumeric => NamedKey::Alphanumeric,
            winit::keyboard::NamedKey::CodeInput => NamedKey::CodeInput,
            winit::keyboard::NamedKey::Compose => NamedKey::Compose,
            winit::keyboard::NamedKey::Convert => NamedKey::Convert,
            winit::keyboard::NamedKey::FinalMode => NamedKey::FinalMode,
            winit::keyboard::NamedKey::GroupFirst => NamedKey::GroupFirst,
            winit::keyboard::NamedKey::GroupLast => NamedKey::GroupLast,
            winit::keyboard::NamedKey::GroupNext => NamedKey::GroupNext,
            winit::keyboard::NamedKey::GroupPrevious => NamedKey::GroupPrevious,
            winit::keyboard::NamedKey::ModeChange => NamedKey::ModeChange,
            winit::keyboard::NamedKey::NextCandidate => NamedKey::NextCandidate,
            winit::keyboard::NamedKey::NonConvert => NamedKey::NonConvert,
            winit::keyboard::NamedKey::PreviousCandidate => NamedKey::PreviousCandidate,
            winit::keyboard::NamedKey::Process => NamedKey::Process,
            winit::keyboard::NamedKey::SingleCandidate => NamedKey::SingleCandidate,
            winit::keyboard::NamedKey::HangulMode => NamedKey::HangulMode,
            winit::keyboard::NamedKey::HanjaMode => NamedKey::HanjaMode,
            winit::keyboard::NamedKey::JunjaMode => NamedKey::JunjaMode,
            winit::keyboard::NamedKey::Eisu => NamedKey::Eisu,
            winit::keyboard::NamedKey::Hankaku => NamedKey::Hankaku,
            winit::keyboard::NamedKey::Hiragana => NamedKey::Hiragana,
            winit::keyboard::NamedKey::HiraganaKatakana => NamedKey::HiraganaKatakana,
            winit::keyboard::NamedKey::KanaMode => NamedKey::KanaMode,
            winit::keyboard::NamedKey::KanjiMode => NamedKey::KanjiMode,
            winit::keyboard::NamedKey::Katakana => NamedKey::Katakana,
            winit::keyboard::NamedKey::Romaji => NamedKey::Romaji,
            winit::keyboard::NamedKey::Zenkaku => NamedKey::Zenkaku,
            winit::keyboard::NamedKey::ZenkakuHankaku => NamedKey::ZenkakuHankaku,
            winit::keyboard::NamedKey::Soft1 => NamedKey::Soft1,
            winit::keyboard::NamedKey::Soft2 => NamedKey::Soft2,
            winit::keyboard::NamedKey::Soft3 => NamedKey::Soft3,
            winit::keyboard::NamedKey::Soft4 => NamedKey::Soft4,
            winit::keyboard::NamedKey::ChannelDown => NamedKey::ChannelDown,
            winit::keyboard::NamedKey::ChannelUp => NamedKey::ChannelUp,
            winit::keyboard::NamedKey::Close => NamedKey::Close,
            winit::keyboard::NamedKey::MailForward => NamedKey::MailForward,
            winit::keyboard::NamedKey::MailReply => NamedKey::MailReply,
            winit::keyboard::NamedKey::MailSend => NamedKey::MailSend,
            winit::keyboard::NamedKey::MediaClose => NamedKey::MediaClose,
            winit::keyboard::NamedKey::MediaFastForward => NamedKey::MediaFastForward,
            winit::keyboard::NamedKey::MediaPause => NamedKey::MediaPause,
            winit::keyboard::NamedKey::MediaPlay => NamedKey::MediaPlay,
            winit::keyboard::NamedKey::MediaPlayPause => NamedKey::MediaPlayPause,
            winit::keyboard::NamedKey::MediaRecord => NamedKey::MediaRecord,
            winit::keyboard::NamedKey::MediaRewind => NamedKey::MediaRewind,
            winit::keyboard::NamedKey::MediaStop => NamedKey::MediaStop,
            winit::keyboard::NamedKey::MediaTrackNext => NamedKey::MediaTrackNext,
            winit::keyboard::NamedKey::MediaTrackPrevious => NamedKey::MediaTrackPrevious,
            winit::keyboard::NamedKey::New => NamedKey::New,
            winit::keyboard::NamedKey::Open => NamedKey::Open,
            winit::keyboard::NamedKey::Print => NamedKey::Print,
            winit::keyboard::NamedKey::Save => NamedKey::Save,
            winit::keyboard::NamedKey::SpellCheck => NamedKey::SpellCheck,
            winit::keyboard::NamedKey::Key11 => NamedKey::Key11,
            winit::keyboard::NamedKey::Key12 => NamedKey::Key12,
            winit::keyboard::NamedKey::AudioBalanceLeft => NamedKey::AudioBalanceLeft,
            winit::keyboard::NamedKey::AudioBalanceRight => NamedKey::AudioBalanceRight,
            winit::keyboard::NamedKey::AudioBassBoostDown => NamedKey::AudioBassBoostDown,
            winit::keyboard::NamedKey::AudioBassBoostToggle => NamedKey::AudioBassBoostToggle,
            winit::keyboard::NamedKey::AudioBassBoostUp => NamedKey::AudioBassBoostUp,
            winit::keyboard::NamedKey::AudioFaderFront => NamedKey::AudioFaderFront,
            winit::keyboard::NamedKey::AudioFaderRear => NamedKey::AudioFaderRear,
            winit::keyboard::NamedKey::AudioSurroundModeNext => NamedKey::AudioSurroundModeNext,
            winit::keyboard::NamedKey::AudioTrebleDown => NamedKey::AudioTrebleDown,
            winit::keyboard::NamedKey::AudioTrebleUp => NamedKey::AudioTrebleUp,
            winit::keyboard::NamedKey::AudioVolumeDown => NamedKey::AudioVolumeDown,
            winit::keyboard::NamedKey::AudioVolumeUp => NamedKey::AudioVolumeUp,
            winit::keyboard::NamedKey::AudioVolumeMute => NamedKey::AudioVolumeMute,
            winit::keyboard::NamedKey::MicrophoneToggle => NamedKey::MicrophoneToggle,
            winit::keyboard::NamedKey::MicrophoneVolumeDown => NamedKey::MicrophoneVolumeDown,
            winit::keyboard::NamedKey::MicrophoneVolumeUp => NamedKey::MicrophoneVolumeUp,
            winit::keyboard::NamedKey::MicrophoneVolumeMute => NamedKey::MicrophoneVolumeMute,
            winit::keyboard::NamedKey::SpeechCorrectionList => NamedKey::SpeechCorrectionList,
            winit::keyboard::NamedKey::SpeechInputToggle => NamedKey::SpeechInputToggle,
            winit::keyboard::NamedKey::LaunchApplication1 => NamedKey::LaunchApplication1,
            winit::keyboard::NamedKey::LaunchApplication2 => NamedKey::LaunchApplication2,
            winit::keyboard::NamedKey::LaunchCalendar => NamedKey::LaunchCalendar,
            winit::keyboard::NamedKey::LaunchContacts => NamedKey::LaunchContacts,
            winit::keyboard::NamedKey::LaunchMail => NamedKey::LaunchMail,
            winit::keyboard::NamedKey::LaunchMediaPlayer => NamedKey::LaunchMediaPlayer,
            winit::keyboard::NamedKey::LaunchMusicPlayer => NamedKey::LaunchMusicPlayer,
            winit::keyboard::NamedKey::LaunchPhone => NamedKey::LaunchPhone,
            winit::keyboard::NamedKey::LaunchScreenSaver => NamedKey::LaunchScreenSaver,
            winit::keyboard::NamedKey::LaunchSpreadsheet => NamedKey::LaunchSpreadsheet,
            winit::keyboard::NamedKey::LaunchWebBrowser => NamedKey::LaunchWebBrowser,
            winit::keyboard::NamedKey::LaunchWebCam => NamedKey::LaunchWebCam,
            winit::keyboard::NamedKey::LaunchWordProcessor => NamedKey::LaunchWordProcessor,
            winit::keyboard::NamedKey::BrowserBack => NamedKey::BrowserBack,
            winit::keyboard::NamedKey::BrowserFavorites => NamedKey::BrowserFavorites,
            winit::keyboard::NamedKey::BrowserForward => NamedKey::BrowserForward,
            winit::keyboard::NamedKey::BrowserHome => NamedKey::BrowserHome,
            winit::keyboard::NamedKey::BrowserRefresh => NamedKey::BrowserRefresh,
            winit::keyboard::NamedKey::BrowserSearch => NamedKey::BrowserSearch,
            winit::keyboard::NamedKey::BrowserStop => NamedKey::BrowserStop,
            winit::keyboard::NamedKey::AppSwitch => NamedKey::AppSwitch,
            winit::keyboard::NamedKey::Call => NamedKey::Call,
            winit::keyboard::NamedKey::Camera => NamedKey::Camera,
            winit::keyboard::NamedKey::CameraFocus => NamedKey::CameraFocus,
            winit::keyboard::NamedKey::EndCall => NamedKey::EndCall,
            winit::keyboard::NamedKey::GoBack => NamedKey::GoBack,
            winit::keyboard::NamedKey::GoHome => NamedKey::GoHome,
            winit::keyboard::NamedKey::HeadsetHook => NamedKey::HeadsetHook,
            winit::keyboard::NamedKey::LastNumberRedial => NamedKey::LastNumberRedial,
            winit::keyboard::NamedKey::Notification => NamedKey::Notification,
            winit::keyboard::NamedKey::MannerMode => NamedKey::MannerMode,
            winit::keyboard::NamedKey::VoiceDial => NamedKey::VoiceDial,
            winit::keyboard::NamedKey::TV => NamedKey::TV,
            winit::keyboard::NamedKey::TV3DMode => NamedKey::TV3DMode,
            winit::keyboard::NamedKey::TVAntennaCable => NamedKey::TVAntennaCable,
            winit::keyboard::NamedKey::TVAudioDescription => NamedKey::TVAudioDescription,
            winit::keyboard::NamedKey::TVAudioDescriptionMixDown => NamedKey::TVAudioDescriptionMixDown,
            winit::keyboard::NamedKey::TVAudioDescriptionMixUp => NamedKey::TVAudioDescriptionMixUp,
            winit::keyboard::NamedKey::TVContentsMenu => NamedKey::TVContentsMenu,
            winit::keyboard::NamedKey::TVDataService => NamedKey::TVDataService,
            winit::keyboard::NamedKey::TVInput => NamedKey::TVInput,
            winit::keyboard::NamedKey::TVInputComponent1 => NamedKey::TVInputComponent1,
            winit::keyboard::NamedKey::TVInputComponent2 => NamedKey::TVInputComponent2,
            winit::keyboard::NamedKey::TVInputComposite1 => NamedKey::TVInputComposite1,
            winit::keyboard::NamedKey::TVInputComposite2 => NamedKey::TVInputComposite2,
            winit::keyboard::NamedKey::TVInputHDMI1 => NamedKey::TVInputHDMI1,
            winit::keyboard::NamedKey::TVInputHDMI2 => NamedKey::TVInputHDMI2,
            winit::keyboard::NamedKey::TVInputHDMI3 => NamedKey::TVInputHDMI3,
            winit::keyboard::NamedKey::TVInputHDMI4 => NamedKey::TVInputHDMI4,
            winit::keyboard::NamedKey::TVInputVGA1 => NamedKey::TVInputVGA1,
            winit::keyboard::NamedKey::TVMediaContext => NamedKey::TVMediaContext,
            winit::keyboard::NamedKey::TVNetwork => NamedKey::TVNetwork,
            winit::keyboard::NamedKey::TVNumberEntry => NamedKey::TVNumberEntry,
            winit::keyboard::NamedKey::TVPower => NamedKey::TVPower,
            winit::keyboard::NamedKey::TVRadioService => NamedKey::TVRadioService,
            winit::keyboard::NamedKey::TVSatellite => NamedKey::TVSatellite,
            winit::keyboard::NamedKey::TVSatelliteBS => NamedKey::TVSatelliteBS,
            winit::keyboard::NamedKey::TVSatelliteCS => NamedKey::TVSatelliteCS,
            winit::keyboard::NamedKey::TVSatelliteToggle => NamedKey::TVSatelliteToggle,
            winit::keyboard::NamedKey::TVTerrestrialAnalog => NamedKey::TVTerrestrialAnalog,
            winit::keyboard::NamedKey::TVTerrestrialDigital => NamedKey::TVTerrestrialDigital,
            winit::keyboard::NamedKey::TVTimer => NamedKey::TVTimer,
            winit::keyboard::NamedKey::AVRInput => NamedKey::AVRInput,
            winit::keyboard::NamedKey::AVRPower => NamedKey::AVRPower,
            winit::keyboard::NamedKey::ColorF0Red => NamedKey::ColorF0Red,
            winit::keyboard::NamedKey::ColorF1Green => NamedKey::ColorF1Green,
            winit::keyboard::NamedKey::ColorF2Yellow => NamedKey::ColorF2Yellow,
            winit::keyboard::NamedKey::ColorF3Blue => NamedKey::ColorF3Blue,
            winit::keyboard::NamedKey::ColorF4Grey => NamedKey::ColorF4Grey,
            winit::keyboard::NamedKey::ColorF5Brown => NamedKey::ColorF5Brown,
            winit::keyboard::NamedKey::ClosedCaptionToggle => NamedKey::ClosedCaptionToggle,
            winit::keyboard::NamedKey::Dimmer => NamedKey::Dimmer,
            winit::keyboard::NamedKey::DisplaySwap => NamedKey::DisplaySwap,
            winit::keyboard::NamedKey::DVR => NamedKey::DVR,
            winit::keyboard::NamedKey::Exit => NamedKey::Exit,
            winit::keyboard::NamedKey::FavoriteClear0 => NamedKey::FavoriteClear0,
            winit::keyboard::NamedKey::FavoriteClear1 => NamedKey::FavoriteClear1,
            winit::keyboard::NamedKey::FavoriteClear2 => NamedKey::FavoriteClear2,
            winit::keyboard::NamedKey::FavoriteClear3 => NamedKey::FavoriteClear3,
            winit::keyboard::NamedKey::FavoriteRecall0 => NamedKey::FavoriteRecall0,
            winit::keyboard::NamedKey::FavoriteRecall1 => NamedKey::FavoriteRecall1,
            winit::keyboard::NamedKey::FavoriteRecall2 => NamedKey::FavoriteRecall2,
            winit::keyboard::NamedKey::FavoriteRecall3 => NamedKey::FavoriteRecall3,
            winit::keyboard::NamedKey::FavoriteStore0 => NamedKey::FavoriteStore0,
            winit::keyboard::NamedKey::FavoriteStore1 => NamedKey::FavoriteStore1,
            winit::keyboard::NamedKey::FavoriteStore2 => NamedKey::FavoriteStore2,
            winit::keyboard::NamedKey::FavoriteStore3 => NamedKey::FavoriteStore3,
            winit::keyboard::NamedKey::Guide => NamedKey::Guide,
            winit::keyboard::NamedKey::GuideNextDay => NamedKey::GuideNextDay,
            winit::keyboard::NamedKey::GuidePreviousDay => NamedKey::GuidePreviousDay,
            winit::keyboard::NamedKey::Info => NamedKey::Info,
            winit::keyboard::NamedKey::InstantReplay => NamedKey::InstantReplay,
            winit::keyboard::NamedKey::Link => NamedKey::Link,
            winit::keyboard::NamedKey::ListProgram => NamedKey::ListProgram,
            winit::keyboard::NamedKey::LiveContent => NamedKey::LiveContent,
            winit::keyboard::NamedKey::Lock => NamedKey::Lock,
            winit::keyboard::NamedKey::MediaApps => NamedKey::MediaApps,
            winit::keyboard::NamedKey::MediaAudioTrack => NamedKey::MediaAudioTrack,
            winit::keyboard::NamedKey::MediaLast => NamedKey::MediaLast,
            winit::keyboard::NamedKey::MediaSkipBackward => NamedKey::MediaSkipBackward,
            winit::keyboard::NamedKey::MediaSkipForward => NamedKey::MediaSkipForward,
            winit::keyboard::NamedKey::MediaStepBackward => NamedKey::MediaStepBackward,
            winit::keyboard::NamedKey::MediaStepForward => NamedKey::MediaStepForward,
            winit::keyboard::NamedKey::MediaTopMenu => NamedKey::MediaTopMenu,
            winit::keyboard::NamedKey::NavigateIn => NamedKey::NavigateIn,
            winit::keyboard::NamedKey::NavigateNext => NamedKey::NavigateNext,
            winit::keyboard::NamedKey::NavigateOut => NamedKey::NavigateOut,
            winit::keyboard::NamedKey::NavigatePrevious => NamedKey::NavigatePrevious,
            winit::keyboard::NamedKey::NextFavoriteChannel => NamedKey::NextFavoriteChannel,
            winit::keyboard::NamedKey::NextUserProfile => NamedKey::NextUserProfile,
            winit::keyboard::NamedKey::OnDemand => NamedKey::OnDemand,
            winit::keyboard::NamedKey::Pairing => NamedKey::Pairing,
            winit::keyboard::NamedKey::PinPDown => NamedKey::PinPDown,
            winit::keyboard::NamedKey::PinPMove => NamedKey::PinPMove,
            winit::keyboard::NamedKey::PinPToggle => NamedKey::PinPToggle,
            winit::keyboard::NamedKey::PinPUp => NamedKey::PinPUp,
            winit::keyboard::NamedKey::PlaySpeedDown => NamedKey::PlaySpeedDown,
            winit::keyboard::NamedKey::PlaySpeedReset => NamedKey::PlaySpeedReset,
            winit::keyboard::NamedKey::PlaySpeedUp => NamedKey::PlaySpeedUp,
            winit::keyboard::NamedKey::RandomToggle => NamedKey::RandomToggle,
            winit::keyboard::NamedKey::RcLowBattery => NamedKey::RcLowBattery,
            winit::keyboard::NamedKey::RecordSpeedNext => NamedKey::RecordSpeedNext,
            winit::keyboard::NamedKey::RfBypass => NamedKey::RfBypass,
            winit::keyboard::NamedKey::ScanChannelsToggle => NamedKey::ScanChannelsToggle,
            winit::keyboard::NamedKey::ScreenModeNext => NamedKey::ScreenModeNext,
            winit::keyboard::NamedKey::Settings => NamedKey::Settings,
            winit::keyboard::NamedKey::SplitScreenToggle => NamedKey::SplitScreenToggle,
            winit::keyboard::NamedKey::STBInput => NamedKey::STBInput,
            winit::keyboard::NamedKey::STBPower => NamedKey::STBPower,
            winit::keyboard::NamedKey::Subtitle => NamedKey::Subtitle,
            winit::keyboard::NamedKey::Teletext => NamedKey::Teletext,
            winit::keyboard::NamedKey::VideoModeNext => NamedKey::VideoModeNext,
            winit::keyboard::NamedKey::Wink => NamedKey::Wink,
            winit::keyboard::NamedKey::ZoomToggle => NamedKey::ZoomToggle,
            winit::keyboard::NamedKey::F1 => NamedKey::F1,
            winit::keyboard::NamedKey::F2 => NamedKey::F2,
            winit::keyboard::NamedKey::F3 => NamedKey::F3,
            winit::keyboard::NamedKey::F4 => NamedKey::F4,
            winit::keyboard::NamedKey::F5 => NamedKey::F5,
            winit::keyboard::NamedKey::F6 => NamedKey::F6,
            winit::keyboard::NamedKey::F7 => NamedKey::F7,
            winit::keyboard::NamedKey::F8 => NamedKey::F8,
            winit::keyboard::NamedKey::F9 => NamedKey::F9,
            winit::keyboard::NamedKey::F10 => NamedKey::F10,
            winit::keyboard::NamedKey::F11 => NamedKey::F11,
            winit::keyboard::NamedKey::F12 => NamedKey::F12,
            winit::keyboard::NamedKey::F13 => NamedKey::F13,
            winit::keyboard::NamedKey::F14 => NamedKey::F14,
            winit::keyboard::NamedKey::F15 => NamedKey::F15,
            winit::keyboard::NamedKey::F16 => NamedKey::F16,
            winit::keyboard::NamedKey::F17 => NamedKey::F17,
            winit::keyboard::NamedKey::F18 => NamedKey::F18,
            winit::keyboard::NamedKey::F19 => NamedKey::F19,
            winit::keyboard::NamedKey::F20 => NamedKey::F20,
            winit::keyboard::NamedKey::F21 => NamedKey::F21,
            winit::keyboard::NamedKey::F22 => NamedKey::F22,
            winit::keyboard::NamedKey::F23 => NamedKey::F23,
            winit::keyboard::NamedKey::F24 => NamedKey::F24,
            winit::keyboard::NamedKey::F25 => NamedKey::F25,
            winit::keyboard::NamedKey::F26 => NamedKey::F26,
            winit::keyboard::NamedKey::F27 => NamedKey::F27,
            winit::keyboard::NamedKey::F28 => NamedKey::F28,
            winit::keyboard::NamedKey::F29 => NamedKey::F29,
            winit::keyboard::NamedKey::F30 => NamedKey::F30,
            winit::keyboard::NamedKey::F31 => NamedKey::F31,
            winit::keyboard::NamedKey::F32 => NamedKey::F32,
            winit::keyboard::NamedKey::F33 => NamedKey::F33,
            winit::keyboard::NamedKey::F34 => NamedKey::F34,
            winit::keyboard::NamedKey::F35 => NamedKey::F35,
            _ => 
            {
                if cfg!(debug_assertions) {
                    panic!("Unhandled winit KeyName : {:?}", value);
                } else {
                    NamedKey::Unknow
                }
            },
        }
    }
}

/// Key represents the meaning of a keypress.
///
/// This is a superset of the UI Events Specification's [`KeyboardEvent.key`] with
/// additions:
/// - All simple variants are wrapped under the `Named` variant
/// - The `Unidentified` variant here, can still identify a key through it's `NativeKeyCode`.
/// - The `Dead` variant here, can specify the character which is inserted when pressing the
///   dead-key twice.
///
/// [`KeyboardEvent.key`]: https://w3c.github.io/uievents-key/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key<Str = KeyText> {
    /// A simple (unparameterised) action
    Named(NamedKey),

    /// A key string that corresponds to the character typed by the user, taking into account the
    /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
    /// effect.
    Character(Str),

    /// This variant is used when the key cannot be translated to any other variant.
    ///
    /// The native key is provided (if available) in order to allow the user to specify keybindings
    /// for keys which are not defined by this API, mainly through some sort of UI.
    Unknow(NativeKey),

    /// Contains the text representation of the dead-key when available.
    ///
    /// ## Platform-specific
    /// - **Web:** Always contains `None`
    Dead(Option<char>),
}

impl<Str> From<WinitKey<Str>> for Key<Str>
{
    fn from(value: WinitKey<Str>) -> Self {
        match value
        {
            winit::keyboard::Key::Named(named_key) => Key::Named(named_key.into()),
            winit::keyboard::Key::Character(c) => Key::Character(c),
            winit::keyboard::Key::Unidentified(native_key) => Key::Unknow(native_key.into()),
            winit::keyboard::Key::Dead(d) => Key::Dead(d),
        }
    }
}

impl From<NamedKey> for Key {
    #[inline]
    fn from(action: NamedKey) -> Self {
        Key::Named(action)
    }
}

impl From<NativeKey> for Key {
    #[inline]
    fn from(code: NativeKey) -> Self {
        Key::Unknow(code)
    }
}

impl<Str> PartialEq<NamedKey> for Key<Str> {
    #[inline]
    fn eq(&self, rhs: &NamedKey) -> bool {
        match &self {
            Key::Named(a) => a == rhs,
            _ => false,
        }
    }
}


impl<Str: PartialEq<str>> PartialEq<str> for Key<Str> {
    #[inline]
    fn eq(&self, rhs: &str) -> bool {
        match &self {
            Key::Character(s) => s == rhs,
            _ => false,
        }
    }
}
impl<Str: PartialEq<str>> PartialEq<&str> for Key<Str> {
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self == *rhs
    }
}

impl<Str> PartialEq<NativeKey> for Key<Str> {
    #[inline]
    fn eq(&self, rhs: &NativeKey) -> bool {
        match &self {
            Key::Unknow(code) => code == rhs,
            _ => false,
        }
    }
}
impl<Str> PartialEq<Key<Str>> for NativeKey {
    #[inline]
    fn eq(&self, rhs: &Key<Str>) -> bool {
        rhs == self
    }
}

impl Key
{
    /// Convert `Key::Character(SmolStr)` to `Key::Character(&str)` so you can more easily match on
    /// `Key`. All other variants remain unchanged.
    pub fn as_ref(&self) -> Key<&str> {
        match self {
            Key::Named(a) => Key::Named(*a),
            Key::Character(ch) => Key::Character(ch.as_str()),
            Key::Dead(d) => Key::Dead(*d),
            Key::Unknow(u) => Key::Unknow(u.clone()),
        }
    }
}


impl NamedKey {
    /// Convert an action to its approximate textual equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// use winit::keyboard::NamedKey;
    ///
    /// assert_eq!(NamedKey::Enter.to_text(), Some("\r"));
    /// assert_eq!(NamedKey::F20.to_text(), None);
    /// ```
    pub fn to_text(&self) -> Option<&str> {
        match self {
            NamedKey::Enter => Some("\r"),
            NamedKey::Backspace => Some("\x08"),
            NamedKey::Tab => Some("\t"),
            NamedKey::Space => Some(" "),
            NamedKey::Escape => Some("\x1b"),
            _ => None,
        }
    }
}

#[bit_index]
/// Keyboard modifier keys, distinguishing between left/right sides and their composite states.
///
/// This enum represents the actual physical modifier keys pressed.
/// The variants can represent a specific side (e.g., `ShiftLeft`), or a **composite**
/// (e.g., `Shift`), which is a convenience flag meaning `ShiftLeft` or  `ShiftRight`.
#[repr(u8)]
pub enum KeyModifiers
{
    ShiftLeft,
    ShiftRight,
    Shift = Self::ShiftLeft | Self::ShiftRight,

    ControlLeft,
    ControlRight,
    Control = Self::ControlLeft | Self::ControlRight,

    AltLeft,
    AltRight,
    Alt = Self::AltLeft | Self::AltRight,

    SuperLeft,
    SuperRight,
    Super = Self::SuperLeft | Self::SuperRight,

    // Todo: add the other modifier ? https://en.wikipedia.org/wiki/Modifier_key
    // Fn, AltGr...
}

pub trait KeyModifiersExtension
{
    fn is_shift(&self) -> bool;
    fn is_control(&self) -> bool;
    fn is_super(&self) -> bool;
    fn is_alt(&self) -> bool;
}

impl KeyModifiersExtension for KeyModifiers
{
    fn is_shift(&self) -> bool { matches!(self, Self::ShiftLeft | Self::ShiftRight) }
    fn is_control(&self) -> bool { matches!(self, Self::ControlLeft | Self::ControlRight) }
    fn is_super(&self) -> bool { matches!(self, Self::SuperLeft | Self::SuperRight) }
    fn is_alt(&self) -> bool { matches!(self, Self::AltLeft | Self::AltRight) }
}

impl Default for KeyModifiersFlags
{
    fn default() -> Self {
        Self::EMPTY
    }
}

impl KeyModifiersFlags
{
    /// Checks if the actual modifier flags match the expected modifier pattern.
    ///
    /// Also handles both composite modifiers (e.g., `Shift`) and side-specific modifiers (e.g., `ShiftLeft`).
    pub fn matches(self, lexem: Self) -> bool 
    {
        fn check_mods(self_bits: KeyModifiersFlags, lexem_bits: KeyModifiersFlags, left: KeyModifiersFlags, right: KeyModifiersFlags, left_or_right: KeyModifiersFlags) -> bool {
            if self_bits.contains(left_or_right) {
                return lexem_bits.contains(left) || lexem_bits.contains(right);
            } else 
            {
                if self_bits.contains(left) && !lexem_bits.contains(left) {
                    return false;
                }
                if self_bits.contains(right) && !lexem_bits.contains(right) {
                    return false;
                }
                return true;
            }
        }
        
        check_mods(self, lexem, Self::ShiftLeft, Self::ShiftRight, Self::Shift) 
        && check_mods(self, lexem, Self::ControlLeft, Self::ControlRight, Self::Control) 
        && check_mods(self, lexem, Self::AltLeft, Self::AltRight, Self::Alt) 
        && check_mods(self, lexem, Self::SuperLeft, Self::SuperRight, Self::Super)
    }
}

impl KeyModifiersExtension for KeyModifiersFlags 
{ 
    fn is_shift(&self) -> bool { self.contains_any(Self::Shift) }
    fn is_control(&self) -> bool { self.contains_any(Self::Control) }
    fn is_super(&self) -> bool { self.contains_any(Self::Super) }
    fn is_alt(&self) -> bool { self.contains_any(Self::Alt) }
}

impl KeyModifiers
{
    pub fn from_keycode(key : KeyCode) -> Option<Self> { Self::try_from(key).ok() }
}

impl TryFrom<KeyCode> for KeyModifiers
{
    type Error = ();
    fn try_from(value: KeyCode) -> Result<KeyModifiers, Self::Error> {
        match value
        {
            KeyCode::ShiftLeft => Ok(Self::ShiftLeft),
            KeyCode::ShiftRight => Ok(Self::ShiftRight),
            KeyCode::ControlLeft => Ok(Self::ControlLeft),
            KeyCode::ControlRight => Ok(Self::ControlRight),
            KeyCode::AltLeft => Ok(Self::AltLeft),
            KeyCode::AltRight => Ok(Self::AltRight),
            KeyCode::SuperLeft => Ok(Self::SuperLeft),
            KeyCode::SuperRight => Ok(Self::SuperRight),
            _ => Err(()),
        }
    }
}

type KeyError = ();

impl TryFrom<NamedKey> for KeyModifiersFlags
{
    type Error = KeyError;
    fn try_from(value: NamedKey) -> Result<KeyModifiersFlags, Self::Error> {
        match value
        {
            NamedKey::Shift => Ok(Self::Shift),
            NamedKey::Alt => Ok(Self::Alt),
            NamedKey::Super => Ok(Self::Super),
            NamedKey::Control => Ok(Self::Control),
            _ => Err(()),
        }
    }
}

impl TryFrom<Key> for KeyModifiersFlags
{
    type Error = KeyError;
    fn try_from(value: Key) -> Result<KeyModifiersFlags, Self::Error> {
        match value
        {
            Key::Named(n) => n.try_into(),
            _ => Err(()),
        }
    }
}