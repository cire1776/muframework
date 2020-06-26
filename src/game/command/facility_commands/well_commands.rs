use super::*;
use WellType::*;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WellType {
    Dry = 0,
    Water,
    Oil,
    Bedrock = 255,
}

impl ToString for WellType {
    fn to_string(&self) -> String {
        match self {
            Dry => "Dry",
            Water => "Water",
            Oil => "Oil",
            Bedrock => "Bedrock",
        }
        .to_string()
    }
}

impl WellType {
    pub fn from(value: i128) -> WellType {
        match value {
            0 => Dry,
            1 => Water,
            2 => Oil,
            255 => Bedrock,
            _ => panic!("unknown well type"),
        }
    }
}

pub struct ActivateWellFillCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateWellFillCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            player,
            facility_id,
        }
    }

    pub fn has_fluid(facility: &Facility) -> bool {
        facility.get_property("fluid") == Water as i128
            || facility.get_property("fluid") == Oil as i128
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && Self::has_fluid(&facility) && player.is_endorsed_with(":can_fill")
    }
}

impl<'a> CommandHandler<'a> for ActivateWellFillCommand<'a> {
    fn expiration(&self) -> u32 {
        (30 + self
            .player
            .get_attribute(Attribute::SkillTime("engineering".into()), 0)) as u32
    }

    fn create_activity(
        &self,
        timer: extern_timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = WellFillActivity::new(
            self.expiration(),
            self.player.id,
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );
        Some(Box::new(activity))
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }
        activity
    }
}

pub struct WellFillActivity {
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: extern_timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl WellFillActivity {
    fn new(
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: extern_timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            _timer: timer,
            guard,
            _update_sender: update_sender,
            _command_sender: command_sender,
        }
    }
}

impl Activity for WellFillActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Filling
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn on_completion(
        &self,
        player_inventory_id: u64,
        facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        let inventory = inventories
            .get_mut(&player_inventory_id)
            .expect("unable to find inventory");

        if !inventory.any_left_after_consuming(ItemClass::Material, "Glass Bottle", 1, items) {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        let fluid = WellType::from(facility.get_property("fluid")).to_string();

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Material,
                format!("Bottle of {}", fluid),
            ),
        );

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}

pub struct ActivateWellDigCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateWellDigCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            player,
            facility_id,
        }
    }

    pub fn is_dry(facility: &Facility) -> bool {
        facility.get_property("fluid") == Dry as i128
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && Self::is_dry(&facility) && player.is_endorsed_with(":can_dig")
    }
}

impl<'a> CommandHandler<'a> for ActivateWellDigCommand<'a> {
    fn expiration(&self) -> u32 {
        (60 + self
            .player
            .get_attribute(Attribute::SkillTime("engineering".into()), 0)) as u32
    }

    fn create_activity(
        &self,
        timer: extern_timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = WellDigActivity::new(
            self.expiration(),
            self.player.id,
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );
        Some(Box::new(activity))
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }
        activity
    }
}

pub struct WellDigActivity {
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: extern_timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl WellDigActivity {
    fn new(
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: extern_timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            _timer: timer,
            guard,
            _update_sender: update_sender,
            _command_sender: command_sender,
        }
    }
}

impl Activity for WellDigActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Digging
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn on_completion(
        &self,
        _player_inventory_id: u64,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        facility.increment_property("depth");

        let water_chance = facility.get_property("chance_of_hitting_water");
        let random_result1 = rng.gen_range(0, water_chance);
        if random_result1 == 0 {
            facility.set_property("fluid", WellType::Water as u128);
            Command::send(Some(&command_sender), Command::ActivityAbort);
            return RefreshInventoryFlag::DontRefreshInventory;
        }

        let oil_chance = facility.get_property("chance_of_hitting_oil");
        let random_result2 = rng.gen_range(0, oil_chance);

        if random_result2 == 0 {
            facility.set_property("fluid", WellType::Oil as u128);
            Command::send(Some(&command_sender), Command::ActivityAbort);
            return RefreshInventoryFlag::DontRefreshInventory;
        }

        RefreshInventoryFlag::DontRefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
