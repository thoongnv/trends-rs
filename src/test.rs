use std::collections::HashMap;
use std::fmt::Write;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use unicode_width::UnicodeWidthStr;

use ratatui::backend::TestBackend;
use ratatui::prelude::*;
use ratatui::Terminal;

use strend::app::{App, AppResult, AppState};
use strend::components::Component;
use strend::handler::handle_events;
use strend::ui;

#[test]
fn launch_app_and_make_few_searches() -> AppResult<()> {
    let query = "".to_string();
    let facets = "".to_string();
    let (sender, receiver) = mpsc::channel();

    let mut app = App::new(query, facets, receiver);
    let mut state: AppState = AppState {
        unfocused: true,
        submitted: false,
        first_render: true,
        facet_indexes: HashMap::new(),
        app_log: String::new(),
        sender,
    };

    let backend: TestBackend = TestBackend::new(140, 40);
    let mut terminal = Terminal::new(backend)?;

    // First launch app
    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
        state.first_render = false;

        ui::render(&mut app, &mut state, frame);
    })?;

    let expected = Buffer::with_lines(vec![
        "┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
        "│ Query:                                                                                                                                   │",
        "│ Facets (optional):                                                                                                                       │",
        "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        "┌Info──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                               Make search by `Enter` a query in search box.                                              │",
        "│                                      Press `Ctrl-C` to stop running, switch between panels by `Tab`                                      │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "│                                                                                                                                          │",
        "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        "                                                                                                                                            ",
        "Switch panels [⇥]  Exit [^C]                                                                                                                ",
    ]);
    println!("{:?}", terminal.backend().buffer());
    terminal.backend().assert_buffer(&expected);

    // Focus searchbox
    app.switch_widgets(&mut state, false)?;
    assert!(app.search_input.focused());

    // Enter query
    for c in "nginx".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    // Focus facets input
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;
    assert!(app.facets_input.focused());

    // Enter facets
    for c in "os:10".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    // Re-render UI
    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    // Test styled TUI
    // let mut expected = Buffer::with_lines(vec![
    //     "┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
    //     "│ Query: nginx                                                                                                                             │",
    //     "│ Facets (optional): os:10                                                                                                                 │",
    //     "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
    //     "┌Info──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                               Make search by `Enter` a query in search box.                                              │",
    //     "│                                      Press `Ctrl-C` to stop running, switch between panels by `Tab`                                      │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "│                                                                                                                                          │",
    //     "└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
    //     "                                                                                                                                            ",
    //     "Search [⏎]  Move cursor [←→]  Delete Char [⌫]  Up/ Down [↑↓]  Unfocused [⎋]  Switch panels [⇥]  Exit [^C]                                   ",
    // ]);

    // // Style searchbox buffer
    // for i in 0..=139 {
    //     expected
    //         .get_mut(i, 0)
    //         .set_style(Style::default().fg(Color::Yellow));
    // }

    // expected
    //     .get_mut(0, 1)
    //     .set_style(Style::default().fg(Color::Yellow));

    // expected
    //     .get_mut(139, 1)
    //     .set_style(Style::default().fg(Color::Yellow));

    // expected
    //     .get_mut(0, 2)
    //     .set_style(Style::default().fg(Color::Yellow));

    // expected
    //     .get_mut(139, 2)
    //     .set_style(Style::default().fg(Color::Yellow));

    // for i in 2..=19 {
    //     expected
    //         .get_mut(i, 2)
    //         .set_style(Style::default().add_modifier(Modifier::BOLD));
    // }

    // for i in 0..=139 {
    //     expected
    //         .get_mut(i, 3)
    //         .set_style(Style::default().fg(Color::Yellow));
    // }

    // terminal.backend().assert_buffer(&expected);

    // Otherwise just check some rendered text in the buffer
    // Note that: println! only output on cargo test -- --nocapture
    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Query: nginx"));
    assert!(buffer_str.contains("Facets (optional): os:10"));
    assert!(buffer_str.contains("Make search by `Enter` a query in search box."));
    assert!(buffer_str.contains("Switch panels [⇥]"));

    // Make search
    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("[x] query=nginx&facets=os"));
    assert!(buffer_str.contains("[x] Linux"));
    assert!(buffer_str.contains("[x] Ubuntu"));
    assert!(buffer_str.contains("[ ] Windows"));
    assert!(buffer_str.contains("Jun 2017"));
    assert!(buffer_str.contains("Up/ Down [↑↓]"));
    assert!(!buffer_str.contains("Export [^E]"));

    // Unfocus searchbox will show Export keybinding
    app.switch_widgets(&mut state, false)?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Export [^E]"));

    // Export chart data
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('E'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Exported chart to ./data.csv"));

    // Clear application log on next rendering
    app.ticks = 100;

    // We currently in Saved queries block, uncheck first MultiStatefulList line
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("Exported chart to ./data.csv"));
    assert!(buffer_str.contains("[ ] query=nginx&facets=os"));

    // Re-export chart, here empty data
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('E'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("No chart to export"));
    app.ticks = 100;

    app.select_widget(0);

    // Clear query input
    for _ in 0..20 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    // Make search with empty query
    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(app.search_input.get_input().is_empty());
    assert!(buffer_str.contains("Invalid search query"));

    // Enter valid query with no resuts
    for c in "port:111222".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    app.select_widget(1);

    // Invalid search facet
    for _ in 0..20 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    for c in "orggg".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Invalid search facet"));

    for _ in 0..2 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    search_and_render(&mut app, &mut state, &mut terminal)?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("No results found"));

    // Back Tab to switch to previous panel
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    for _ in 0..20 {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    for c in "apache port:80".chars() {
        handle_events(
            Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty())),
            &mut app,
            &mut state,
        )?;
    }

    search_and_render(&mut app, &mut state, &mut terminal)?;

    // Need one more render to see first saved queries checked
    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("No results found"));
    assert!(buffer_str.contains("[x] query=apache+port%3A8"));
    assert!(buffer_str.contains("[ ] query=nginx&facets=os"));
    assert!(buffer_str.contains("[x] Amazon.com"));
    assert!(app.line_chart.data[0].len() > 1);

    // Focus Facet values
    app.select_widget(3);
    assert!(app.facet_values.focused());

    // Unselect all lines
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("[ ] Amazon.com"));
    assert!(app.line_chart.data[0].len() == 1);

    // Select all lines
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("[x] Amazon.com"));
    assert!(buffer_str.contains("[x] Korea Telecom"));
    assert!(app.line_chart.data[0].len() > 1);

    // Export chart data again
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(buffer_str.contains("Exported chart to ./data.csv"));
    assert!(app.facet_values.focused());
    assert!(!state.unfocused);
    assert!(app.running);

    // Unfocus all widgets
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty())),
        &mut app,
        &mut state,
    )?;

    terminal.draw(|frame| {
        ui::render(&mut app, &mut state, frame);
    })?;

    println!("{:?}", terminal.backend().buffer());
    let buffer_str = buffer_view(terminal.backend().buffer());
    assert!(!buffer_str.contains("Up/ Down [↑↓]"));
    assert!(!app.facet_values.focused());
    assert!(state.unfocused);
    assert!(app.running);

    // Close application
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::CONTROL)),
        &mut app,
        &mut state,
    )?;
    assert!(!app.running);

    Ok(())
}

fn search_and_render(
    app: &mut App,
    state: &mut AppState,
    terminal: &mut Terminal<TestBackend>,
) -> AppResult<()> {
    // Input Enter key
    handle_events(
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())),
        app,
        state,
    )?;

    // Waiting for API response
    while app.blocking > 0 {
        sleep(Duration::from_millis(app.tick_rate));
        let _ = app.tick();
    }

    // Render TUI
    terminal.draw(|frame| {
        ui::render(app, state, frame);
    })?;

    Ok(())
}

// Clone ratatui-0.22.0/src/backend/test.rs::buffer_view as I can't import it
fn buffer_view(buffer: &Buffer) -> String {
    let mut view = String::with_capacity(buffer.content.len() + buffer.area.height as usize * 3);
    for cells in buffer.content.chunks(buffer.area.width as usize) {
        let mut overwritten = vec![];
        let mut skip: usize = 0;
        view.push('"');
        for (x, c) in cells.iter().enumerate() {
            if skip == 0 {
                view.push_str(&c.symbol);
            } else {
                overwritten.push((x, &c.symbol));
            }
            skip = std::cmp::max(skip, c.symbol.width()).saturating_sub(1);
        }
        view.push('"');
        if !overwritten.is_empty() {
            write!(&mut view, " Hidden by multi-width symbols: {overwritten:?}").unwrap();
        }
        view.push('\n');
    }
    view
}
