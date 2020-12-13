use bevy::prelude::*;
use gdnative::godot_print;
use std::sync::{mpsc, Mutex};

pub enum G2BMessage {
    Quite,
}

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(receive_handler.system());
    }
}

pub struct Terminal {
    pub receiver: Mutex<mpsc::Receiver<G2BMessage>>,
}

fn receive_handler(terminal: Res<Terminal>) {
    if let Ok(receiver) = terminal.receiver.try_lock() {
        for msg in receiver.try_iter() {
            match msg {
                G2BMessage::Quite => godot_print!("Bevy Terminal: Quite Message Reccived!"),
            }
        }
    }
}
