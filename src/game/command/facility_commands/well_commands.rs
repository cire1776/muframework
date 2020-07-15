use super::*;
use WellType::*;

pub struct ActivateWellFillCommand<'a> {
    product: WellType,
    player: &'a mut Player,
    facility_id: u64,
    timer: &'a mut Timer,
}

impl<'a> ActivateWellFillCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &FacilityList,
        timer: &'a mut Timer,
    ) -> Self {
        let facility = facilities.get(facility_id).expect("unable to get well");

        let well_type = if CookingFillingSkill::can_produce(player, facility) {
            Water
        } else {
            Oil
        };

        Self {
            product: well_type,
            player,
            facility_id,
            timer,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
            && (CookingFillingSkill::can_produce(player, facility)
                || AlchemyFillingSkill::can_produce(player, facility))
            && player.is_endorsed_with(":can_fill")
    }
}

impl<'a> CommandHandler<'a> for ActivateWellFillCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        if self.product == Water {
            CookingFillingSkill::expiration(self.player)
        } else {
            AlchemyFillingSkill::expiration(self.player)
        }
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
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let (class, description) = if CookingFillingSkill::can_produce(player, facility) {
            CookingFillingSkill::consume_from_inventory_for(player, inventories, items);
            CookingFillingSkill::produce_results_for(player, facility, rng, Some(&update_sender))
        } else {
            AlchemyFillingSkill::consume_from_inventory_for(player, inventories, items);
            AlchemyFillingSkill::produce_results_for(player, facility, rng, Some(&update_sender))
        };

        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(player.inventory_id(), class, description),
        );

        if !CookingFillingSkill::can_produce(player, facility)
            && !AlchemyFillingSkill::can_produce(player, facility)
        {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

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
        EngineeringSkill::expiration(self.player)
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

        let level = self.player.get_level_for(Engineering);

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
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let result =
            EngineeringSkill::produce_results_for(player, facility, rng, Some(update_sender));

        if result != Dry {
            facility.set_property("fluid", result as u128);
            Command::send(Some(command_sender.clone()), Command::ActivityAbort);
        }

        let level = player.get_level_for(Engineering);

        if !EngineeringSkill::can_produce(level, facility) {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::DontRefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
