use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use winit::event::{ElementState, KeyboardInput, ModifiersState};

mod key {
    #[derive(Debug, Clone, Copy)]
    pub struct State {
        pub held_down: bool,
        pub this_frame: bool,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                held_down: false,
                this_frame: true,
            }
        }
    }
}

pub struct KeyBoardState {
    modifier_state: ModifiersState,
    keys: [key::State; 163],
}

impl KeyBoardState {
    pub fn modifier_state(&self) -> &ModifiersState {
        &self.modifier_state
    }

    pub fn update_modifier_state(&mut self, state: ModifiersState) {
        self.modifier_state = state;
    }
    pub fn update_keys(&mut self, input: KeyboardInput) {
        self.keys[input.virtual_keycode.unwrap() as usize].held_down =
            input.state == ElementState::Pressed;
        self.keys[input.virtual_keycode.unwrap() as usize].this_frame = true;
    }
    pub fn downgrade_keys(&mut self) {
        self.keys.par_iter_mut().for_each(|state| {
            state.this_frame = false;
        })
    }
}

impl Default for KeyBoardState {
    fn default() -> Self {
        Self {
            modifier_state: Default::default(),
            keys: [key::State::default(); 163],
        }
    }
}
