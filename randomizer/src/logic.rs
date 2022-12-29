use crate::logic_mode::LogicMode::*;
use crate::progress::Progress;
use serde::{Deserialize, Serialize, Serializer};

// TODO I'd eventually like to externalize the logic, both for organization purposes and to allow users to write custom logic. But this is fine for now.

pub fn serialize_foo_option<S>(
    maybe_foo: &Option<fn(&Progress) -> bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("string")
}

#[derive(Copy, Clone, Serialize)]
pub struct Logic {

    #[serde(serialize_with = "serialize_foo_option")]
    pub normal: Option<fn(&Progress) -> bool>,
    #[serde(serialize_with = "serialize_foo_option")]
    pub hard: Option<fn(&Progress) -> bool>,
    #[serde(serialize_with = "serialize_foo_option")]
    pub glitch_basic: Option<fn(&Progress) -> bool>,
    #[serde(serialize_with = "serialize_foo_option")]
    pub glitch_advanced: Option<fn(&Progress) -> bool>,
    #[serde(serialize_with = "serialize_foo_option")]
    pub glitch_hell: Option<fn(&Progress) -> bool>,
}

impl Logic {
    pub fn new(normal: Option<fn(&Progress) -> bool>,
               hard: Option<fn(&Progress) -> bool>,
               glitch_basic: Option<fn(&Progress) -> bool>,
               glitch_advanced: Option<fn(&Progress) -> bool>,
               glitch_hell: Option<fn(&Progress) -> bool>) -> Self {
        Self {
            normal,
            hard,
            glitch_basic,
            glitch_advanced,
            glitch_hell,
        }
    }

    pub fn can_access(self, progress: &Progress) -> bool {

        // Progression is available if the current logic or a lower tiered logic passes
        for logic in match progress.get_settings().logic.mode {
            Normal => Vec::from([self.normal]),
            Hard => Vec::from([self.normal, self.hard]),
            GlitchBasic => Vec::from([self.normal, self.hard, self.glitch_basic]),
            GlitchAdvanced => Vec::from([self.normal, self.hard, self.glitch_basic, self.glitch_advanced]),
            GlitchHell => Vec::from([self.normal, self.hard, self.glitch_basic, self.glitch_advanced, self.glitch_hell]),
            NoLogic => { return true; }
        } {
            if logic.is_some() && (logic.unwrap())(progress) {
                return true;
            }
        }

        false
    }

    pub fn free() -> Self {
        Self {
            normal: accessible(),
            hard: accessible(),
            glitch_basic: accessible(),
            glitch_advanced: accessible(),
            glitch_hell: accessible(),
        }
    }
}

pub fn accessible() -> Option<fn(&Progress) -> bool> {
    Some(|_| true)
}