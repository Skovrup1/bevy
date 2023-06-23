use std::time::Duration;

use bevy::{prelude::*};
use bevy::audio::{AddAudioSource, Source};
use bevy::reflect::{TypeUuid, TypePath};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_audio_source::<SineAudio>()
        .add_systems(Startup, produce_sound)
        .run();
}

fn produce_sound(
    mut assets: ResMut<Assets<SineAudio>>, 
    audio: Res<Audio<SineAudio>>
) {
    let audio_handle = assets.add(SineAudio::new(440f32, 1f32));
    audio.play_with_settings(audio_handle);

}

//How to figure out if Audio has ended

struct Sequencer {
    sounds: Vec<Audio>, 
    duration_of_sound: Duration, 
}

impl Sequencer {
    fn play() {

    }

    fn stop() {

    }
}

// This struct usually contains the data for the audio being played.
// This is where data read from an audio file would be stored, for example.
// Implementing `TypeUuid` will automatically implement `Asset`.
// This allows the type to be registered as an asset.
#[derive(TypePath, TypeUuid)]
#[uuid = "c2090c23-78fd-44f1-8508-c89b1f3cec29"]
struct SineAudio {
    frequency: f32,
    duration: f32,
}

impl SineAudio {
    fn new(frequency: f32, duration: f32) -> SineAudio {
        SineAudio { frequency, duration }
    }
}

// This decoder is responsible for playing the audio,
// and so stores data about the audio being played.
struct SineDecoder {
    // how far along one period the wave is (between 0 and 1)
    current_progress: f32,
    // how much we move along the period every frame
    progress_per_frame: f32,
    // how long to play the audio for
    duration: f32,
    // how long a period is
    period: f32,
    sample_rate: u32,
}

impl SineDecoder {
    fn new(frequency: f32, duration: f32) -> Self {
        // standard sample rate for most recordings
        let sample_rate = 48*10u32.pow(3);
        SineDecoder {
            current_progress: 0.,
            progress_per_frame: frequency / sample_rate as f32,
            duration,
            period: std::f32::consts::PI * 2.,
            sample_rate,
        }
    }
}

// The decoder must implement iterator so that it can implement `Decodable`.
impl Iterator for SineDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.duration -= 1.0/self.sample_rate as f32;
        self.current_progress += self.progress_per_frame;
        // we loop back round to 0 to avoid floating point inaccuracies
        self.current_progress %= 1.;

        if self.duration > 0.0 {
            Some(f32::sin(self.period * self.current_progress))
        } else {
            None
        }
    }
}
// `Source` is what allows the audio source to be played by bevy.
// This trait provides information on the audio.
impl Source for SineDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(self.duration))
    }
}

// Finally `Decodable` can be implemented for our `SineAudio`.
impl Decodable for SineAudio {
    type Decoder = SineDecoder;

    type DecoderItem = <SineDecoder as Iterator>::Item;

    fn decoder(&self) -> Self::Decoder {
        SineDecoder::new(self.frequency, self.duration)
    }
}