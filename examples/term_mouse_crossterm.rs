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

    println!("Please start moving, clicking, dragging with mouse in terminal. To exit press q.");

    // Main event loop.
    loop {
        // Poll for an event every 500ms
        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(key_event) => {
                    // If 'q' is pressed, exit.
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => break,
                        KeyCode::Char('d') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => break,
                        _ => {
                            println!("\rKey event: {:?}", key_event);
                            println!();
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::Down(btn) => {
                            println!("\rMouse down at ({}, {}), button: {:?}", 
                                mouse_event.column, mouse_event.row, btn
                            );
                            println!();
                        }
                        MouseEventKind::Up(btn) => {
                            println!("\rMouse up at ({}, {}), button: {:?}", 
                                mouse_event.column, mouse_event.row, btn
                            );
                        }
                        MouseEventKind::Drag(btn) => {
                            println!("\rMouse drag at ({}, {}), button: {:?}", 
                                mouse_event.column, mouse_event.row, btn
                            );
                        }
                        MouseEventKind::Moved => {
                            println!("\rMouse moved to ({}, {})", 
                                mouse_event.column, mouse_event.row
                            );
                        }
                        MouseEventKind::ScrollDown => {
                            println!("\rScroll down at ({}, {})", 
                                mouse_event.column, mouse_event.row
                            );
                        }
                        MouseEventKind::ScrollUp => {
                            println!("\rScroll up at ({}, {})", 
                                mouse_event.column, mouse_event.row
                            );
                        }
                        MouseEventKind::ScrollLeft => {
                            println!("\rScroll left at ({}, {})", 
                                mouse_event.column, mouse_event.row
                            );
                        }
                        MouseEventKind::ScrollRight => {
                            println!("\rScroll right at ({}, {})", 
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
