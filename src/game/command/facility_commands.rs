extern crate chrono;
extern crate timer;

pub use super::*;
pub use std::sync::mpsc::channel;

#[allow(dead_code)]
// TODO: change facility to reflect open chest status.
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

pub struct ActivatePickAppleTreeCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivatePickAppleTreeCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            player,
            facility_id,
        }
    }
}

impl<'a> CommandHandler for ActivatePickAppleTreeCommand<'a> {
    fn can_perform(&self) -> bool {
        self.player.is_endorsed_with(":can_pick_apples")
    }
    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        if !self.can_perform() {
            return;
        }

        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        let guard = timer.schedule_repeating(chrono::Duration::seconds(60), move || {
            Command::send(Some(&command_sender), Command::ActivityComplete);
        });

        let activity = AppleTreePickingActivity::new(
            self.player.inventory_id(),
            self.facility_id,
            timer,
            guard,
            update_sender,
            command_tx.unwrap().clone(),
        );
        self.player.activity = Some(Box::new(activity));
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

#[allow(dead_code)]
pub struct AppleTreePickingActivity {
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> AppleTreePickingActivity {
    pub fn new(
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            player_inventory_id,
            facility_id,
            timer,
            guard: Some(guard),
            update_sender,
            command_sender,
        }
    }
}

impl<'a> Activity for AppleTreePickingActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(Some(update_tx), GameUpdate::ActivityStarted(60000));
    }

    fn complete(&mut self, facilities: &mut FacilityList) {
        let facility = facilities
            .get_mut(self.facility_id)
            .expect("can't find facility");

        self.on_completion(
            self.player_inventory_id,
            facility,
            &self.update_sender,
            &self.command_sender,
        );
    }

    fn on_completion(
        &self,
        player_inventory_id: u64,
        facility: &mut Facility,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, ItemClass::Food, "Apple".into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        let count = facility.get_property("apples");
        facility.set_property("apples", count - 1);
        if facility.get_property("apples") <= 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}
