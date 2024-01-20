mod args;

use {
    args::*,
    clap::Parser,
    crokey::*,
    crossterm::{
        event::{
            read,
            Event,
        },
        style::Stylize,
        terminal,
    },
};

pub fn main() {
    let args = Args::parse();
    let fmt = KeyCombinationFormat::default();
    println!(
        "Many terminal applications let you define key bindings but it's hard to know which ones are available."
    );
    println!(
        "This program shows you the key combinations that can be used in a program running in this terminal."
    );
    let mut combiner = Combiner::default();
    match args.mode {
        Mode::Ansi => {
            println!("You chose the ANSI mode.");
        }
        Mode::Kitty => {
            println!("You chose the Kitty mode.");
            if combiner.enable_combining().unwrap() {
                combiner.set_mandate_modifier_for_multiple_keys(true);
                println!(
                    "Combinations with several simple keys are possible as soon as there's at least one modifier key."
                );
            } else {
                println!("But it's not supported by your terminal.");
            }
        }
        Mode::KittyMultiSimples => {
            println!("You chose the KittyMultiSimples mode.");
            if combiner.enable_combining().unwrap() {
                combiner.set_mandate_modifier_for_multiple_keys(false);
                println!(
                    "Combinations with several simple keys are possible even without modifiers."
                );
            } else {
                println!("But it's not supported by your terminal.");
            }
        }
    }
    if !combiner.is_combining() {
        println!("Combinations will be reduced to one simple key with optional modifiers");
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
                    key!(ctrl - c) => {
                        println!("You killed me with a {}", key.red());
                        break;
                    }
                    key!(ctrl - q) => {
                        println!("You typed {} which gracefully quits", key.green());
                        break;
                    }
                    key!('?') | key!(shift - '?') => {
                        println!("You typed {} but there's no help in this app", key.yellow());
                    }
                    _ => {
                        println!("You typed {}", key.blue());
                    }
                }
            }
            e => {
                // any other event, for example a resize, we quit
                eprintln!("Quitting on {:?}", e);
                break;
            }
        }
    }
}
