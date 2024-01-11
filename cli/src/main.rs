mod args;
mod cli;

use args::{Args, Class, Cmd};
use cli::Cli;
use hidg::{Button, Device, Key, Keyboard, Mouse, StateChange, ValueChange};
use rustyline::{error::ReadlineError, Editor};

fn main() -> anyhow::Result<()> {
    let args: Args = clap::Parser::parse();

    match args.cmd {
        Cmd::Repl { class, path } => match class {
            Class::Keyboard => {
                let mut dev = Device::<Keyboard>::open(path)?;

                let mut rl = Editor::<Cli>::new()?;
                rl.set_helper(Cli::new(class).into());
                loop {
                    let readline = rl.readline(">> ");
                    match readline {
                        Ok(line) => {
                            rl.add_history_entry(line.as_str());

                            let mut words = line.split(char::is_whitespace);
                            if let Some(cmd) = words.next() {
                                match cmd {
                                    "" => {
                                        let keys = dev
                                            .input()
                                            .pressed()
                                            .map(|k| k.to_string())
                                            .collect::<Vec<_>>()
                                            .join(" ");

                                        let leds = dev
                                            .output()
                                            .lit()
                                            .map(|l| l.to_string())
                                            .collect::<Vec<_>>()
                                            .join(" ");

                                        println!("Keys pressed: {}, Leds lit: {}", keys, leds);
                                    }
                                    "press" => {
                                        let keys = words
                                            .map(|k| k.parse().map(StateChange::press))
                                            .collect::<Result<Vec<StateChange<Key>>, _>>()?;
                                        dev.updates(keys)?;
                                    }
                                    "release" => {
                                        let keys = words
                                            .map(|k| k.parse().map(StateChange::release))
                                            .collect::<Result<Vec<StateChange<Key>>, _>>()?;
                                        dev.updates(keys)?;
                                    }
                                    other => {
                                        println!("Unknown command: {}", other);
                                    }
                                }
                            }
                        }
                        Err(ReadlineError::Interrupted) => {
                            println!("CTRL-C");
                            break;
                        }
                        Err(ReadlineError::Eof) => {
                            println!("CTRL-D");
                            break;
                        }
                        Err(err) => {
                            println!("Error: {:?}", err);
                            break;
                        }
                    }
                }
            }
            Class::Mouse => {
                let mut dev = Device::<Mouse>::open(path)?;
            }
        },
    }

    Ok(())
}
