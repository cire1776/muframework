use super::*;
use VeinType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum VeinType {
    Dirt = 1,
    Sand,
    Stone,
    Tin,
    Copper,
}

impl ToString for VeinType {
    fn to_string(&self) -> String {
        match self {
            Dirt => "Dirt".into(),
            Sand => "Sand".into(),
            Stone => "Stone".into(),
            Tin => "Tin".into(),
            Copper => "Copper".into(),
        }
    }
}

impl VeinType {
    pub fn from(value: i128) -> Self {
        match value {
            1 => Dirt,
            2 => Sand,
            3 => Stone,
            4 => Tin,
            5 => Copper,
            _ => panic!("unknown vein type"),
        }
    }
}

pub struct ActivateVeinCommand<'a> {
    vein_type: VeinType,
    player: &'a mut Player,
    facility_id: u64,
    timer: &'a mut Timer,
}

impl<'a> ActivateVeinCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &mut FacilityList,
        timer: &'a mut Timer,
    ) -> Self {
        let facility = facilities
            .get(facility_id)
            .expect("unable to locate facility.");

        Self {
            vein_type: Self::determine_vein_type(facility),
            player,
            facility_id,
            timer,
        }
    }

    fn determine_vein_type(facility: &Facility) -> VeinType {
        let ore_type = facility.get_property("ore_type");
        VeinType::from(ore_type)
    }

    fn is_diggable(facility: &Facility) -> bool {
        let ore_type = Self::determine_vein_type(facility);
        ore_type == Dirt || ore_type == Sand
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
            && (player.is_endorsed_with(":can_dig") && Self::is_diggable(facility)
                || (player.is_endorsed_with(":can_mine") && !Self::is_diggable(facility)))
    }
}

impl<'a> CommandHandler<'a> for ActivateVeinCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (match self.vein_type {
            Dirt => 40,
            Sand => 20,
            _ => 60,
        } + self.player.get_attribute(Attribute::SkillTime(Mining), 0)) as u32
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = VeinActivity::new(
            self.vein_type,
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
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }
        activity
    }
}

pub struct VeinActivity {
    vein_type: VeinType,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    guard: Option<Guard>,
}

impl VeinActivity {
    fn new(
        vein_type: VeinType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            vein_type,
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            guard,
        }
    }
}

impl Activity for VeinActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        if self.vein_type == Dirt || self.vein_type == Sand {
            ui::pane::PaneTitle::Digging
        } else {
            ui::pane::PaneTitle::Mining
        }
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
        rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let ore_type = match self.vein_type {
            Dirt => "Dirt",
            Sand => "Sand",
            Stone => "Stone",
            Tin => "Tin Ore",
            Copper => "Copper Ore",
        }
        .to_string();

        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(player.inventory_id(), ItemClass::Ore, ore_type),
        );

        let exhastion_chance = facility.get_property("chance_of_exhaustion");
        if rng.succeeds(0, exhastion_chance, "chance_of_exhaustion") {
            Command::send(
                Some(command_sender.clone()),
                Command::DestroyFacility(facility.id),
            );
            Command::send(Some(command_sender), Command::ActivityAbort);
            return RefreshInventoryFlag::RefreshInventory;
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
