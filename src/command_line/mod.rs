mod goto;
mod parse;
mod spawn;
use parse::parse;

use bevy::prelude::*;
use bevy_egui::egui;

use crate::objects::player::Player;

#[derive(Resource)]
pub struct CommandLineUiState {
    command: String,
}

pub fn show_command_line_ui(
    commands: &mut Commands,
    q_player: &mut Query<&mut Transform, With<Player>>,
    ui: &mut egui::Ui,
    state: &mut CommandLineUiState,
) {
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::multiline(&mut state.command)
                .code_editor()
                .hint_text("enter command")
                .desired_rows(1),
        );
        if ui.button("run").clicked() {
            match parse(&state.command) {
                Some(("spawn", rest)) => spawn::spawn(commands, rest),
                Some(("goto", rest)) => goto::goto(q_player, rest),
                Some((command, _)) => println!("command {:?} does not exist", command),
                None => println!("invalid command format"),
            }
        }
    });
    ui.separator();
}

pub struct CommandLinePlugin;

impl Plugin for CommandLinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommandLineUiState {
            command: String::new(),
        });
    }
}
