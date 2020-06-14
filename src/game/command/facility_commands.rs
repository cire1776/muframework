extern crate chrono;
extern crate timer;

pub use super::*;
pub use std::sync::mpsc::channel;

#[derive(Debug)]
pub struct OpenChestCommand<'a> {
    x: i32,
    y: i32,
    player: &'a mut Player,
    external_inventory: &'a Inventory,
    facility_id: u64,
    facilities: &'a FacilityList,
}

impl<'a> OpenChestCommand<'a> {
    pub fn new(
        x: i32,
        y: i32,
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a FacilityList,
        inventories: &'a InventoryList,
    ) -> Self {
        let external_inventory = inventories.get(&facility_id).unwrap();
        Self {
            x,
            y,
            player,
            external_inventory,
            facility_id,
            facilities,
        }
    }
}

impl<'a> CommandHandler for OpenChestCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        self.player.external_inventory = Some(self.external_inventory.to_vec());
    }

    fn announce(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ExternalInventoryOpened(
                self.external_inventory.to_vec(),
                self.external_inventory.id(),
            ),
        );
    }
}

pub struct ActivateAppleTreeCommand<'a> {
    pub game_state: &'a mut game::GameState,
}

impl<'a> ActivateAppleTreeCommand<'a> {
    pub fn new(game_state: &'a mut game::GameState) -> Self {
        Self { game_state }
    }
}

impl<'a> CommandHandler for ActivateAppleTreeCommand<'a> {
    fn can_perform(&self, game_state: &game::GameState) -> bool {
        game_state
            .game_data
            .player
            .is_endorsed_with(":can_pick_apples")
    }
    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        let player_inventory_id = self.game_state.get_player_inventory().id();

        self.game_state.activity_guard = Some(
            timer.schedule_repeating(chrono::Duration::seconds(60), move || {
                Self::complete_activity(player_inventory_id, &update_sender, &command_sender)
            }),
        );
        self.game_state.activity_timer = Some(timer);
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        Self::start_activity(update_tx)
    }
}

impl<'a> ActivateAppleTreeCommand<'a> {
    fn start_activity(update_tx: &GameUpdateSender) {
        GameUpdate::send(Some(update_tx), GameUpdate::ActivityStarted(60000));
    }

    fn complete_activity(
        player_inventory_id: u64,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, ItemClass::Food, "Apple".into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        Self::start_activity(&update_sender);
    }
}
