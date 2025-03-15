use heapless::Vec;

use crate::{action::KeyAction};

// Max number of fork behaviors
pub(crate) const FORK_MAX_NUM: usize = 16;
// Max number of fork state conditions
pub(crate) const FORK_MAX_CONDITION_LENGTH: usize = 8 + 3;//8 modifiers + 3 leds

#[derive(Clone, Debug)]
pub struct Fork {
    pub(crate) trigger: KeyAction,    
    pub(crate) none_output: KeyAction,
    pub(crate) any_output: KeyAction,
    pub(crate) conditions: Vec<KeyAction, FORK_MAX_CONDITION_LENGTH>
}

impl Default for Fork {
    fn default() -> Self {
        Self::empty()
    }
}

impl Fork {
    pub fn new<I: IntoIterator<Item = KeyAction>>(
        trigger: KeyAction,    
        none_output: KeyAction,
        any_output: KeyAction,
        conditions: I
    ) -> Self {
        Self {
            trigger,    
            none_output,
            any_output,
            conditions: Vec::from_iter(conditions),
        }
    }

    pub fn empty() -> Self {
        Self::new(
            KeyAction::No,
            KeyAction::No,
            KeyAction::No,
            Vec::<KeyAction, FORK_MAX_CONDITION_LENGTH>::new()
        )
    }
}
