use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    println!("Enabling raw mode...");
    enable_raw_mode()?;
    
    // Flush any buffered input
    while poll(Duration::from_millis(0))? {
        let _ = read()?;
    }
    
    println!("Press keys (ESC to exit)...");
    
    loop {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => println!("Got char: '{}' ASCII: {}", c, c as u32),
                    KeyCode::Enter => println!("Got Enter"),
                    _ => println!("Got other key"),
                }
            }
        }
    }
    
    disable_raw_mode()?;
    Ok(())
}
