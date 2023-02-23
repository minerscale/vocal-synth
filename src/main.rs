mod synth;

use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::Source;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = synth::VocalSynth::new(440.0).take_duration(Duration::from_secs_f32(30.0));
    sink.append(source);
    sink.sleep_until_end();
}
