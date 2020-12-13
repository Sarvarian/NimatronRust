use bevy::prelude::*;
use gdnative::godot_print;
use std::sync::{self, mpsc, Mutex};

pub fn bevy(receiver: mpsc::Receiver<G2BMessage>) {
    godot_print!("Bevy: Run!");
    App::build()
        .add_plugins(MinimalPlugins)
        .add_resource(Terminal {
            receiver: Mutex::new(receiver),
        })
        .add_system(receive_handler.system())
        .run();
}

pub enum G2BMessage {
    Quite,
}

struct Terminal {
    receiver: Mutex<mpsc::Receiver<G2BMessage>>,
}

fn receive_handler(
    terminal: Res<Terminal>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    loop {
        let msg = match terminal.receiver.try_lock() {
            Ok(receiver) => match receiver.try_recv() {
                Ok(msg) => Some(msg),
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => None,
                    mpsc::TryRecvError::Disconnected => None,
                },
            },
            Err(err) => match err {
                sync::TryLockError::WouldBlock => None,
                sync::TryLockError::Poisoned(_) => None,
            },
        };
        if let Some(msg) = msg {
            match msg {
                G2BMessage::Quite => {
                    godot_print!("Bevy Terminal: Quite Message Reccived!");
                    app_exit_events.send(bevy::app::AppExit);
                }
            }
        } else {
            break;
        }
    }
}
