use sfml::window::Key;
use std::sync::mpsc::{Receiver, Sender};

pub struct Input {
    current_input: char,
}

impl Input {
    pub fn new() -> Input {
        Input { current_input: ' ' }
    }

    fn get_keyboard_input(&mut self) {
        loop {
            if Key::Left.is_pressed() {
                self.current_input = '4';
                break;
            } else if Key::Right.is_pressed() {
                self.current_input = '6';
                break;
            } else if Key::Down.is_pressed() {
                self.current_input = '2';
                break;
            } else if Key::Space.is_pressed() {
                self.current_input = ' ';
                break;
            } else if Key::Escape.is_pressed() {
                self.current_input = '\n';
                break;
            }
        }
    }

    // if game sends read(false) than listener will be chcecking for input.
    // if input is given it sends it to game, and blocks input checking
    // until sended input is read by the game (game sends read(true))
    // if game sends game_off(true) loop will terminate
    pub fn input_listener_activate(
        &mut self,
        input_tx: Sender<char>,
        input_read_rx: Receiver<bool>,
        game_off_rx: Receiver<bool>,
    ) {
        let mut sended = false;
        'listener_loop: loop {
            match game_off_rx.try_recv() {
                Ok(off) => {
                    if off {
                        break 'listener_loop;
                    }
                }
                _ => (),
            }
            match input_read_rx.recv() {
                Ok(read) => {
                    if read && !sended {
                        self.get_keyboard_input();
                        input_tx.send(self.current_input).unwrap();
                        if self.current_input == '\n' {
                            break 'listener_loop;
                        }
                        sended = true;
                    } else if !read {
                        sended = false;
                    }
                }
                _ => (),
            }
        }
    }
}
