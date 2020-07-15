rltk::add_wasm_support!();

extern crate muframework;
use bracket_lib::prelude::*;
use muframework::*;
use std::env;
use std::sync::mpsc;
use std::thread;
use ui::window::BasicWindow;
use ui::UIState;

mod shell;

fn main() -> BError {
    let (update_tx, update_rx) = mpsc::channel();
    let (command_tx, command_rx) = mpsc::channel();

    let args: Vec<String> = env::args().collect();
    let auto_save_enabled = args.iter().find(|a| &a[..] == "--no-save").is_none();
    let cloned_command_tx = command_tx.clone();
    let cloned_update_tx = update_tx.clone();
    let _game_handle = thread::spawn(move || {
        game::GameState::game_loop(
            auto_save_enabled,
            cloned_update_tx,
            command_rx,
            cloned_command_tx,
        );
    });

    let cloned_command_tx = command_tx.clone();

    let _shell_handle = thread::spawn(move || loop {
        let cloned_command_tx = cloned_command_tx.clone();
        let handle = thread::spawn(move || {
            shell::shell_loop(cloned_command_tx);
        });

        if handle.join().is_ok() {
            break;
        }
    });

    if !auto_save_enabled {
        GameUpdate::send(
            Some(&update_tx),
            GameUpdate::Message {
                message: "Autosave disabled".to_string(),
                message_type: MessageType::System,
                timestamp: "".to_string(),
            },
        );
    }
    let width = 84;
    let height = 60;

    let context = BTermBuilder::new()
        .with_dimensions(width, height)
        .with_tile_dimensions(16, 16)
        .with_title("MUFramework")
        .with_font("Kai-1280x400bw.png", 16, 16)
        .with_simple_console(width, height, "Kai-1280x400bw.png")
        .build()?;

    let mut ui_state: UIState = UIState::new(update_rx, command_tx);

    ui_state.map_window.scroll_to(4, 0);
    main_loop(context, ui_state)
}
