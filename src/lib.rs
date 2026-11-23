//! Replay Studio: the replay editor, as a sandboxed Horizon mod.
//!
//! Recording, `.mcpr` files and playback stay in the game; the mod drives them through the replay
//! calls of `horizon-mod-api` and draws the editor on top (recordings browser, timeline, free
//! camera, camera paths). This first version only registers its key; the editor arrives in steps.

use mod_api::{Hooks, KeyConflictContext, KeyRegistration, RegistrationContext, Registrar};

/// The mod's id: its settings folder and its key's line in the game's key file.
pub const ID: &str = "replay_studio";
/// The key that will open the studio, its id within the mod.
pub const KEY_OPEN: &str = "open";
pub const DEFAULT_KEY_OPEN: &str = "key.keyboard.f8";

#[derive(Default)]
struct ReplayStudio;

#[mod_api::horizon_mod]
impl Hooks for ReplayStudio {
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        r.register_key_mapping(
            KeyRegistration::new(KEY_OPEN, DEFAULT_KEY_OPEN)
                .label("key.replay_studio.open")
                .context(KeyConflictContext::InGame),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_key_is_one_the_game_names() {
        assert!(DEFAULT_KEY_OPEN.starts_with("key.keyboard."));
        assert!(!ID.is_empty() && !KEY_OPEN.is_empty());
    }
}
