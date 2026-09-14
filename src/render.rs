pub mod render{
    use std::{error::Error, io::Stdout, ops::{Add, Sub}, sync::{Arc, Mutex}};

use crossterm::{event::{DisableMouseCapture, KeyCode, KeyEvent}, execute, terminal::{LeaveAlternateScreen, disable_raw_mode}};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, style::Style};
use cpal::{Host, HostId};
use crossterm::event::{self, Event};
use ratatui::{widgets::{Block, Borders, List, ListItem}};

use crate::model::Model::{MyColor, expand_to_len, gen_rand_len};
use cpal::traits::HostTrait; 
use std::time::Duration;
use ratatui::{widgets::{Paragraph, Wrap}, style::Color, text::{Line, Span}};


    #[derive(Debug,PartialEq, Eq,Clone, Copy)]
    pub enum ControlSelection{
        Amp_and_Bars,
        Fall_and_Rise
    }

    impl ControlSelection{
        pub const ALL: [ControlSelection;2] = [ControlSelection::Amp_and_Bars,ControlSelection::Fall_and_Rise];

        pub fn next(&self) -> ControlSelection{
            let idx = Self::ALL.iter().position(|x| x == self).unwrap();
            Self::ALL[(idx+1) % Self::ALL.len()]
        }

        pub fn name(&self) -> &'static str{
            match self{
                ControlSelection::Amp_and_Bars => "Amp=⇅,Bars=⇄",
                ControlSelection::Fall_and_Rise => "RiseSpeed=⇅,FallSpeed=⇄",
            }
        }

        pub fn prev(&self) -> ControlSelection{
            let idx = Self::ALL.iter().position(|x| x == self).unwrap();
            Self::ALL[(idx + Self::ALL.len() - 1) % Self::ALL.len()]           
        }
    }

    impl Default for ControlSelection{
        fn default() -> Self {
            Self::Amp_and_Bars
        }
    }





    pub struct App{
        pub bars: usize,   
        pub amp: f32,
        pub colors: Vec<MyColor>,
        pub is_stereo: bool,
        pub spectrum: Vec<f32>,
        pub left_spectrum: Vec<f32>,
        pub right_spectrum: Vec<f32>,
        pub spectrum_peaks: Vec<f32>,
        pub left_spectrum_peaks: Vec<f32>,
        pub right_spectrum_peaks: Vec<f32>,
        pub fall_speed: f32,
        pub rise_speed: f32,
        pub rand: bool,
        pub curr_controls: ControlSelection
    }

    pub struct SelectionState {
        pub items: Vec<String>,
        pub selected: usize,
    }

    impl SelectionState {
        pub fn new(items: Vec<String>) -> Self {
            Self { items, selected: 0 }
        }

        pub fn next(&mut self) {
            if !self.items.is_empty() {
                self.selected = (self.selected + 1) % self.items.len();
            }
        }

        pub fn prev(&mut self) {
            if !self.items.is_empty() {
                self.selected = if self.selected == 0 {
                    self.items.len() - 1
                } else {
                    self.selected - 1
                };
            }
        }

        pub fn get_selected(&self) -> Option<&String> {
            self.items.get(self.selected)
        }
    }

    pub fn select_host(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<HostId, Box<dyn Error>> {        
        let hosts  = cpal::available_hosts().into_iter().collect::<Vec<HostId>>();
        let mut state = SelectionState::new(hosts.iter().map(|h| h.name().to_string()).collect::<Vec<String>>());
        terminal.clear()?;

        loop {
            terminal.draw(|f| {
                let size = f.area();
                let block = Block::default().title("Select audio host").borders(Borders::ALL);
                
                let list_items: Vec<ListItem> = state.items.iter().enumerate().map(|(i, name)| {
                    if i == state.selected {
                        ListItem::new(format!("> {}", name))
                    } else {
                        ListItem::new(format!("  {}", name))
                    }
                }).collect();
                let list = List::new(list_items).block(block).style(Style::default());
                f.render_widget(list, size);
            })?;

            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up | KeyCode::Tab => state.prev(),
                        KeyCode::Down | KeyCode::BackTab => state.next(),
                        KeyCode::Enter => {
                            if let Some(&host_id) = hosts.get(state.selected) {
                                terminal.clear()?;
                                return Ok(host_id);
                            }
                        },
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(&mut terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                            terminal.show_cursor()?;
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn select_device(terminal: &mut Terminal<CrosstermBackend<Stdout>>, host: &Host) -> Result<String, Box<dyn Error>> {
        let mut state = SelectionState::new(host.input_devices()?.map(|d| d.to_string()).collect::<Vec<String>>());
        terminal.clear()?;

        loop {
            terminal.draw(|f| {
                let size = f.area();
                let block = Block::default().title("Select input device").borders(Borders::ALL);  
                let list_items: Vec<ListItem> = state.items.iter().enumerate().map(|(i, name)| {
                    if i == state.selected {
                        ListItem::new(format!("> {}", name))
                    } else {
                        ListItem::new(format!("  {}", name))
                    }
                }).collect();

                let list = ratatui::widgets::List::new(list_items).block(block).style(ratatui::style::Style::default());
                f.render_widget(list, size);
            })?;

            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up | KeyCode::BackTab => state.prev(),
                        KeyCode::Down | KeyCode::Tab => state.next(),
                        KeyCode::Enter => {
                            if let Some(device_name) = state.get_selected() {
                                terminal.clear()?;
                                return Ok(device_name.clone());
                            }
                        },
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(&mut terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                            terminal.show_cursor()?;
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    impl App{
        pub fn new(bars: usize,amp:f32,colors: Vec<MyColor>,is_stereo: bool,rand:bool) -> Self{
            Self { bars, amp, colors, is_stereo,spectrum: vec![0.0; bars],left_spectrum: vec![0.0; bars],right_spectrum: vec![0.0; bars],spectrum_peaks: vec![0.0; bars],left_spectrum_peaks: vec![0.0; bars],right_spectrum_peaks: vec![0.0; bars],fall_speed: 0.15,rise_speed: 0.3,rand,curr_controls:ControlSelection::default()}
        }
    }

    pub fn run_app(terminal:&mut Terminal<CrosstermBackend<Stdout>>,app: Arc<Mutex<App>>) -> Result<(),Box<dyn Error>>{
        let mut last_tick = std::time::Instant::now();
        let tick_rate = Duration::from_millis(8); // 120 FPS

        loop {
            terminal.draw(|f| {
                let app_guard = app.lock().unwrap();
                render_spectrum(f, &app_guard);
            })?;
            let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    let mut app_guard = app.lock().unwrap();
                    handle_input(&mut app_guard, key);
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = std::time::Instant::now();
            }
        }

        disable_raw_mode()?;
        execute!(&mut terminal.backend_mut(),LeaveAlternateScreen,DisableMouseCapture)?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn handle_input(app: &mut App,key: KeyEvent){
        match key.code{
            KeyCode::Left => {
                    match app.curr_controls{
                        ControlSelection::Amp_and_Bars => {
                            app.bars = app.bars.saturating_sub(1);
                            app.spectrum = vec![0.0; app.bars];
                            app.left_spectrum = vec![0.0; app.bars];
                            app.right_spectrum = vec![0.0; app.bars];
                            app.spectrum_peaks = vec![0.0; app.bars];
                            app.left_spectrum_peaks = vec![0.0; app.bars];
                            app.right_spectrum_peaks = vec![0.0; app.bars];
                            if app.rand{
                                app.colors = gen_rand_len(app.bars);
                            }
                            app.colors = expand_to_len(app.colors.clone(), app.bars); 
                        },
                        ControlSelection::Fall_and_Rise => {
                            app.fall_speed = app.fall_speed.sub(0.1).clamp(0.0, 1.0);
                        }
                    }
           },
            KeyCode::Right => {
                    match app.curr_controls{
                        ControlSelection::Amp_and_Bars => {
                            app.bars = app.bars.saturating_add(1);
                            app.spectrum = vec![0.0; app.bars];
                            app.left_spectrum = vec![0.0; app.bars];
                            app.right_spectrum = vec![0.0; app.bars];
                            app.spectrum_peaks = vec![0.0; app.bars];
                            app.left_spectrum_peaks = vec![0.0; app.bars];
                            app.right_spectrum_peaks = vec![0.0; app.bars];
                            if app.rand{
                                app.colors = gen_rand_len(app.bars);
                            }
                            app.colors = expand_to_len(app.colors.clone(), app.bars);
                        },
                        ControlSelection::Fall_and_Rise => {
                            app.fall_speed = app.fall_speed.add(0.1).clamp(0.0, 1.0);
                        }
                    }

            },
            KeyCode::Tab => {
                app.curr_controls = app.curr_controls.next();
            },
            KeyCode::BackTab => {
                app.curr_controls = app.curr_controls.prev();
            },
            KeyCode::Down => {
                match app.curr_controls{
                    ControlSelection::Amp_and_Bars => {
                        app.amp = app.amp.sub(0.1).clamp(0.0, 1.0);
                    },
                    ControlSelection::Fall_and_Rise => {
                        app.rise_speed = app.rise_speed.sub(0.1).clamp(0.0,1.0);
                    }
                }
            },
            KeyCode::Up => {
                match app.curr_controls{
                    ControlSelection::Amp_and_Bars => {
                        app.amp = app.amp.add(0.1).clamp(0.0, 1.0);
                    },
                    ControlSelection::Fall_and_Rise => {
                        app.rise_speed = app.rise_speed.add(0.1).clamp(0.0,1.0);
                    },
                }
            }
            _ => {} 
        }
    }

    fn render_spectrum(f: &mut Frame, app: &App) {
        let size = f.area();
        let bar_width = size.width as usize / app.bars;
        let max_height = size.height as usize - 2;
        let mut lines = Vec::new();
        
        for y in (0..max_height).rev() {
            let mut spans = Vec::new();
            for (bar_idx, &magnitude) in app.spectrum.iter().enumerate() {
                let normalized_height = ((magnitude + (magnitude * app.amp)) as usize).min(max_height);
                let color = app.colors.get(bar_idx).copied().unwrap_or_default();
                
                if y < normalized_height {
                    let bar_char = if bar_width >= 2 { "█" } else { "│" };
                    spans.push(Span::styled(bar_char.repeat(bar_width.max(1)),Style::default().fg(Color::Rgb(color.r, color.g, color.b))));
                } else {
                    spans.push(Span::styled(" ".repeat(bar_width.max(1)), Style::default()));
                }
            }
            lines.push(Line::from(spans));
        }
    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(format!("Controls:{}   Amp({})  Bars({})  FallSpeed({})  RiseSpeed({})",app.curr_controls.name(),app.amp,app.bars,app.fall_speed,app.rise_speed))).wrap(Wrap { trim: false });
    f.render_widget(paragraph, size);
    }
}