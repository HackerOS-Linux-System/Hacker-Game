use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::fs;
use crate::resources::TerminalState;
use crate::states::AppState;

pub fn terminal_system(mut contexts: EguiContexts, mut terminal: ResMut<TerminalState>, mut state: ResMut<NextState<AppState>>) {
    egui::Window::new("HackerOS").show(contexts.ctx_mut(), |ui| {
        ui.text_edit_singleline(&mut terminal.input);
        for line in &terminal.output {
            ui.label(line);
        }
        if ui.button("Execute").clicked() {
            let cmd = terminal.input.clone();
            terminal.output.push(format!("> {}", cmd));
            match cmd.as_str() {
                "scan" => terminal.output.push("Znaleziono: firewall_01, terminal_02".into()),
                "edit config" => {
                    if let Ok(config) = fs::read_to_string("config/hacker_os.conf") {
                        terminal.output.push(format!("Config: {}", config));
                    }
                    // Edycja: zapisz zmiany
                    fs::write("config/hacker_os.conf", "theme=neon_green").unwrap();
                },
                "hack" => state.set(AppState::Hacking),
                _ => terminal.output.push("Nieznana komenda".into()),
            }
            terminal.input.clear();
        }
    });
}
