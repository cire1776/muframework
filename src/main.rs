rltk::add_wasm_support!();

extern crate muframework;
use bracket_lib::prelude::*;
use muframework::*;
use std::sync::mpsc;
use std::thread;
use ui::window::BasicWindow;
use ui::UIState;

fn main() -> BError {
    let (update_tx, update_rx) = mpsc::channel();
    let (command_tx, command_rx) = mpsc::channel();

    let cloned_command_tx = command_tx.clone();

    let _game_handle =
        thread::spawn(move || game::GameState::game_loop(update_tx, command_rx, cloned_command_tx));

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
