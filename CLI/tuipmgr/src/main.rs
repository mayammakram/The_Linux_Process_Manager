// use procfs::keyring::Key;
//ProcSys or ProcPulse
// use chrono::prelude::*;
//use nix::sys::signal::{kill, Signal};
//use nix::unistd::Pid;

use std::os::raw::c_int;
use libc::{c_long, pid_t, setpriority, PRIO_PROCESS};

use nix::errno::Errno;
use std::process::Command;

use libc::signal;
use nix::sys::select::select;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

use crate::process::Stdio;
extern crate libc;
use tui::widgets::ListState;
use std::process::Child;
use std::{process, string};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use psutil::process::processes;
use sysinfo::{System, SystemExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState, Tabs, Wrap},
    Terminal,
};

extern crate procfs;
extern crate sysinfo;
// use std::fs::File;
// use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use sysconf;

// const DB_PATH: &str = "./data/db.json";

//static mut searched_proc: String = String::from("Hello, world!");


#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone)]

//create a struct called process and store all the information we want to display
struct Proc {
    pid: i32,
    name: String,
    state: char,
    parent_id: i32,
    priority: i64,
    niceness: i64,
    user_id: u32,
    memory: i64,
    cpu_time: String,
    open_files: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MenuItem {
    Home,
    Table,
    Graphs,
}
impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Table => 1,
            MenuItem::Graphs => 2,
        }
    }
}

#[derive(PartialEq)]
enum OptionsItem {
    Sort,
    Search,
    Filter,
    Terminate,
    SetPriority,
    None,
}
impl From<OptionsItem> for usize {
    fn from(input: OptionsItem) -> usize {
        match input {
            OptionsItem::Sort => 0,
            OptionsItem::Search => 1,
            OptionsItem::Filter => 2,
            OptionsItem::Terminate => 3,
            OptionsItem::SetPriority => 4,
            OptionsItem::None => 5,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum FieldItem {
    Pid,
    Name,
    State,
    ParentPid,
    Priority,
    Niceness,
    UserId,
    Memory,
    CpuTime,
    OpenFiles,
    Exit,
}
impl From<FieldItem> for usize {
    fn from(input: FieldItem) -> usize {
        match input {
            FieldItem::Pid => 0,
            FieldItem::Name => 1,
            FieldItem::State => 2,
            FieldItem::ParentPid => 3,
            FieldItem::Priority => 4,
            FieldItem::Niceness => 5,
            FieldItem::UserId => 6,
            FieldItem::Memory => 7,
            FieldItem::CpuTime => 8,
            FieldItem::OpenFiles => 9,
            FieldItem::Exit => 10,
        }
    }
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}
struct InputField {
    input: String,
    //History of recorded messages
    messages: Vec<String>,
}
impl Default for InputField {
    fn default() -> InputField {
        InputField {
            input: String::new(),
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;
    let mut active_options_item: OptionsItem = OptionsItem::None;
    let mut active_field = FieldItem::Pid;
    let mut proc_list_state: TableState = TableState::default();
    proc_list_state.select(Some(0));
    let mut sort_field = "";
    let mut active_input_state = InputMode::Normal;
    let mut input_field = InputField::default();
    input_field.messages.push("".to_string());
    loop {
        
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new("The Linux Process Manager 2023 - all rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(render_menu(active_menu_item), chunks[0]);
            match active_menu_item {
                MenuItem::Home => {
                    let home_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
                        )
                        .split(chunks[1]);
                    rect.render_widget(render_home(), home_chunks[0]);
                    rect.render_widget(render_sys(), home_chunks[1]);
                    rect.render_widget(render_hometab(), chunks[2]);
                }
                MenuItem::Table => {
                    let (left, right,searchedProc) = render_searchbar(&input_field);
                    rect.render_stateful_widget(
                        render_table(sort_field, &proc_list_state,searchedProc),
                        chunks[1],
                        &mut proc_list_state,
                    );
                    match active_options_item {
                        OptionsItem::Sort => {
                            let (left, right) = render_sortbar(active_field);
                            let sort_chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(6), Constraint::Percentage(94)]
                                        .as_ref(),
                                )
                                .split(chunks[2]);
                            rect.render_widget(left, sort_chunks[0]);
                            rect.render_widget(right, sort_chunks[1]);
                        }
                        OptionsItem::Filter => {
                            let (left, right) = render_filterbar(&input_field);
                            let subchunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(9), Constraint::Percentage(91)]
                                        .as_ref(),
                                )
                                .split(chunks[2]);
                            rect.render_widget(left, subchunks[0]);
                            rect.render_widget(right, subchunks[1]);
                        }
                        OptionsItem::Search => {
                            let (left, right,searchedProc) = render_searchbar(&input_field);
                            let subchunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(9), Constraint::Percentage(91)]
                                        .as_ref(),
                                )
                                .split(chunks[2]);
                            rect.render_widget(left, subchunks[0]);
                            rect.render_widget(right, subchunks[1]);

                        }
                        OptionsItem::None => rect.render_widget(render_tabletab(), chunks[2]),
                        OptionsItem::SetPriority => {
                            // let (left, right) = render_priority_bar();
                            let left = render_priority_bar();
                            let subchunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(8), Constraint::Percentage(92)]
                                        .as_ref(),
                                )
                                .split(chunks[2]);
                            rect.render_widget(left, subchunks[0]);
                            // rect.render_widget(right, subchunks[1]);
                        }
                        OptionsItem::Terminate => {
                            //let (left, right) = render_term_bar();
                            let left = render_term_bar();
                            let subchunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(
                                    [Constraint::Percentage(6), Constraint::Percentage(94)]
                                        .as_ref(),
                                )
                                .split(chunks[2]);
                            rect.render_widget(left, subchunks[0]);
                            // rect.render_widget(right, subchunks[1]);
                        }
                    }
                }
                MenuItem::Graphs => {
                    rect.render_widget(render_graphs(), chunks[1]);
                    rect.render_widget(render_graphtab(), chunks[2]);
                }
            }
            rect.render_widget(copyright, chunks[3]);
        })?;
        if active_input_state == InputMode::Normal {
            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        terminal.clear()?;
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('Q') => {
                        terminal.clear()?;
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('h') => {
                        active_menu_item = MenuItem::Home;
                        active_options_item = OptionsItem::None;
                    }
                    KeyCode::Char('H') => {
                        active_menu_item = MenuItem::Home;
                        active_options_item = OptionsItem::None;
                    }
                    KeyCode::Char('d') => {
                        active_menu_item = MenuItem::Table;
                        active_options_item = OptionsItem::None;
                    }
                    KeyCode::Char('D') => {
                        active_menu_item = MenuItem::Table;
                        active_options_item = OptionsItem::None;
                    }
                    KeyCode::Char('g') => {
                        active_menu_item = MenuItem::Graphs;
                        active_options_item = OptionsItem::None;
                    }
                    KeyCode::Char('G') => {
                        active_menu_item = MenuItem::Graphs;
                        active_options_item = OptionsItem::None;
                    }
                    KeyCode::Char('S') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::Sort;
                        } else {
                        }
                    }
                    KeyCode::Char('s') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::Sort;
                        } else {
                        }
                    }
                    KeyCode::Char('F') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::Filter;
                            active_input_state = InputMode::Editing;
                            input_field.messages.clear();
                            input_field.messages.push("".to_string());
                        }
                    }
                    KeyCode::Char('f') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::Filter;
                            active_input_state = InputMode::Editing;
                            input_field.messages.clear();
                            input_field.messages.push("".to_string());
                        }
                    }
                    KeyCode::Char('e') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::Search;
                            active_input_state = InputMode::Editing;
                            input_field.messages.clear();
                            input_field.messages.push("".to_string());
                        }
                    }
                    KeyCode::Char('E') => {
                        if active_options_item != OptionsItem::Search
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::Search;
                            active_input_state = InputMode::Editing;
                            input_field.messages.clear();
                            input_field.messages.push("".to_string());
                        }
                    }
                    KeyCode::Char('p') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::SetPriority;
                        }
                    }
                    KeyCode::Char('P') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::SetPriority;
                        }
                    }
                    KeyCode::Char('T') => {
                        if active_options_item == OptionsItem::None
                            && active_menu_item == MenuItem::Table
                        {
                            active_options_item = OptionsItem::None;
                        }
                        let mut proc_list: Vec<Proc> = collect_proc_data().expect("can fetch proc data");
                        proc_list = sort(sort_field, proc_list);
                        let mut selected_pid = proc_list
                        .get(
                            proc_list_state
                                .selected()
                                .expect("there is always a selected pet"),
                        )
                        .expect("exists")
                        .clone().pid;
                
                    for proc in proc_list {
                        if proc.pid==selected_pid
                          {println!("{}",proc.pid);
                          let mut child = Command::new("kill")
                          .arg("-9")
                          .arg(selected_pid.to_string())
                          .spawn()
                          .expect("failed to execute process");
                        }
                    }
                        
                    }
                    KeyCode::Char('t') => {
                        let mut proc_list: Vec<Proc> = collect_proc_data().expect("can fetch proc data");
                        proc_list = sort(sort_field, proc_list);
                        let mut selected_pid = proc_list
                        .get(
                            proc_list_state
                                .selected()
                                .expect("there is always a selected pet"),
                        )
                        .expect("exists")
                        .clone().pid;
                
                    for proc in proc_list {
                        if proc.pid==selected_pid
                          {println!("{}",proc.pid);
                          let mut child = Command::new("kill")
                          .arg("-9")
                          .arg(selected_pid.to_string())
                          .spawn()
                          .expect("failed to execute process");
                        }
                    }

                    }
                    KeyCode::Char('x') => {
                        if active_options_item != OptionsItem::None {
                            active_options_item = OptionsItem::None;
                        }
                    }
                    KeyCode::Char('X') => {
                        if active_options_item != OptionsItem::None {
                            active_options_item = OptionsItem::None;
                        }
                    }
                    KeyCode::Char('z') => {
                        if active_options_item != OptionsItem::None {
                            active_options_item = OptionsItem::None;
                        }
                    }
                    KeyCode::Char('Z') => {
                        if active_options_item != OptionsItem::None {
                            active_options_item = OptionsItem::None;
                        }
                    }
                    KeyCode::Down => {
                        if active_menu_item == MenuItem::Table {
                            if active_options_item == OptionsItem::None {
                                if let Some(selected) = proc_list_state.selected() {
                                    let amount_procs =
                                        collect_proc_data().expect("can fetch pet list").len();
                                    if selected >= amount_procs - 1 {
                                        proc_list_state.select(Some(0));
                                    } else {
                                        proc_list_state.select(Some(selected + 1));
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Up => {
                        if active_menu_item == MenuItem::Table {
                            if active_options_item == OptionsItem::None {
                                if let Some(selected) = proc_list_state.selected() {
                                    let amount_procs =
                                        collect_proc_data().expect("can fetch pet list").len();
                                    if selected > 0 {
                                        proc_list_state.select(Some(selected - 1));
                                    } else {
                                        proc_list_state.select(Some(amount_procs - 1));
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Right => {
                        if active_options_item == OptionsItem::Sort {
                            match active_field {
                                FieldItem::Pid => active_field = FieldItem::Name,
                                FieldItem::Name => active_field = FieldItem::State,
                                FieldItem::State => active_field = FieldItem::ParentPid,
                                FieldItem::ParentPid => active_field = FieldItem::Priority,
                                FieldItem::Priority => active_field = FieldItem::Niceness,
                                FieldItem::Niceness => active_field = FieldItem::UserId,
                                FieldItem::UserId => active_field = FieldItem::Memory,
                                FieldItem::Memory => active_field = FieldItem::CpuTime,
                                FieldItem::CpuTime => active_field = FieldItem::OpenFiles,
                                FieldItem::OpenFiles => active_field = FieldItem::Exit,
                                FieldItem::Exit => active_field = FieldItem::Pid,
                            }
                        }
                    }
                    KeyCode::Left => {
                        if active_options_item == OptionsItem::Sort {
                            match active_field {
                                FieldItem::Pid => active_field = FieldItem::Exit,
                                FieldItem::Name => active_field = FieldItem::Pid,
                                FieldItem::State => active_field = FieldItem::Name,
                                FieldItem::ParentPid => active_field = FieldItem::State,
                                FieldItem::Priority => active_field = FieldItem::ParentPid,
                                FieldItem::Niceness => active_field = FieldItem::Priority,
                                FieldItem::UserId => active_field = FieldItem::Niceness,
                                FieldItem::Memory => active_field = FieldItem::UserId,
                                FieldItem::CpuTime => active_field = FieldItem::Memory,
                                FieldItem::OpenFiles => active_field = FieldItem::CpuTime,
                                FieldItem::Exit => active_field = FieldItem::OpenFiles,
                            };
                        }
                    }
                    KeyCode::Enter => {
                        if active_options_item == OptionsItem::Sort {
                            match active_field {
                                FieldItem::Pid => sort_field = "pid",
                                FieldItem::Name => sort_field = "name",
                                FieldItem::State => sort_field = "state",
                                FieldItem::ParentPid => sort_field = "parent_pid",
                                FieldItem::Priority => sort_field = "priority",
                                FieldItem::Niceness => sort_field = "niceness",
                                FieldItem::UserId => sort_field = "user_id",
                                FieldItem::Memory => sort_field = "memory",
                                FieldItem::CpuTime => sort_field = "cpu_time",
                                FieldItem::OpenFiles => sort_field = "open_files",
                                FieldItem::Exit => {
                                    active_options_item = OptionsItem::None;
                                    sort_field = "";
                                }
                            };
                        }
                    }
                    _ => {}
                },
                Event::Tick => {}
            }
        }
        
        if active_input_state == InputMode::Editing && active_options_item != OptionsItem::None {
            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Enter => {
                        input_field
                            .messages
                            .push(input_field.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        input_field.input.push(c);
                    }
                    KeyCode::Backspace => {
                        input_field.input.pop();
                    }
                    KeyCode::Esc => {
                        input_field.input.clear();
                        active_input_state = InputMode::Normal;
                        active_options_item = OptionsItem::None;
                    }
                    _ => {}
                },
                Event::Tick => {}
            }
        }
    }
    Ok(())
}
fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "The Linux Process Manager",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(
            "Press 'D' to display the table of processes",
        )]),
        Spans::from(vec![Span::raw("Press 'G' to display graphs and charts")]),
        Spans::from(vec![Span::raw("Press 'H' to display the home screen")]),
        Spans::from(vec![Span::raw("Press 'Q' to quit the application")]),
        Spans::from(vec![Span::raw(
            "Use the Options bar to guide you through more options!",
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}
fn render_sys<'a>() -> Paragraph<'a> {
    let sys = System::new_all();
    let sys_name = sys.name().unwrap();
    let sys_kernel_version = sys.kernel_version().unwrap();
    let sys_os_version = sys.os_version().unwrap();
    let sys_host_name = sys.host_name().unwrap();
    let tab = "   ";
    let text = vec![
        Spans::from(vec![Span::styled(
            "   About your system:",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "System name:{}{}{}{}{}",
            tab, tab, tab, tab, sys_name
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "System kernel version:  {}",
            sys_kernel_version
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "System OS version:{}{}{}",
            tab, tab, sys_os_version
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "System host name: {}{}{}",
            tab, tab, sys_host_name
        ))]),
    ];

    let systeminfo = Paragraph::new(text)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    systeminfo
}
fn render_table<'a>(sort_field: &str, proc_list_state: &TableState,searchedProc: String) -> Table<'a> {
    let mut proc_list: Vec<Proc> = collect_proc_data().expect("can fetch proc data");
    proc_list = sort(sort_field, proc_list);
    let mut rows = Vec::new();
    let mut selected_process = proc_list
        .get(
            proc_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();
    
    if searchedProc!=""
    {
        for proc in proc_list {
            if searchedProc == proc.name.to_string(){
            let row = Row::new(vec![
                Cell::from(Span::raw(proc.pid.to_string())),
                Cell::from(Span::raw(proc.name.to_string())),
                Cell::from(Span::raw(proc.state.to_string())),
                Cell::from(Span::raw(proc.parent_id.to_string())),
                Cell::from(Span::raw(proc.priority.to_string())),
                Cell::from(Span::raw(proc.niceness.to_string())),
                Cell::from(Span::raw(proc.user_id.to_string())),
                Cell::from(Span::raw(proc.memory.to_string())),
                Cell::from(Span::raw(proc.cpu_time.to_string())),
                Cell::from(Span::raw(proc.open_files.to_string())),
            ]);
            rows.push(row);
            }
        }

    }
else{
    for proc in proc_list {
        let row = Row::new(vec![
            Cell::from(Span::raw(proc.pid.to_string())),
            Cell::from(Span::raw(proc.name.to_string())),
            Cell::from(Span::raw(proc.state.to_string())),
            Cell::from(Span::raw(proc.parent_id.to_string())),
            Cell::from(Span::raw(proc.priority.to_string())),
            Cell::from(Span::raw(proc.niceness.to_string())),
            Cell::from(Span::raw(proc.user_id.to_string())),
            Cell::from(Span::raw(proc.memory.to_string())),
            Cell::from(Span::raw(proc.cpu_time.to_string())),
            Cell::from(Span::raw(proc.open_files.to_string())),
        ]);
        rows.push(row);
    }
}
    let proc_detail = Table::new(rows)
        .header(Row::new(vec![
            Cell::from(Spans::from(vec![
                Span::styled(
                    "P",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "rocess ID",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "N",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "ame",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "S",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "tate",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "Parent Process ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "I",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "D",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "Priorit",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Y",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "Nice ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "V",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "alue",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "U",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "ser ID",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "M",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "emory (KB)",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "C",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "PU Time",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            Cell::from(Spans::from(vec![
                Span::styled(
                    "F",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                ),
                Span::styled(
                    "iles Opened",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Processes")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(8),  //pid
            Constraint::Percentage(27), //name
            Constraint::Percentage(5),  //state
            Constraint::Percentage(13), //parent_id
            Constraint::Percentage(7),  //priority
            Constraint::Percentage(7),  //niceness
            Constraint::Percentage(5),  //user_id
            Constraint::Percentage(8),  //memory
            Constraint::Percentage(7),  //cpu_time
            Constraint::Percentage(10), //open_files
        ])
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    proc_detail
}
fn render_graphs<'a>() -> Paragraph<'a> {
    let graphs = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Graphs")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Coming soon!")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Graphs")
            .border_type(BorderType::Plain),
    );
    graphs
}
fn sort(sort_field: &str, procs: Vec<Proc>) -> Vec<Proc> {
    let mut sorted_procs = procs.clone();
    match sort_field {
        "pid" => sorted_procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
        "name" => sorted_procs.sort_by(|a, b| a.name.cmp(&b.name)),
        "state" => sorted_procs.sort_by(|a, b| a.state.cmp(&b.state)),
        "parent_id" => sorted_procs.sort_by(|a, b| a.parent_id.cmp(&b.parent_id)),
        "priority" => sorted_procs.sort_by(|a, b| a.priority.cmp(&b.priority)),
        "niceness" => sorted_procs.sort_by(|a, b| a.niceness.cmp(&b.niceness)),
        "user_id" => sorted_procs.sort_by(|a, b| a.user_id.cmp(&b.user_id)),
        "memory" => sorted_procs.sort_by(|a, b| a.memory.cmp(&b.memory)),
        "cpu_time" => sorted_procs.sort_by(|a, b| a.cpu_time.cmp(&b.cpu_time)),
        "open_files" => sorted_procs.sort_by(|a, b| a.open_files.cmp(&b.open_files)),
        "" => sorted_procs.sort_by(|a, b| a.cpu_time.cmp(&b.cpu_time)),
        _ => sorted_procs.sort_by(|a, b| a.cpu_time.cmp(&b.cpu_time)),
    }
    sorted_procs.reverse();
    sorted_procs
}
fn render_sortbar<'a>(active_field: FieldItem) -> (Paragraph<'a>, Tabs<'a>) {
    let sort_opt_bar = Paragraph::new("Sort by: ")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .title("Sort")
                .border_type(BorderType::Plain),
        );
    let options = vec![
        "Process ID",
        "Name",
        "State",
        "Parent Process ID",
        "Priority",
        "Niceness",
        "User ID",
        "Memory",
        "CPU Time",
        "Open Files",
        "Exit",
    ];
    let table_opts: Vec<Spans> = options
        .iter()
        .map(|t: &&str| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let table_opt_tabs = Tabs::new(table_opts)
        .select(active_field.into())
        .block(Block::default().borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow))
        .divider(Span::raw("|"));
    (sort_opt_bar, table_opt_tabs)
}
fn render_filterbar<'a>(input_field: &'a InputField) -> (Paragraph<'a>, Paragraph<'a>) {
    // Creates a Paragraph widget called 'filter_text' which contains the text from the input field
    // and is styled with the default style.
    let filter_text = Paragraph::new(input_field.input.as_ref())
        .style(Style::default())
        // Creates a new block which has borders on the right, top and bottom.
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Plain),
        );
    // Creates a Paragraph widget called 'item' which contains the last message from the input field
    // and is styled with the default style.
    let value = input_field.messages[input_field.messages.len() - 1].clone();
    let item = Paragraph::new(value)
        .style(Style::default())
        // Creates a new block which has borders on the right, top and bottom.
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Plain),
        );
    // Creates a Paragraph widget called 'filter_opt_bar' which contains the text 'Filter field: '
    // and is styled with the default style.
    let filter_opt_bar = Paragraph::new("Filter field: ")
        .style(Style::default().fg(Color::White))
        // Aligns the text to the left
        .alignment(Alignment::Left)
        // Creates a new block which has borders on the left, top and bottom and sets the title to 'Filter'
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .title("Filter")
                .border_type(BorderType::Plain),
        );
    (filter_opt_bar, filter_text)
    // (filter_opt_bar, filter_text)
}
fn render_searchbar<'a>(input_field: &'a InputField ) -> (Paragraph<'a>, Paragraph<'a>,String) {
    //This is to display the text box that expects the user to type in
    let search_text = Paragraph::new(input_field.input.as_ref())
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Plain),
        );
    //This is to display the search field part of the text
    let opt_bar = Paragraph::new("Search field: ")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .title("Search")
                .border_type(BorderType::Plain),
        );
    //The following is a hack to get the last message in the input field --> last searched for
    let value = input_field.messages[input_field.messages.len() - 1].clone();
    let item = Paragraph::new(value).style(Style::default()).block(
        Block::default()
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
            .border_type(BorderType::Plain),
    );
    let searched_proc=input_field.input.clone();
    (opt_bar, search_text,searched_proc)
}
fn render_term_bar<'a>() -> Paragraph<'a> /*, Tabs<'a>*/ {
    let terminate_opt_bar = Paragraph::new("Terminate the process? (y/n)")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .title("Terminate")
                .border_type(BorderType::Plain),
        );
    // (terminate_opt_bar, table_opt_tabs)
    terminate_opt_bar
}
fn render_priority_bar<'a>() -> Paragraph<'a> {
    let set_priority_opt_bar = Paragraph::new("Set priority: ")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::White))
                .title("Set Priority")
                .border_type(BorderType::Plain),
        );

    // (set_priority_opt_bar, table_opt_tabs)
    set_priority_opt_bar
}
fn collect_proc_data() -> Result<Vec<Proc>, Error> {
    let mut all_procs: Vec<Proc> = vec![];
    let processes: Vec<procfs::process::Process> = procfs::process::all_processes().unwrap();
    for process in processes {
        let cpu_time = process.stat.utime + process.stat.stime;
        let cpu_time_secs = Duration::from_secs(cpu_time as u64 / sysconf::page::pagesize() as u64);
        let cputtime_str = format!("{:?}", cpu_time_secs);

        let open_files_count = match process.fd() {
            Ok(open_files) => open_files.len(),
            Err(_) => 0,
        };

        let instproc = Proc {
            pid: process.stat.pid,
            name: process.stat.comm,
            state: process.stat.state,
            parent_id: process.stat.ppid,
            priority: process.stat.priority,
            niceness: process.stat.nice,
            user_id: process.owner,
            memory: process.stat.rss * (sysconf::page::pagesize() as i64) / 1024,
            cpu_time: cputtime_str,
            open_files: open_files_count,
        };
        all_procs.push(instproc);
    }
    Ok(all_procs)
}
fn render_graphtab<'a>() -> Tabs<'a> {
    let graph_options = vec!["Coming Soon", "Quit"];
    let graphs_opt: Vec<Spans> = graph_options
        .iter()
        .map(|t: &&str| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let graphs_opt_tabs = Tabs::new(graphs_opt)
        .block(Block::default().title("Options").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .divider(Span::raw("|"));
    graphs_opt_tabs
}
fn render_tabletab<'a>() -> Tabs<'a> {
    let table_options = vec![
        "Sort",
        "Filter",
        "Search",
        "Terminate Process",
        "Set Priority",
        "Quit",
    ];
    let table_opts: Vec<Spans> = table_options
        .iter()
        .map(|t: &&str| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let table_opt_tabs = Tabs::new(table_opts)
        .block(Block::default().title("Options").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .divider(Span::raw("|"));
    table_opt_tabs
}
fn render_hometab<'a>() -> Tabs<'a> {
    let home_options = vec!["Display Process Table", "Display Graph & Charts", "Quit"];
    let home_opts: Vec<Spans> = home_options
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let home_opt_tabs = Tabs::new(home_opts)
        .block(Block::default().title("Options").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .divider(Span::raw("|"));
    home_opt_tabs
}
fn render_menu<'a>(active_menu_item: MenuItem) -> Tabs<'a> {
    let menu_titles = vec!["Home", "Display Process Table", "Graphs & Charts"];
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let tabs = Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    tabs
}
