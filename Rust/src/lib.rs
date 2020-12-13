use gdnative::prelude::*;
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
mod gbevy;
use gbevy::G2BMessage;

fn init(handle: InitHandle) {
    handle.add_class::<Game>();
}

godot_init!(init);

#[derive(NativeClass)]
#[inherit(Node)]
struct Game {
    handle: Option<JoinHandle<()>>,
    sender: Option<mpsc::Sender<G2BMessage>>,
}

#[methods]
impl Game {
    fn new(_owner: &Node) -> Self {
        Game {
            handle: None,
            sender: None,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("Game: Hello!");
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        self.handle = Some(thread::spawn(move || {
            gbevy::bevy(rx);
        }));
    }

    #[export]
    fn _exit_tree(&mut self, _owner: &Node) {
        if let Some(sender) = &self.sender {
            sender.send(G2BMessage::Quite).unwrap_or(());
        }
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap_or(());
        }
    }
}
