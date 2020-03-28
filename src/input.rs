use sfml::window::Key;
use std::sync::mpsc::{Receiver, Sender};

pub struct Input {
    current_input: Option<char>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            current_input: None,
        }
    }

    fn get_keyboard_input(&mut self) {
        let inp: char;
        loop {
            if Key::Left.is_pressed() {
                inp = '4';
                break;
            } else if Key::Right.is_pressed() {
                inp = '6';
                break;
            } else if Key::Down.is_pressed() {
                inp = '2';
                break;
            } else if Key::Space.is_pressed() {
                inp = ' ';
                break;
            } else if Key::Escape.is_pressed() {
                inp = '\n';
                break;
            }
        }
        if inp == '\0' {
            self.current_input = None;
        } else {
            self.current_input = Some(inp);
        }
    }

    pub fn input_listener_activate(
        &mut self,
        input_tx: Sender<Option<char>>,
        input_read_rx: Receiver<bool>,
        game_off_rx: Receiver<bool>,
    ) {
        let mut sended = false;
        'listener_loop: loop {
            match input_read_rx.recv() {
                Ok(read) => {
                    if read && !sended {
                        self.get_keyboard_input();
                        println!("char sended {:?}", self.current_input);
                        input_tx.send(self.current_input).unwrap();
                        sended = true;
                    } else if !read {
                        sended = false;
                    }
                }
                _ => (), // do nothing
            }
            match game_off_rx.try_recv() {
                Ok(off) => {
                    if off {
                        break 'listener_loop;
                    }
                }
                _ => (), // do nothing
            }
        }
    }
}

/*



let mut input_map: HashMap<char, bool> = HashMap::new();
input_map.insert('2', false);
input_map.insert('4', false);
input_map.insert('6', false);
input_map.insert(' ', false);
input_map.insert('\n', false);



*/
