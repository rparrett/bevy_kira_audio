//! # Bevy Kira audio
//!
//! This crate is an audio plugin for the game engine Bevy. It uses the library
//! Kira to play audio and offers an API to control running audio.
//!
//! See the repository <https://github.com/NiklasEi/bevy_kira_audio/> for additional
//! documentation and usage examples.
//! ```edition2018
//! # use bevy_kira_audio::{AudioChannel, Audio, AudioPlugin};
//! # use bevy::prelude::*;
//! fn main() {
//!    let mut app = App::build();
//!    app
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(AudioPlugin)
//!         .add_startup_system(start_background_audio.system());
//!    app.run();
//! }
//!
//! fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//!     audio.play_looped(asset_server.load("background_audio.mp3"));
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs, unused_imports)]

use bevy::prelude::*;

pub use audio::Audio;
pub use source::AudioSource;

mod audio;
mod audio_output;
mod channel;
mod source;

use crate::audio_output::{initialize_audio_system, play_queued_audio_system};

pub use channel::AudioChannel;

#[cfg(feature = "flac")]
use crate::source::FlacLoader;
#[cfg(feature = "mp3")]
use crate::source::Mp3Loader;
#[cfg(feature = "ogg")]
use crate::source::OggLoader;
#[cfg(feature = "wav")]
use crate::source::WavLoader;

/// A Bevy plugin to add audio functionallity
///
/// Add this plugin to your Bevy app to get access to
/// the Audio resource
/// ```edition2018
/// # use bevy_kira_audio::{AudioChannel, Audio, AudioPlugin};
/// # use bevy::prelude::*;
/// fn main() {
///    let mut app = App::build();
///    app
///         .add_plugins(DefaultPlugins)
///         .add_plugin(AudioPlugin)
///         .add_startup_system(start_background_audio.system());
///    app.run();
/// }
///
/// fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
///     audio.play_looped(asset_server.load("background_audio.mp3"));
/// }
/// ```
#[derive(Default)]
pub struct AudioPlugin;

/// Initialization state of the audio manager
///
/// Inserting this resource with `needed: true` before adding `AudioPlugin` will
/// result in the audio manager not getting initialized.
///
/// ```edition2018
/// use bevy_kira_audio::AudioInitialization;
///
/// app.insert_resource(AudioInitialization::new(false));
/// app.add_plugin(AudioPlugin)
/// ```
///
/// It is then possible to overwrite this resource at at some other time, which
/// will cause the initialization to occur.
///
/// ```edition2018
/// use bevy_kira_audio::AudioInitialization;
///
/// commands.insert_resource(AudioInitialization::new(true));
/// ```
///
/// This is generally not needed, but could be useful when targetting wasm. Google
/// Chrome requires users to have interacted with the page before sound can be played,
/// and initializing after an in-game button press is one strategy for dealing with
/// that.
pub struct AudioInitialization {
    /// Should the audio manager be initialized at the next opportunity?
    pub needed: bool,
    /// Has the audio manager already been initialized?
    done: bool,
}
impl AudioInitialization {
    /// Creates a new initialization state
    pub fn new(needed: bool) -> Self {
        AudioInitialization {
            needed,
            done: false,
        }
    }
}
impl Default for AudioInitialization {
    fn default() -> Self {
        AudioInitialization {
            needed: true,
            done: false,
        }
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<AudioSource>();

        #[cfg(feature = "mp3")]
        app.init_asset_loader::<Mp3Loader>();
        #[cfg(feature = "ogg")]
        app.init_asset_loader::<OggLoader>();
        #[cfg(feature = "wav")]
        app.init_asset_loader::<WavLoader>();
        #[cfg(feature = "flac")]
        app.init_asset_loader::<FlacLoader>();

        app.init_resource::<Audio>().add_system_to_stage(
            CoreStage::PostUpdate,
            play_queued_audio_system.exclusive_system(),
        );
        app.init_resource::<AudioInitialization>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                initialize_audio_system.exclusive_system(),
            );
    }
}
