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

impl<E> ScopedMessage<E> for Keyboard where E:IEvent
{
    fn begin_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) {
        ScopedMessage::<E>::begin_flow(&mut self.key, flow,el);
        ScopedMessage::<E>::begin_flow(&mut self.key_repeated, flow,el);
    }

    fn end_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) {
        ScopedMessage::<E>::end_flow(&mut self.key, flow,el);
        ScopedMessage::<E>::end_flow(&mut self.key_repeated, flow,el);
    }

    fn begin_input(&mut self, input: &InputEvent, el: &EventLoopActive) {
        ScopedMessage::<E>::begin_input(&mut self.key, input,el);
        ScopedMessage::<E>::begin_input(&mut self.key_repeated, input,el);
    }
    fn end_input(&mut self, el: &EventLoopActive) {
        ScopedMessage::<E>::end_input(&mut self.key,el);
        ScopedMessage::<E>::end_input(&mut self.key_repeated,el);
    }
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



impl<E> ScopedMessage<E> for KeyCodeManager where E:IEvent
{
    fn begin_flow_paused(&mut self, _el: &EventLoopActive) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }

    fn begin_flow_resumed(&mut self, _el: &EventLoopActive) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear(); 
    }

    fn end_flow_update(&mut self, _el: &EventLoopActive) {
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

    fn begin_input_key(&mut self, key: &KeyEvent, _el: &EventLoopActive) 
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