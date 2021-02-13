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

/// TODO document how this helps get audio workinging in wasm
/// builds in chrome.
///
/// ```edition2018
/// # use bevy_kira_audio::AudioInitialization;
/// app.insert_resource(AudioInitialization {
///     needed: false,
///     ..Default::default()
/// })
/// ```
///
/// app.add_plugins(AudioPlugin)
/// ```edition2018
/// commands.insert_resource(AudioInitialization {
///     needed: true,
///     ..Default::default()
/// })
pub struct AudioInitialization {
    /// Should the audio manager be initialized at the next opportunity?
    pub needed: bool,
    /// Has the audio manager already been initialized?
    pub done: bool,
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
            stage::POST_UPDATE,
            play_queued_audio_system.exclusive_system(),
        );
        app.init_resource::<AudioInitialization>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                initialize_audio_system.exclusive_system(),
            );
    }
}
