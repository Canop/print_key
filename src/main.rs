use {
    crokey::*,
    crossterm::{
        event::{read, Event},
        terminal,
        style::Stylize,
    },
};

pub fn main() {
    let fmt = KeyCombinationFormat::default();
    println!("Many terminal applications let you define key bindings but it's hard to know which ones are available.");
    println!("This program shows you the key combinations that can be used in a program running in this terminal.");
    let mut combiner = Combiner::default();
    let combines = combiner.enable_combining().unwrap();
    if combines {
        println!("Your terminal supports combining keys");
    } else {
        println!("Your terminal doesn't support combining standard (non modifier) keys");
    }
    println!("Type any key combination.");
    loop {
        terminal::enable_raw_mode().unwrap();
        let e = read();
        terminal::disable_raw_mode().unwrap();
        match e {
            Ok(Event::Key(key_event)) => {
                let Some(key_combination) = combiner.transform(key_event) else {
                    continue;
                };
                let key = fmt.to_string(key_combination);
                match key_combination {
                    key!(ctrl-c) => {
                        println!("You killed me with a {}", key.red());
                        break;
                    }
                    key!(ctrl-q) => {
                        println!("You typed {} which gracefully quits", key.green());
                        break;
                    }
                    key!('?') | key!(shift-'?') => {
                        println!("You typed {} but there's no help in this app", key.yellow());
                    }
                    _ => {
                        println!("You typed {}", key.blue());
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
