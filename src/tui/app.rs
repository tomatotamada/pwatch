use crate::port::{self, PortInfo};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Normal,
    Search,
    Confirm {
        force: bool,
    },
}

#[derive(Debug, Clone)]
pub struct App {
    pub ports: Vec<PortInfo>,
    pub selected: usize,
    pub filter: String,
    pub mode: AppMode,
    pub should_quit: bool,
    pub message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let ports = port::scan();
        Self {
            ports,
            selected: 0,
            filter: String::new(),
            mode: AppMode::Normal,
            should_quit: false,
            message: None,
        }
    }

    pub fn refresh(&mut self) {
        self.ports = port::scan();
        if self.selected >= self.ports.len() && !self.ports.is_empty() {
            self.selected = self.ports.len() - 1;
        }
        self.message = Some("リフレッシュしました".to_string());
    }

    pub fn filtered_ports(&self) -> Vec<&PortInfo> {
        if self.filter.is_empty() {
            self.ports.iter().collect()
        } else {
            self.ports
                .iter()
                .filter(|p| {
                    p.port.to_string().contains(&self.filter)
                        || p.process_name.contains(&self.filter)
                        || p.command.contains(&self.filter)
                })
                .collect()
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let len = self.filtered_ports().len();
        if len > 0 && self.selected < len - 1 {
            self.selected += 1;
        }
    }

    pub fn selected_port(&self) -> Option<&PortInfo> {
        self.filtered_ports().get(self.selected).copied()
    }

    pub fn kill_selected(&mut self, force: bool) {
        if let Some(info) = self.selected_port().cloned() {
            match port::kill_process(info.pid, force) {
                Ok(()) => {
                    let sig = if force { "SIGKILL" } else { "SIGTERM" };
                    self.message = Some(format!(
                        "PID {} ({}) に {} を送信しました",
                        info.pid, info.process_name, sig
                    ));
                    self.refresh();
                }
                Err(e) => {
                    self.message = Some(format!("エラー: {}", e));
                }
            }
        }
        self.mode = AppMode::Normal;
    }
}
