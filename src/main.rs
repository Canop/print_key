use {
    crokey::*,
    crossterm::{
        event::{read, Event},
        terminal,
        style::Stylize,
    },
};

pub fn main() {
    let fmt = KeyEventFormat::default();
    println!("Many terminal applications let you define key bindings but it's hard to know which ones are available.");
    println!("This program shows you the key combinations that can be used in a program running in this terminal.");
    println!("Type any key combination.");
    loop {
        terminal::enable_raw_mode().unwrap();
        let e = read();
        terminal::disable_raw_mode().unwrap();
        match e {
            Ok(Event::Key(key_event)) => {
                match key_event {
                    key!(ctrl-c) => {
                        println!("You killed me with a {}", fmt.to_string(key_event).red());
                        break;
                    }
                    key!(ctrl-q) => {
                        println!("You typed {} which gracefully quits", fmt.to_string(key_event).green());
                        break;
                    }
                    key!('?') | key!(shift-'?') => {
                        println!("You typed {} but there's no help in this app", fmt.to_string(key_event).yellow());
                    }
                    _ => {
                        println!("You typed {}", fmt.to_string(key_event).blue());
                    }
                }
            }
            e => { // any other event, for example a resize, we quit
                eprintln!("Quitting on {:?}", e);
                break;
            }
        }
    }
}
