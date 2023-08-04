use clap::Parser;
use crossterm::event::Event as CrosstermEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::sync::mpsc;
use strend::app::{App, AppResult, AppState, EXIT_ERROR_CODE};
use strend::event::{Event, EventHandler};
use strend::handler::handle_events;
use strend::tui::Tui;

#[derive(Parser, Debug)]
#[command(
    name = "Shodan",
    about = "Search and visualize Shodan historical data in the terminal."
)]

struct Args {
    /// Search query used to search the historical database, e.g. "product:nginx port:443"
    #[arg(long)]
    query: Option<String>,

    /// A comma-separated list of properties to get summary information on, e.g. country:10
    #[arg(long)]
    facets: Option<String>,
}

fn main() -> AppResult<()> {
    let args = Args::parse();
    let query = args.query.unwrap_or(String::new());
    let facets = args.facets.unwrap_or(String::new());
    // Shared data to run API requests in separate thread so it's not block application
    let (sender, receiver) = mpsc::channel();

    // Must provide query if input --facets
    if !facets.is_empty() && query.is_empty() {
        println!("Error: Invalid arguments, please check: strend --help");
        std::process::exit(EXIT_ERROR_CODE);
    }

    // Create an application.
    let mut app = App::new(query, facets, receiver);
    let mut state: AppState = AppState {
        unfocused: true,
        submitted: false,
        sender,
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app, &mut state)?;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                let _ = app.tick();
            }
            Event::Key(event) => {
                // Skip process events on waiting for API response
                if !app.blocking {
                    handle_events(CrosstermEvent::Key(event), &mut app, &mut state)?
                }
            }
            _ => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
