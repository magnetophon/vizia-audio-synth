use vizia::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {

    Application::new(|cx|{
        // UI will go here
    })
        .title("Audio Synth")
        .inner_size((200, 120))
        .run();
}
