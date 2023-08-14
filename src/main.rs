use clap::{Parser, Subcommand};
use crossterm::event::Event as CrosstermEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc;
use strend::app::{App, AppResult, AppState, EXIT_ERROR_CODE, EXIT_SUCCESS_CODE};
use strend::event::{Event, EventHandler};
use strend::handler::handle_events;
use strend::tui::Tui;
use strend::util::init_api_key;

#[derive(Parser, Debug)]
#[command(
    name = "strend",
    about = "Search and visualize Shodan historical data in the terminal.",
    version
)]
struct Cli {
    /// Search query used to search the historical database, e.g. "product:nginx port:443"
    #[arg(long)]
    query: Option<String>,

    /// A comma-separated list of properties to get summary information on, e.g. country:10
    #[arg(long)]
    facets: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize Shodan API key, grab it from https://account.shodan.io
    Init { key: String },
}

fn main() -> AppResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { key }) => {
            init_api_key(key.to_string())?;
            std::process::exit(EXIT_SUCCESS_CODE);
        }
        None => {}
    }

    let query = cli.query.unwrap_or(String::new());
    let facets = cli.facets.unwrap_or(String::new());
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
        first_render: true,
        facet_indexes: HashMap::new(),
        sender,
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app, &mut state)?;
        state.first_render = false;

        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                let _ = app.tick();
            }
            Event::Key(event) => {
                // Skip process events on waiting for API response
                if app.blocking == 0 {
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
