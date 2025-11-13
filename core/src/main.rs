mod plugins;
mod bus;
mod core;

use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::{terminal, execute, event};
use crossterm::event::{Event, KeyEventKind};
use ratatui::{Terminal, prelude::*, backend::CrosstermBackend};
use plugin_api::*;

use crate::core::Core;

fn main() -> Result<()> {
    // Build Core
    let core: Core = Core::new();
    core.log.log("test");

    return Result::Ok(());

    // Setup TUI
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Event Bus Communication
    let (tx, mut rx) = bus::channel();

    // Plugin Loading
    let mut loaded = plugins::load_from_dir(std::path::Path::new("plugins"))?;
    for lp in loaded.iter_mut() {
        lp.plugin.init(Box::new(tx.clone()))?;
        lp.plugin.start_tasks(Box::new(tx.clone()))?;
    }

    // Collect Panes
    let mut panes: Vec<Box<dyn Pane>> = loaded.iter_mut()
        .flat_map(|lp| lp.plugin.panes())
        .collect();
    let mut focused = 0usize;
    let mut last = Instant::now();

    'main: loop {
        while let Some((_topic, _bytes)) = rx.try_recv() {
            // If needed: route to panes here.
        }

        // Tick Manager
        let now = Instant::now();
        let dt = now - last; last = now;
        for p in panes.iter_mut() {
            p.on_tick(Tick { dt });
        }

        // Display
        terminal.draw(|f| {
            // unsafe but isolated: pass Frame pointer through FrameWrapper
            let ptr = f as *const _ as *mut ();
            let mut fw = FrameWrapper::new(ptr);
            panes[focused].draw(&mut fw);
        })?;

        // Capture Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press {
                    if k.code == crossterm::event::KeyCode::Char('q') { break 'main; }
                    if k.code == crossterm::event::KeyCode::Tab { focused = (focused + 1) % panes.len().max(1); }
                }
            }
        }
    }

    // Quit Application
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, event::DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
