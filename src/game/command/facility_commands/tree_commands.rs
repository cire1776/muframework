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
    product: ProduceType,
    player: &'a mut Player,
    facility_id: u64,
    timer: &'a mut Timer,
}

impl<'a> ActivateTreePickingCommand<'a> {
    pub fn new(
        tree_type: TreeType,
        player: &'a mut Player,
        facility_id: u64,
        timer: &'a mut Timer,
    ) -> Self {
        let product = match tree_type {
            TreeType::Apple => ProduceType::Apple,
            TreeType::Olive => ProduceType::Olive,
            _ => panic!("{:?} is not a fruit tree.", tree_type),
        };

        Self {
            product,
            player,
            facility_id,
            timer,
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
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        HarvestingSkill::expiration(self.product, self.player)
    }
    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = TreePickingActivity::new(
            self.product,
            self.expiration(),
            self.player.inventory_id(),
            self.facility_id,
            guard,
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
    product: ProduceType,
    expiration: u32,
    player_inventory_id: u64,
    facility_id: u64,
    guard: Option<Guard>,
}

impl<'a> TreePickingActivity {
    pub fn new(
        product: ProduceType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Guard,
    ) -> Self {
        Self {
            product,
            expiration,
            player_inventory_id,
            facility_id,
            guard: Some(guard),
        }
    }
}

impl<'a> Activity for TreePickingActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        match self.product {
            ProduceType::Apple => ui::pane::PaneTitle::PickingApples,
            ProduceType::Olive => ui::pane::PaneTitle::PickingOlives,
            _ => panic!("Non-fruit tree specified"),
        }
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn on_completion(
        &self,
        player: &mut Player,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let (class, description) = HarvestingSkill::produce_results_for(
            self.product,
            player,
            facility,
            rng,
            Some(update_sender),
        );
        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(player.inventory_id(), class, description),
        );

        if HarvestingSkill::is_exhasuted(facility) {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

pub struct ActivateTreeLoggingCommand<'a> {
    tree_type: TreeType,
    player: &'a mut Player,
    facility_id: u64,
    timer: &'a mut Timer,
}

impl<'a> ActivateTreeLoggingCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        timer: &'a mut Timer,
    ) -> Self {
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
            timer,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && LoggingSkill::can_produce(player, facility)
    }
}

impl<'a> CommandHandler<'a> for ActivateTreeLoggingCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        LoggingSkill::expiration(self.tree_type, self.player)
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = TreeLoggingActivity::new(
            self.tree_type,
            self.player.inventory_id(),
            self.expiration(),
            self.facility_id,
            guard,
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
    expiration: u32,
    facility_id: u64,
    guard: Option<Guard>,
}

impl<'a> TreeLoggingActivity {
    pub fn new(
        tree_type: TreeType,
        player_inventory_id: u64,
        expiration: u32,
        facility_id: u64,
        guard: Guard,
    ) -> Self {
        Self {
            tree_type,
            player_inventory_id,
            expiration,
            facility_id,
            guard: Some(guard),
        }
    }
}

impl<'a> Activity for TreeLoggingActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Logging
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn on_completion(
        &self,
        player: &mut Player,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        #[allow(unreachable_patterns)]
        let (class, description) =
            LoggingSkill::produce_results_for(self.tree_type, player, rng, Some(update_sender));

        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(player.inventory_id(), class, description),
        );

        if facility.decrement_property("logs") <= 0 {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}
