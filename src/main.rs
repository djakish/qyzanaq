use clap::Parser;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    {cursor, execute},
};
use std::{io::stdout, process::Command, thread, time::Duration};

/// Simple pomodoro program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Work time in seconds
    #[clap(short, long, default_value_t = 1500)]
    work: u16,

    /// Short break in seconds
    #[clap(short, long, default_value_t = 300)]
    short: u16,

    /// Long break in seconds
    #[clap(short, long, default_value_t = 1200)]
    long: u16,

    /// Execute command on each round
    #[clap(short, long, default_value = "")]
    execute: String,
}

enum Pomodoro {
    Work,
    Long,
    Short,
}

fn main() {
    let args = Args::parse();
    let mut pomodoro = Pomodoro::Work;
    let mut counter = 1;
    let mut time = args.work;
    let mut playing = true;
    let mut color = Color::Green;
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    loop {
        if poll(Duration::from_secs(0)).unwrap() {
            match read().unwrap() {
                // key listener
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: _no_modifiers,
                }) => {
                    playing = !playing;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: _no_modifiers,
                }) => {
                    playing = !playing;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: _no_modifiers,
                }) => {
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    break;
                }
                _ => (),
            }
            if playing {
                color = Color::Green;
            } else {
                color = Color::Red;
            }
        } else {
            let out = display(time, &pomodoro);
            execute!(
                stdout,
                Clear(ClearType::All),
                cursor::MoveTo(0, 0),
                SetForegroundColor(color),
                Print(out),
            )
            .unwrap();
            execute!(stdout, cursor::MoveTo(0, 1)).unwrap();

            if playing {
                time -= 1;
            }

            if time == 0 {
                match pomodoro {
                    Pomodoro::Work => {
                        if counter == 4 {
                            time = args.long;
                            pomodoro = Pomodoro::Long;
                            counter = 0;
                            notification("Long break start!".to_string(), &args.execute);
                        } else {
                            time = args.short;
                            pomodoro = Pomodoro::Short;
                            notification("Short break start!".to_string(), &args.execute);
                        }
                        counter += 1;
                    }
                    _ => {
                        time = args.work;
                        pomodoro = Pomodoro::Work;
                        notification("Work start!".to_string(), &args.execute);
                    }
                }
            }
            thread::sleep(Duration::from_secs(1))
        }
    }
    disable_raw_mode().unwrap();
}

fn notification(text: String, execute: &str) {
    Command::new("notify-send")
        .arg("QYZANAQ")
        .arg(text)
        .status()
        .expect("couldn't send command");
    if !execute.is_empty() {
        let clone = execute.to_string();
        thread::spawn(move || {
            Command::new("/bin/sh")
                .arg("-c")
                .arg(clone)
                .status()
                .expect("couldn't run command");
        });
    }
}

fn display(time: u16, status: &Pomodoro) -> String {
    let min = time / 60;
    let sec = time % 60;
    match status {
        Pomodoro::Work => {
            format!("Work! {:02}:{:02} ", min, sec)
        }
        Pomodoro::Long => {
            format!("Long break! {:02}:{:02} ", min, sec)
        }
        Pomodoro::Short => {
            format!("Short break! {:02}:{:02} ", min, sec)
        }
    }
}
