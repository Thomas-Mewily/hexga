use std::collections::HashSet;

use super::*;

impl From<KeyBinding> for Binding
{
    fn from(value: KeyBinding) -> Self {
        Self::Key(value)
    }
}
impl From<KeyCode> for Binding
{
    fn from(value: KeyCode) -> Self {
        Self::Key(value.into())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum Binding
{
    Key(KeyBinding),
    Or(Vec<Binding>),
    And(Vec<Binding>),
    Not(Box<Binding>),
    #[default]
    None,
}

impl Binding
{
    pub fn alt() -> Self { Self::from(KeyCode::AltLeft).or(KeyCode::AltRight) }
    pub fn shift() -> Self { Self::from(KeyCode::ShiftLeft).or(KeyCode::ShiftRight) }
    pub fn control() -> Self { Self::from(KeyCode::ControlLeft).or(KeyCode::ControlRight) }
    pub fn meta() -> Self { Self::from(KeyCode::SuperLeft).or(KeyCode::SuperRight) }
}

impl Binding
{
    pub fn is_none(&self) -> bool
    {
        match self
        {
            Binding::Or(v) => v.is_empty(),
            Binding::And(v) => v.is_empty(),
            Binding::None => true,
            _ => false
        }
    }

    pub fn and(self, other: impl Into<Binding>) -> Self
    {
        let other = other.into();

        if self.is_none() { return other; }
        if other.is_none() { return self; }

        fn flatten_and(binding: Binding, out: &mut Vec<Binding>) {
            if let Binding::And(inner) = binding {
                for b in inner {
                    flatten_and(b, out);
                }
            } else {
                out.push(binding);
            }
        }

        let mut items = Vec::new();
        flatten_and(self, &mut items);
        flatten_and(other, &mut items);

        // Remove duplicates while preserving order
        let mut seen = HashSet::new();
        items.retain(|b| seen.insert(b.clone()));

        Binding::And(items)
    }

    pub fn or(self, other: impl Into<Binding>) -> Self
    {
        let other = other.into();

        if self.is_none() { return other; }
        if other.is_none() { return self; }

        fn flatten_or(binding: Binding, out: &mut Vec<Binding>) {
            if let Binding::Or(inner) = binding {
                for b in inner {
                    flatten_or(b, out);
                }
            } else {
                out.push(binding);
            }
        }

        let mut items = Vec::new();
        flatten_or(self, &mut items);
        flatten_or(other, &mut items);

        // Remove duplicates while preserving order
        let mut seen = HashSet::new();
        items.retain(|b| seen.insert(b.clone()));

        Binding::Or(items)
    }


    pub fn not(self) -> Self
    {
        if let Self::Not(n) = self
        {
            return *n;
        }
        Self::Not(Box::new(self))
    }
}

impl<B> BitAnd<B> for Binding where B:Into<Self>
{
    type Output=Self;
    fn bitand(self, rhs: B) -> Self::Output { self.and(rhs) }
}
impl<B> BitAndAssign<B> for Binding where B:Into<Self>
{
    fn bitand_assign(&mut self, rhs: B)
    {
        let mut empty = Binding::None;
        std::mem::swap(&mut empty, self);
        *self = empty.and(rhs);
    }
}

impl<B> BitOr<B> for Binding where B:Into<Self>
{
    type Output=Self;
    fn bitor(self, rhs: B) -> Self::Output { self.or(rhs) }
}
impl<B> BitOrAssign<B> for Binding where B:Into<Self>
{
    fn bitor_assign(&mut self, rhs: B)
    {
        let mut empty = Binding::None;
        std::mem::swap(&mut empty, self);
        *self = empty.or(rhs);
    }
}

impl Not for Binding
{
    type Output=Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}


pub trait Bindable
{
    fn is_requested(&self) -> bool;
}
impl Bindable for Binding
{
    fn is_requested(&self) -> bool
    {
        match self
        {
            Binding::Key(v) => v.is_requested(),
            Binding::Or(v) => v.iter().any(|v| v.is_requested()),
            Binding::And(v) => v.iter().all(|v| v.is_requested()),
            Binding::Not(v) => !v.is_requested(),
            Binding::None => false,
        }
    }
}





// TODO: Make trait Getter/Setter for Toggle, State, Repeat..
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct KeyBinding
{
    pub code  : KeyCode,
    pub repeat: ButtonRepeat,
    pub state : ButtonState,
    pub toggle: ButtonToggle,
}
impl KeyBinding
{
    pub const fn new(code : KeyCode) -> Self { Self { code, repeat: ButtonRepeat::NotRepeated, state: ButtonState::Down, toggle: ButtonToggle::Hold }}
    pub const fn with_code(mut self, code : KeyCode) -> Self { self.code = code; self }
    pub const fn with_repeat(mut self, repeat : ButtonRepeat) -> Self { self.repeat = repeat; self }
    pub const fn with_state(mut self, state : ButtonState) -> Self { self.state = state; self }
    pub const fn with_toggle(mut self, toggle : ButtonToggle) -> Self { self.toggle = toggle; self }
}
impl From<KeyCode> for KeyBinding
{
    fn from(value: KeyCode) -> Self { Self::new(value) }
}


impl KeyBinding
{
    pub(crate) fn evolution(&self) -> ButtonEvolution
    {
        Input.keyboard().key_manager_mut(self.repeat).evolution(self.code)
    }
}
impl Bindable for KeyBinding
{
    fn is_requested(&self) -> bool
    {
        let evo = self.evolution();
        let hold = evo.value() == self.state;
        match self.toggle
        {
            ButtonToggle::Hold => hold,
            ButtonToggle::Toggle => hold && evo.value() != evo.old_value(),
        }
    }
}

/*
impl IEvolution<bool> for KeyBinding
{
    fn value(&self) -> bool
    {
        match self.toggle
        {
            ButtonToggle::Hold => self.evolution().value() == self.state,
            ButtonToggle::Toggle => (self.evolution().value() == self.state) == ,
        }
    }
    fn old_value(&self) -> bool { self.evolution().old_value() == self.state }
}*/
/*
impl IUsedFlag for KeyBinding
{
    fn is_used(&self) -> bool { Input.keyboard.key_manager_mut(self.repeat).is_key_used(self.code) }
    fn set_used(&mut self, used: bool) -> &mut Self { Input.keyboard.key_manager_mut(self.repeat).set_key_used(self.code, used); self }
}
*/





#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct BindingDpad
{
    pub up: Binding,
    pub down: Binding,
    pub left: Binding,
    pub right: Binding,
}

impl Default for BindingDpad
{
    fn default() -> Self
    {
        Self
        {
            up: Binding::from(KeyCode::Up).or(KeyCode::W),
            down: Binding::from(KeyCode::Down).or(KeyCode::S),
            left: Binding::from(KeyCode::Left).or(KeyCode::A),
            right: Binding::from(KeyCode::Right).or(KeyCode::D)
        }
    }
}