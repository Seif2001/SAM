#[macro_use]
mod util;
mod system;
mod render;
mod console;
mod app;
mod process;
mod parser;
mod cmd;

use std::io;
use std::io::Write;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::env;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction};
use tui::Terminal;
use termion::raw::IntoRawMode;
use termion::cursor::Goto;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::event::Key;

use crate::system::System;
use crate::util::*;
use crate::render::*;
use crate::app::App;

#[macro_use]
extern crate nom;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1] == "help"{
            println!("\n Welcome to SAM Task Manager");
            println!("\n To run the TUI, type 'bash run.sh -c-' ");
            println!("\n This is an interactive task manager");
            println!("\n To sort type 'sort {{cpu, mem, pid, name, nice}}' Sorting is done descinfingly ");
            println!("\n To filter by name type 'name {{name}}', this accepts regex wildcards");
            println!("\n To filter by pid type 'pid {{pid}}'");
            println!("\n To filter by ppid type 'ppid {{ppid}}'");
            println!("\n To filter by state type 'state {{state}}'");
            println!("\n To filter by nice type 'nice {{nice}}'");
            println!("\n To add columns type 'add {{cpu, mem, nice, ppid, state}}'");
            println!("\n To kill a process type 'kill {{pid}}");
            println!("\n To increase priority of a process type 'incPriority {{pid}}");
            println!("\n To decrease priority of a process type 'decPriority {{pid}}");
            println!("\n To remove all filters do ctrl + r");
            println!("\n To exit ctrl + c");
        }

        
    }
    else{
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = util::Events::new();
    let mut system = System::new(terminal.size()?.width);
    let mut app = App {
        mode: Mode::Main,
        processes_sort_by: SortBy::CPU,
        processes_add_by: Addby::Nice,
        processes_sort_direction: SortDirection::DESC,
        size: tui::layout::Rect::new(0, 0, 0, 0),
        console: crate::console::Console::new(),
        system: System::new(terminal.size()?.width),
        should_render: true,
        pid: -1,
        nice: -21,
        name:format!(""),
        state:'x',
        ppid:-1
    };

    // Sets up separate thread for polling system resources
    let (system_tx, system_rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let system_update = system.update();
            system_tx.send(system_update).unwrap();
            thread::sleep(Duration::from_millis(2500));
        }
    });

   

    loop {
        app.size = terminal.size()?;

        // Received updated system info from separate thread
        if let Ok(updated_system) = system_rx.try_recv() {
            app.system = updated_system;
            app.should_render = true;
        }

        // Renders everything lazily to increase performance
        if app.should_render {
        
            let main_view_constraints = [Constraint::Length(4),Constraint::Length(4),Constraint::Min(6), Constraint::Length(3),  Constraint::Length(3)].as_ref();
            // Define layouts for the different sections of the display
            let main_view_layout = define_layout(Direction::Vertical, &main_view_constraints, terminal.size()?);
            
            
            // TODO: Implement lazy rendering
            terminal.draw(|mut f| {
                render_sparklines_layout(&mut f, &[main_view_layout[0], main_view_layout[1]], &app);
                render_processes_layout(&mut f, main_view_layout[2], &mut app);
                render_console_layout(&mut f, main_view_layout[3], &app);
                render_input_layout(&mut f, main_view_layout[4], &app);
            })?;

            app.should_render = false;
        }

        // Positions cursor after user input
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(2 + app.console.input.len() as u16, app.size.height - 1)
        )?;

        terminal.show_cursor()?;
        if let util::Event::Input(input) = events.next()? {
            match input {

                // Quit the program
                Key::Ctrl('c') => break,

                // Toggle showing the debugging log
                //Key::Char('/') => app.console.toggle_visibility(),

                // If enter was pressed, attempt to process current input as command
                Key::Char('\n') => app.process_command(),

                Key::Ctrl('r') =>{
                    app.pid = -1;
                    app.name = String::from("");
                    app.ppid = -1;
                    app.state = 'x';
                    app.nice = -21;
                }

                // Capture text input into the console
                Key::Char(c) => app.console.append_input(c),

                Key::Backspace => app.console.backspace(),

                _ => {}
            }

            app.should_render = true;
        }
        terminal.hide_cursor()?;
    }
    }
    Ok(())
}
