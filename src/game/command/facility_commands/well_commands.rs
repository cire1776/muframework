use super::*;
use WellType::*;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WellType {
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
    timer: &'a mut Timer,
}

impl<'a> ActivateWellFillCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, timer: &'a mut Timer) -> Self {
        Self {
            player,
            facility_id,
            timer,
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
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (30 + self
            .player
            .get_attribute(Attribute::SkillTime(Engineering), 0)) as u32
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = WellFillActivity::new(
            self.expiration(),
            self.player.id,
            self.facility_id,
            Some(guard),
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
    guard: Option<Guard>,
}

impl WellFillActivity {
    fn new(
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            guard,
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
        player: &mut Player,
        facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to find inventory");

        if !inventory.any_left_after_consuming(ItemClass::Material, "Glass Bottle", 1, items) {
            Command::send(Some(command_sender.clone()), Command::ActivityAbort);
        }

        let fluid = WellType::from(facility.get_property("fluid")).to_string();

        Command::send(
            Some(command_sender),
            Command::SpawnItem(
                player.inventory_id(),
                if fluid == "Water" {
                    ItemClass::Ingredient
                } else {
                    ItemClass::Material
                },
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
    facilities: &'a mut FacilityList,
    timer: &'a mut Timer,
}

impl<'a> ActivateWellDigCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        timer: &'a mut Timer,
    ) -> Self {
        Self {
            player,
            facility_id,
            facilities,
            timer,
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
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (60 + self
            .player
            .get_attribute(Attribute::SkillTime(Engineering), 0)) as u32
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = WellDigActivity::new(
            self.expiration(),
            self.player.id,
            self.facility_id,
            Some(guard),
        );

        let facility = self
            .facilities
            .get(self.facility_id)
            .expect("unable to get facility.");

        let level = self.player.get_attribute(Attribute::SkillLevel(Mining), 0) as u8;

        if EngineeringSkill::can_produce(level, facility) {
            Some(Box::new(activity))
        } else {
            None
        }
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
    guard: Option<Guard>,
}

impl WellDigActivity {
    fn new(
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            guard,
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
        player: &mut Player,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let result = EngineeringSkill::produce_results_for(player, facility, rng);

        if result != Dry {
            facility.set_property("fluid", result as u128);
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::DontRefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
