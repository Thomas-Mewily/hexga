use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Keyboard
{
    pub(crate) key : KeyCodeManager,
    pub(crate) key_repeated : KeyCodeManager,
}

impl Default for Keyboard
{
    fn default() -> Self {
        Self 
        { 
            key: KeyCodeManager::new(ButtonRepeat::NotRepeated), 
            key_repeated: KeyCodeManager::new(ButtonRepeat::Repeated) 
        }
    }
}

impl<E> ScopedEvent<E> for Keyboard where E:IEvent
{
    /* 
    fn begin_update(&mut self) 
    {
        ScopedEvent::<E>::begin_update(&mut self.key);
        ScopedEvent::<E>::begin_update(&mut self.key_repeated);
    }

    fn end_update(&mut self) {
        ScopedEvent::<E>::begin_update(&mut self.key.end_update();
        ScopedEvent::<E>::begin_update(&mut self.key_repeated.end_update();
    }

    fn begin_paused(&mut self) {
        ScopedEvent::<E>::begin_update(&mut self.key.begin_paused();
    }
    fn begin_resumed(&mut self) {
        ScopedEvent::<E>::begin_update(&mut self.key.begin_resumed();
    }

    fn begin_input_key(&mut self, key: &KeyEvent) {
        ScopedEvent::<E>::begin_update(&mut self.key.begin_input_key(key);
        ScopedEvent::<E>::begin_update(&mut self.key_repeated.begin_input_key(key);
    }
    */
}


pub trait IKeyboard
{
    fn keys(&self) -> &KeyCodeManager { self.key_manager(ButtonRepeat::NotRepeated) }
    fn keys_mut(&mut self) -> &mut KeyCodeManager { self.key_manager_mut(ButtonRepeat::NotRepeated) }

    fn keys_repeated(&self) -> &KeyCodeManager  { self.key_manager(ButtonRepeat::Repeated) }
    fn keys_repeated_mut(&mut self) -> &mut KeyCodeManager  { self.key_manager_mut(ButtonRepeat::Repeated) }

    fn key_manager(&self, repeat: ButtonRepeat) -> &KeyCodeManager;
    fn key_manager_mut(&mut self, repeat: ButtonRepeat) -> &mut KeyCodeManager;
}
impl<T> IKeyboard for T where T: HasRef<Keyboard> + HasMut<Keyboard>
{
    fn key_manager(&self, repeat: ButtonRepeat) -> &KeyCodeManager 
    {
        let s = self.retrive();
        if repeat.is_repeated() { &s.key_repeated } else { &s.key }
    }

    fn key_manager_mut(&mut self, repeat: ButtonRepeat) -> &mut KeyCodeManager 
    {
        let s = self.retrive_mut();
        if repeat.is_repeated() { &mut s.key_repeated } else { &mut s.key }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct KeyCodeManager
{
    pub(crate) repeat  : ButtonRepeat,
    pub(crate) down    : HashSet<KeyCode>,
    pub(crate) old_down: HashSet<KeyCode>,
    pub(crate) pressed : HashSet<KeyCode>,
    pub(crate) released: HashSet<KeyCode>,
}

impl KeyCodeManager
{
    pub fn new(repeat : ButtonRepeat) -> Self { Self { repeat, down: ___(), old_down: ___(), pressed: ___(), released: ___() }}
}
impl<E> ScopedEvent<E> for KeyCodeManager where E:IEvent
{
    fn begin_paused(&mut self) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }

    fn begin_resumed(&mut self) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear(); 
    }

    fn end_update(&mut self) {
        match self.repeat
        {
            ButtonRepeat::NotRepeated => 
            {
                self.pressed.clear();
                self.released.clear();
                self.old_down.clone_from(&self.down);
            },
            ButtonRepeat::Repeated => 
            {
                std::mem::swap(&mut self.pressed, &mut self.released);
                self.pressed.clear();
                std::mem::swap(&mut self.old_down, &mut self.down);
                self.down.clear();
            },
        }
    }

    fn begin_input_key(&mut self, key: &KeyEvent) 
    {
        let code = key.code;
        match key.state
        {
            ButtonState::Up => 
            {
                self.down.remove(&code);
                self.released.insert(code);
            },
            ButtonState::Down =>
            {
                if !self.down.contains(&code)
                {
                    self.pressed.insert(code);
                }
                self.down.insert(code);
            }
        }
    }
}