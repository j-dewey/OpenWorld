use winit::event::{WindowEvent, VirtualKeyCode, KeyboardInput, ElementState};
use hashbrown::HashMap;

// returns the keyboard keys that i feel like being allowed
fn get_suppoerted_keys() -> Vec<VirtualKeyCode>{
    vec![
        // letters
        VirtualKeyCode::A,
        VirtualKeyCode::B,
        VirtualKeyCode::C,
        VirtualKeyCode::D,
        VirtualKeyCode::E,
        VirtualKeyCode::F,
        VirtualKeyCode::G,
        VirtualKeyCode::H,
        VirtualKeyCode::I,
        VirtualKeyCode::J,
        VirtualKeyCode::K,
        VirtualKeyCode::L,
        VirtualKeyCode::M,
        VirtualKeyCode::N,
        VirtualKeyCode::O,
        VirtualKeyCode::P,
        VirtualKeyCode::Q,
        VirtualKeyCode::R,
        VirtualKeyCode::S,
        VirtualKeyCode::T,
        VirtualKeyCode::U,
        VirtualKeyCode::V,
        VirtualKeyCode::W,
        VirtualKeyCode::X,
        VirtualKeyCode::Y,
        VirtualKeyCode::Z,
        // arrow keys
        VirtualKeyCode::Up,
        VirtualKeyCode::Down,
        VirtualKeyCode::Left,
        VirtualKeyCode::Right,
        VirtualKeyCode::Space
    ]
}

pub struct InputHandler{
    // keyboard states
    key_states: HashMap<VirtualKeyCode, bool>,
    // match a certain event such as up wih a specific key
    key_events: HashMap<String, VirtualKeyCode>,
    new_pressed: Vec<VirtualKeyCode>
}

impl InputHandler{
    pub fn new() -> Self{
        // set up keys
        let supported_keys = get_suppoerted_keys();
        let mut key_states: HashMap<VirtualKeyCode, bool> = HashMap::new();
        for key in supported_keys{
            // just set every key as being unpressed to start
            key_states.insert(key, false);
        }

        // default key events
        let mut key_events: HashMap<String, VirtualKeyCode> = HashMap::new();
        key_events.insert("forward".into(), VirtualKeyCode::W);
        key_events.insert("backward".into(), VirtualKeyCode::S);
        key_events.insert("strafe-left".into(), VirtualKeyCode::A);
        key_events.insert("strafe-right".into(), VirtualKeyCode::D);
        key_events.insert("rotate-right".into(), VirtualKeyCode::Right);
        key_events.insert("rotate-left".into(), VirtualKeyCode::Left);
        key_events.insert("rotate-up".into(), VirtualKeyCode::Up);
        key_events.insert("rotate-down".into(), VirtualKeyCode::Down);
        key_events.insert("down".into(), VirtualKeyCode::X);
        key_events.insert("up".into(), VirtualKeyCode::Z);
        key_events.insert("toggle-physics".into(), VirtualKeyCode::Q);
        key_events.insert("toggle-debug".into(), VirtualKeyCode::L);

        Self { 
            key_states: key_states,
            key_events: key_events,
            new_pressed: Vec::new()
         }
    }

    pub fn handle_input(&mut self, event: &WindowEvent ) -> bool{
        // each WindowEvent is passed through here
        // returns false if the event doesn't change anything
        match event{
            WindowEvent::KeyboardInput { input: KeyboardInput{ state, virtual_keycode: Some(virtual_keycode), .. }, ..  } => {
                // set the key in keystates to reflect whether it is pressed or not
                if *state == ElementState::Pressed && !self.key_states.get(virtual_keycode).unwrap(){
                    self.new_pressed.push(*virtual_keycode)
                }
                self.key_states.insert(*virtual_keycode, *state == ElementState::Pressed);
                true
            },
            _ => { false }
        }
    }

    pub fn get_key_event(&self, event: String) -> bool{
        // needs to be derefrenced since get() returns a borrow
        *self.key_states.get(
            self.key_events.get(&event).unwrap()
        ).unwrap()
    }

    pub fn get_events(&self, events: Vec<String>) -> Vec<bool>{
        let mut event_states: Vec<bool> = Vec::new();
        for event in events{
            event_states.push(
                *self.key_states.get(
                    self.key_events.get(&event).unwrap()
                ).unwrap()
            );
        }
        event_states
    }

    pub fn check_new_event(&self, event: String) -> bool{
        let key = self.key_events.get(&event).unwrap();
        self.new_pressed.contains(key)
    }

    pub fn flush_new_presses(&mut self){
        self.new_pressed.clear()
    }
}