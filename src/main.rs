use std::io::{self, Write};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal,
};

fn main() -> std::io::Result<()> {
    // Enable raw mode so we can capture events directly.
    terminal::enable_raw_mode()?;

    // Enable mouse capture.
    execute!(io::stdout(), EnableMouseCapture)?;

    // Main event loop.
    loop {
        // Poll for an event every 500ms
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(key_event) => {
                    // If 'q' is pressed, exit.
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        _ => {
                            println!("Key event: {:?}", key_event);
                        }
                    }
                    MouseEventKind::ScrollLeft => {
                        println!(
                            "Scroll left at ({}, {})",
                            mouse_event.column, mouse_event.row
                        );
                    }
                    MouseEventKind::ScrollRight => {
                        println!(
                            "Scroll right at ({}, {})",
                            mouse_event.column, mouse_event.row
                        );
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::Down(btn) => {
                            println!(
                                "Mouse down at ({}, {}), button: {:?}",
                                mouse_event.column, mouse_event.row, btn
                            );
                        }
                        MouseEventKind::Up(btn) => {
                            println!(
                                "Mouse up at ({}, {}), button: {:?}",
                                mouse_event.column, mouse_event.row, btn
                            );
                        }
                        MouseEventKind::Drag(btn) => {
                            println!(
                                "Mouse drag at ({}, {}), button: {:?}",
                                mouse_event.column, mouse_event.row, btn
                            );
                        }
                        MouseEventKind::Moved => {
                            println!(
                                "Mouse moved to ({}, {})",
                                mouse_event.column, mouse_event.row
                            );
                        }
                        MouseEventKind::ScrollDown => {
                            println!(
                                "Scroll down at ({}, {})",
                                mouse_event.column, mouse_event.row
                            );
                        }
                        MouseEventKind::ScrollUp => {
                            println!(
                                "Scroll up at ({}, {})",
                                mouse_event.column, mouse_event.row
                            );
                        }
                    }
                    io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }
    }

    // Disable mouse capture and restore the terminal.
    execute!(io::stdout(), DisableMouseCapture)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
