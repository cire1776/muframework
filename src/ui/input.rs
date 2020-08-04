use super::*;
use window::InventoryWindowMode;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputState {
    Normal,
    PickupSelection,
    ExternalInventoryOpen,
    Activity,
    DisplayOptions,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MouseState {
    LeftButtonUp,
    LeftButtonDown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Input {
    pub key: Option<VirtualKeyCode>,
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
}

impl Input {
    pub fn new(context: &BTerm) -> Self {
        Self {
            key: context.key,
            shift: context.shift,
            control: context.control,
            alt: context.alt,
        }
    }
}

impl UIState {
    pub fn process_input(&mut self, context: &mut BTerm) {
        let mut input = Input::new(context);

        self.process_mouse_input(context);
        Self::set_modifier_keys(&mut input);

        if input.key != None {
            self.process_keyboard_input(&input);
        }
    }

    fn process_mouse_input(&mut self, context: &mut BTerm) {
        if context.left_click {
            if self.mouse_state == MouseState::LeftButtonUp {
                self.process_left_click(context);
                self.mouse_state = MouseState::LeftButtonDown;
            } else {
                self.mouse_state = MouseState::LeftButtonUp
            }
        }
    }

    fn process_left_click(&mut self, context: &mut BTerm) {
        let mouse_pos = context.mouse_point();

        let mut windows: [Box<&mut dyn MouseReceiver>; 4] = [
            Box::new(&mut self.inventory_window),
            Box::new(&mut self.map_window),
            Box::new(&mut self.message_window),
            Box::new(&mut self.info_window),
        ];
        for window in windows.iter_mut() {
            if window.rect().point_in_rect(mouse_pos) {
                let mouse_point = window.mouse_point(context);

                if window.is_within_frame(mouse_point) {
                    window.handle_left_click(mouse_point.x, mouse_point.y, context);
                }
            }
        }
    }

    fn process_keyboard_input(&mut self, input: &Input) {
        let command = self.get_command_from_keyboard_input(input);

        if command == Command::None {
            return;
        }
        &self.command_tx.send(command);
    }

    /// public for testing purposes.
    pub fn get_command_from_keyboard_input(&mut self, input: &Input) -> Command {
        match self.input_state {
            InputState::Normal => self.process_normal_mode_keyboard_input(input),
            InputState::PickupSelection => self.process_inventory_selection_keyboard_input(input),
            InputState::ExternalInventoryOpen => {
                self.process_external_inventory_selection_keyboard_input(input)
            }
            InputState::Activity => self.process_activity_keyboard_input(input),
            InputState::DisplayOptions => self.process_display_options_input(input),
        }
    }

    fn process_normal_mode_keyboard_input(&mut self, input: &Input) -> Command {
        match input.key {
            None => Command::None,
            Some(key) => match key {
                VirtualKeyCode::Up => {
                    self.convert_key_to_move_command(Direction::Up, Direction::UpRight, input)
                }
                VirtualKeyCode::Down => {
                    self.convert_key_to_move_command(Direction::Down, Direction::DownLeft, input)
                }
                VirtualKeyCode::Left => {
                    self.convert_key_to_move_command(Direction::Left, Direction::UpLeft, input)
                }
                VirtualKeyCode::Right => {
                    self.convert_key_to_move_command(Direction::Right, Direction::DownRight, input)
                }
                VirtualKeyCode::W => {
                    if input.shift {
                        self.map_window.scroll_by(0, 1);
                    }

                    Command::None
                }
                VirtualKeyCode::S => {
                    if input.shift {
                        self.map_window.scroll_by(0, -1);
                    }
                    Command::None
                }
                VirtualKeyCode::A => {
                    if input.shift {
                        self.map_window.scroll_by(1, 0);
                    }
                    Command::None
                }
                VirtualKeyCode::D => self.handle_d(input),
                VirtualKeyCode::T => {
                    if self.are_items_at_player_location() {
                        self.input_state = InputState::PickupSelection;
                    }
                    Command::None
                }

                VirtualKeyCode::E => {
                    if self.inventory_window.window_mode == InventoryWindowMode::Inventory {
                        if self.inventory_window.selected_item == None {
                            Command::None
                        } else {
                            // let item = &self.inventory
                            //     [(self.inventory_window.selected_item.unwrap() - 1) as usize];
                            let item_id =
                                self.inventory_window.get_selected_item_id(&self.inventory);

                            if let Some(item_id) = item_id {
                                Command::EquipItem(item_id)
                            } else {
                                Command::None
                            }
                        }
                    } else {
                        if self.inventory_window.selected_equipment == None {
                            Command::None
                        } else {
                            let possible_item_id =
                                self.inventory_window.get_selected_item_id(&self.equipment);

                            if let Some(item_id) = possible_item_id {
                                Command::UnequipItem(item_id)
                            } else {
                                Command::None
                            }
                        }
                    }
                }
                VirtualKeyCode::R => {
                    if input.shift {
                        Command::RefreshInventory
                    } else {
                        Command::None
                    }
                }
                VirtualKeyCode::Backslash => Command::DumpPlayer,
                VirtualKeyCode::F12 => {
                    if input.shift {
                        Command::LoadGame
                    } else {
                        Command::SaveGame
                    }
                }
                VirtualKeyCode::B => Command::ConstructionSiteBegin,

                _ => Command::None,
            },
        }
    }
    fn are_items_at_player_location(&self) -> bool {
        let items_at_location = self.items.at(self.player.x, self.player.y);
        match items_at_location {
            None => false,
            _ => true,
        }
    }

    fn handle_d(&mut self, input: &Input) -> Command {
        if input.shift {
            self.map_window.scroll_by(-1, 0);
            Command::None
        } else {
            let mut result = Command::None;
            let window = &self.inventory_window;
            if window.window_mode == InventoryWindowMode::Inventory {
                if window.selected_item != None {
                    let item_id = self.inventory_window.get_selected_item_id(&self.inventory);
                    if let Some(item_id) = item_id {
                        result = Command::DropItem(item_id)
                    }
                }
                result
            } else {
                println!("d pressed in equipment pane");
                Command::None
            }
        }
    }
    fn process_inventory_selection_keyboard_input(&mut self, input: &Input) -> Command {
        let command: Command = match input.key {
            Some(VirtualKeyCode::Escape) => {
                self.input_state = InputState::Normal;
                Command::None
            }
            Some(key) if key >= VirtualKeyCode::A && key <= VirtualKeyCode::Z => {
                let item_position = key as u64 - VirtualKeyCode::A as u64 + 1;
                self.input_state = InputState::Normal;
                Command::TakeItem(item_position)
            }
            _ => Command::None,
        };

        command
    }

    fn process_external_inventory_selection_keyboard_input(&mut self, input: &Input) -> Command {
        let inventory_id = self
            .external_inventory_id
            .expect("missing external inventory id");

        let command: Command = match input.key {
            Some(VirtualKeyCode::Escape) => Command::CloseExternalInventory,
            Some(VirtualKeyCode::T) => {
                if let Some(selection) = self.map_window.active_pane().unwrap().selection {
                    let item = if let Some(ref external_inventory) = self.external_inventory {
                        &external_inventory[(selection - 1) as usize]
                    } else {
                        panic!("inventory not found");
                    };

                    Command::TransferItem(item.id, inventory_id, self.player.inventory_id)
                } else {
                    Command::None
                }
            }
            Some(VirtualKeyCode::D) => {
                if let Some(selection) = self.inventory_window.selected_item {
                    let item = &self.inventory[(selection - 1) as usize];
                    Command::TransferItem(item.id, self.player.inventory_id, inventory_id)
                } else {
                    Command::None
                }
            }
            Some(VirtualKeyCode::A) => {
                if input.shift {
                    Command::TransferAllItems(self.player.inventory_id, inventory_id)
                } else {
                    Command::TransferAllItems(inventory_id, self.player.inventory_id)
                }
            }
            _ => Command::None,
        };

        command
    }

    fn process_activity_keyboard_input(&mut self, input: &Input) -> Command {
        let command: Command = match input.key {
            Some(VirtualKeyCode::Escape) => Command::ActivityAbort,
            Some(VirtualKeyCode::F1) => Command::ActivityShortCircuit,
            Some(VirtualKeyCode::F12) => Command::SaveGame,
            _ => Command::None,
        };

        command
    }

    fn process_display_options_input(&mut self, input: &Input) -> Command {
        let command: Command = match input.key {
            Some(VirtualKeyCode::Escape) => {
                self.input_state = InputState::Normal;
                self.map_window.window_mode = MapWindowMode::Normal;
                self.map_window.active_pane = None;
                Command::None
            }
            Some(VirtualKeyCode::Return) => {
                self.input_state = InputState::Normal;
                self.map_window.window_mode = MapWindowMode::Normal;
                let command = Command::ChoiceSelected(
                    self.map_window.active_pane.unwrap().selection.unwrap(),
                    self.continuation.unwrap(),
                    self.continuation_ref.unwrap(),
                );
                self.map_window.active_pane = None;
                command
            }
            _ => Command::None,
        };
        command
    }

    pub fn set_modifier_keys(input: &mut Input) {
        let lock = INPUT.lock();
        input.shift = lock.is_key_pressed(VirtualKeyCode::LShift)
            || lock.is_key_pressed(VirtualKeyCode::RShift);

        input.control = lock.is_key_pressed(VirtualKeyCode::LControl)
            || lock.is_key_pressed(VirtualKeyCode::RControl);

        input.alt =
            lock.is_key_pressed(VirtualKeyCode::LAlt) || lock.is_key_pressed(VirtualKeyCode::RAlt);
    }

    pub fn convert_key_to_move_command(
        &self,
        base_dir: Direction,
        alt_dir: Direction,
        input: &Input,
    ) -> Command {
        let mode: MoveCommandMode;
        if input.control {
            if input.shift {
                mode = MoveCommandMode::SneakUse
            } else {
                mode = MoveCommandMode::Use
            }
        } else if input.shift {
            mode = MoveCommandMode::Sneak
        } else {
            mode = MoveCommandMode::Normal
        }

        if input.alt {
            Command::Move(alt_dir, mode)
        } else {
            Command::Move(base_dir, mode)
        }
    }
}

#[cfg(test)]
mod normal_input_mode {
    use super::*;

    #[test]
    fn arrows_produce_appropriate_move_command() {
        let (_update_tx, update_rx) = mpsc::channel();
        let (command_tx, _command_rx) = mpsc::channel();
        let mut subject = UIState::new(update_rx, command_tx);

        let keys = [
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
        ];
        let directions = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];

        for (i, key) in keys.iter().enumerate() {
            let mut input = super::Input {
                key: Some(*key),
                shift: false,
                control: false,
                alt: false,
            };

            let command = subject.get_command_from_keyboard_input(&mut input);

            if let Command::Move(direction, mode) = command {
                assert_eq!(direction, directions[i]);
                assert_eq!(mode, MoveCommandMode::Normal)
            } else {
                panic!("wrong command returned")
            }
        }
    }
    #[test]
    fn tilted_arrows_produce_appropriate_move_command() {
        let (_update_tx, update_rx) = mpsc::channel();
        let (command_tx, _command_rx) = mpsc::channel();
        let mut subject = UIState::new(update_rx, command_tx);

        let keys = [
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
        ];
        let directions = [
            Direction::UpLeft,
            Direction::DownRight,
            Direction::UpRight,
            Direction::DownLeft,
        ];

        for (i, key) in keys.iter().enumerate() {
            let mut input = super::Input {
                key: Some(*key),
                shift: false,
                control: false,
                alt: true,
            };

            let command = subject.get_command_from_keyboard_input(&mut input);

            if let Command::Move(direction, mode) = command {
                assert_eq!(direction, directions[i]);
                assert_eq!(mode, MoveCommandMode::Normal)
            } else {
                panic!("wrong command returned")
            }
        }
    }

    #[test]
    fn control_arrows_produce_appropriate_use_command() {
        let (_update_tx, update_rx) = mpsc::channel();
        let (command_tx, _command_rx) = mpsc::channel();
        let mut subject = UIState::new(update_rx, command_tx);

        let keys = [
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
        ];
        let directions = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];

        for (i, key) in keys.iter().enumerate() {
            let mut input = super::Input {
                key: Some(*key),
                shift: false,
                control: true,
                alt: false,
            };

            let command = subject.get_command_from_keyboard_input(&mut input);

            if let Command::Move(direction, mode) = command {
                assert_eq!(direction, directions[i]);
                assert_eq!(mode, MoveCommandMode::Use)
            } else {
                panic!("wrong command returned")
            }
        }
    }

    #[test]
    fn tilted_control_arrow_produce_appropriate_use_command() {
        let (_update_tx, update_rx) = mpsc::channel();
        let (command_tx, _command_rx) = mpsc::channel();
        let mut subject = UIState::new(update_rx, command_tx);

        let keys = [
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
        ];
        let directions = [
            Direction::UpLeft,
            Direction::DownRight,
            Direction::UpRight,
            Direction::DownLeft,
        ];

        for (i, key) in keys.iter().enumerate() {
            let mut input = super::Input {
                key: Some(*key),
                shift: false,
                control: true,
                alt: true,
            };

            let command = subject.get_command_from_keyboard_input(&mut input);

            if let Command::Move(direction, mode) = command {
                assert_eq!(direction, directions[i]);
                assert_eq!(mode, MoveCommandMode::Use)
            } else {
                panic!("wrong command returned")
            }
        }
    }
}
