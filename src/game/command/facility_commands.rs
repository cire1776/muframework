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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TreeUse {
    Picking,
    Logging,
}

pub struct ActivateTreeCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
    tree_use: TreeUse,
}

impl<'a> ActivateTreeCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64) -> Self {
        let tree_use = ActivateTreeCommand::how_is_player_using(player);

        Self {
            player,
            facility_id,
            tree_use,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
            && (player.is_endorsed_with(":can_pick") || player.is_endorsed_with(":can_chop"))
    }

    fn how_is_player_using(player: &Player) -> TreeUse {
        if player.is_endorsed_with(":can_pick") {
            TreeUse::Picking
        } else {
            if player.is_endorsed_with(":can_chop") {
                TreeUse::Logging
            } else {
                panic!("player must either be picking or logging")
            }
        }
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> Box<dyn Activity> {
        let command_sender = command_sender.clone();
        let update_sender = update_sender.clone();

        let activity = AppleTreeActivity::new(
            self.tree_use,
            self.player.inventory_id(),
            self.facility_id,
            timer,
            guard,
            update_sender,
            command_sender,
        );
        Box::new(activity)
    }
}

impl<'a> CommandHandler for ActivateTreeCommand<'a> {
    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        // currently base timer is the same for picking and logging
        let base_time = 60;

        let guard = timer.schedule_repeating(chrono::Duration::seconds(base_time), move || {
            Command::send(Some(&command_sender), Command::ActivityComplete);
        });

        let activity = self.create_activity(timer, guard, &update_sender, command_tx.unwrap());
        self.player.activity = Some(activity);
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

#[allow(dead_code)]
pub struct AppleTreeActivity {
    tree_use: TreeUse,
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> AppleTreeActivity {
    pub fn new(
        tree_use: TreeUse,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            tree_use,
            player_inventory_id,
            facility_id,
            timer,
            guard: Some(guard),
            update_sender,
            command_sender,
        }
    }
}

impl<'a> Activity for AppleTreeActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        let title = match self.tree_use {
            TreeUse::Logging => ui::pane::PaneTitle::Logging,
            TreeUse::Picking => ui::pane::PaneTitle::PickingApples,
        };

        GameUpdate::send(Some(update_tx), GameUpdate::ActivityStarted(60000, title));
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

        let mut item_class = ItemClass::Food;
        let mut item_description = "Apple";
        let mut item_property = "fruit";

        if self.tree_use == TreeUse::Logging {
            item_class = ItemClass::Material;
            item_description = "Hardwood Logs";
            item_property = "logs"
        }

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, item_class, item_description.into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        let count = facility.get_property(item_property);
        facility.set_property(item_property, count - 1);
        if facility.get_property(item_property) <= 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

#[allow(dead_code)]
pub struct AppleTreeLoggingActivity {
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> AppleTreeLoggingActivity {
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

impl<'a> Activity for AppleTreeLoggingActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(60000, ui::pane::PaneTitle::Logging),
        );
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
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Material,
                "Hardwood Log".into(),
            ),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        let count = facility.get_property("logs");
        facility.set_property("logs", count - 1);
        if facility.get_property("logs") <= 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}
