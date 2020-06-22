use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TreeUse {
    Picking,
    Logging,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TreeType {
    Apple,
    Oak,
    Olive,
    Pine,
}

impl TreeType {
    #[inline]
    pub fn from_facility_class(class: FacilityClass) -> TreeType {
        match class {
            FacilityClass::AppleTree => TreeType::Apple,
            FacilityClass::OliveTree => TreeType::Olive,
            _ => panic!("{:?} is not a recognized tree", class),
        }
    }
}

pub struct ActivateTreePickingCommand<'a> {
    tree_type: TreeType,
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateTreePickingCommand<'a> {
    pub fn new(tree_type: TreeType, player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            tree_type,
            player,
            facility_id,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
            && player.is_endorsed_with(":can_pick")
            && match facility.class {
                FacilityClass::AppleTree | FacilityClass::OliveTree => true,
                _ => false,
            }
    }
}

impl<'a> CommandHandler<'a> for ActivateTreePickingCommand<'a> {
    fn expiration(&self) -> u32 {
        match self.tree_type {
            TreeType::Apple => 60,
            TreeType::Olive => 90,
            _ => panic!("Non-fruit tree supplied"),
        }
    }
    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = TreePickingActivity::new(
            self.tree_type,
            self.expiration(),
            self.player.inventory_id(),
            self.facility_id,
            timer,
            guard,
            update_sender,
            command_sender,
        );
        Some(Box::new(activity))
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }
        activity
    }
}

#[allow(dead_code)]
pub struct TreePickingActivity {
    tree_type: TreeType,
    expiration: u32,
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> TreePickingActivity {
    pub fn new(
        tree_type: TreeType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            tree_type,
            expiration,
            player_inventory_id,
            facility_id,
            timer,
            guard: Some(guard),
            update_sender,
            command_sender,
        }
    }
}

impl<'a> Activity for TreePickingActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        let title = match self.tree_type {
            TreeType::Apple => ui::pane::PaneTitle::PickingApples,
            TreeType::Olive => ui::pane::PaneTitle::PickingOlives,
            _ => panic!("Non-fruit tree specified"),
        };

        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(self.expiration * 1000, title),
        );
    }

    fn complete(
        &mut self,
        facilities: &mut FacilityList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
    ) {
        let facility = facilities
            .get_mut(self.facility_id)
            .expect("can't find facility");

        self.on_completion(
            self.player_inventory_id,
            facility,
            items,
            inventories,
            &self.update_sender,
            &self.command_sender,
        );
    }

    fn on_completion(
        &self,
        player_inventory_id: u64,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        let item_class: ItemClass;
        let item_description: &str;

        match self.tree_type {
            TreeType::Apple => {
                item_class = ItemClass::Food;
                item_description = "Apple"
            }
            TreeType::Olive => {
                item_class = ItemClass::Food;
                item_description = "Olive"
            }
            _ => panic!("{:?} is not a fruit tree.", self.tree_type),
        }

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, item_class, item_description.into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        if facility.decrement_property("fruit") <= 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

pub struct ActivateTreeLoggingCommand<'a> {
    tree_type: TreeType,
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateTreeLoggingCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a mut FacilityList) -> Self {
        let facility = facilities
            .get(facility_id)
            .expect("unable to find facility");

        let tree_type = match facility.class {
            FacilityClass::AppleTree => TreeType::Apple,
            FacilityClass::OliveTree => TreeType::Olive,
            FacilityClass::PineTree => TreeType::Pine,
            FacilityClass::OakTree => TreeType::Oak,
            _ => panic!("unknown tree type"),
        };

        Self {
            tree_type,
            player,
            facility_id,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":can_chop")
    }
}

impl<'a> CommandHandler<'a> for ActivateTreeLoggingCommand<'a> {
    fn expiration(&self) -> u32 {
        60
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let command_sender = command_sender.clone();
        let update_sender = update_sender.clone();

        let activity = TreeLoggingActivity::new(
            self.tree_type,
            self.player.inventory_id(),
            self.facility_id,
            timer,
            guard,
            update_sender,
            command_sender,
        );
        Some(Box::new(activity))
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }

        activity
    }
}

#[allow(dead_code)]
pub struct TreeLoggingActivity {
    tree_type: TreeType,
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> TreeLoggingActivity {
    pub fn new(
        tree_type: TreeType,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            tree_type,
            player_inventory_id,
            facility_id,
            timer,
            guard: Some(guard),
            update_sender,
            command_sender,
        }
    }
}

impl<'a> Activity for TreeLoggingActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(60000, ui::pane::PaneTitle::Logging),
        );
    }

    fn complete(
        &mut self,
        facilities: &mut FacilityList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
    ) {
        let facility = facilities
            .get_mut(self.facility_id)
            .expect("can't find facility");

        self.on_completion(
            self.player_inventory_id,
            facility,
            items,
            inventories,
            &self.update_sender,
            &self.command_sender,
        );
    }

    fn on_completion(
        &self,
        player_inventory_id: u64,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        #[allow(unreachable_patterns)]
        let wood_type = match self.tree_type {
            TreeType::Apple | TreeType::Olive | TreeType::Oak => "Hardwood Log",
            TreeType::Pine => "Softwood Log",
            _ => panic!("unknown tree type"),
        };

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, ItemClass::Material, wood_type.into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        if facility.decrement_property("logs") == 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}
