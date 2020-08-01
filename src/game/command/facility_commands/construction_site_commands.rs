use super::*;

use skills::construction::ConstructionSiteType::*;

pub struct ActivateConstructionSiteBuildCommand<'a> {
    player: &'a mut Player,
    selection: u8,
    inventory: &'a mut Inventory,
    timer: &'a mut Timer,
}

impl<'a> ActivateConstructionSiteBuildCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        selection: u8,
        inventories: &'a mut InventoryList,
        timer: &'a mut Timer,
    ) -> Self {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to get inventory");

        Self {
            player,
            selection,
            inventory,
            timer,
        }
    }

    pub fn can_perform(selection: u8, player: &Player, inventory: &Inventory) -> bool {
        let level = player.get_level_for(Construction);
        ConstructionBuildSiteSkill::can_produce(selection, level, inventory)
    }
}

impl<'a> CommandHandler<'a> for ActivateConstructionSiteBuildCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        ConstructionBuildSiteSkill::expiration(self.selection, self.player)
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity =
            ConstructionSiteBuildActivity::new(self.expiration(), self.selection, Some(guard));

        let level = self.player.get_level_for(Construction);

        if ConstructionBuildSiteSkill::can_produce(self.selection, level, self.inventory) {
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

pub struct ConstructionSiteBuildActivity {
    expiration: u32,
    site_type: ConstructionSiteType,
    guard: Option<Guard>,
}

impl ConstructionSiteBuildActivity {
    fn new(expiration: u32, selection: u8, guard: Option<Guard>) -> Self {
        let site_type = match selection {
            2 => MediumSite,
            3 => LargeSite,
            _ => SmallSite,
        };

        Self {
            expiration,
            site_type,
            guard,
        }
    }
}

impl Activity for ConstructionSiteBuildActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Building
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn facility_id(&self) -> u64 {
        0
    }

    fn on_completion(
        &self,
        player: &mut Player,
        _facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        Command::send(Some(command_sender.clone()), Command::ActivityAbort);

        ConstructionBuildSiteSkill::consume_from_inventory_for(
            self.site_type,
            player,
            inventories,
            items,
        );

        ConstructionBuildSiteSkill::produce_results_for(
            self.site_type,
            player,
            rng,
            Some(&update_sender),
            Some(command_sender),
        );

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}

pub struct ActivateSetConstructionSiteCommand<'a> {
    player: &'a mut Player,
    facility: &'a mut Facility,
}

impl<'a> ActivateSetConstructionSiteCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a mut FacilityList) -> Self {
        let facility = facilities
            .get_mut(facility_id)
            .expect("unable to find facility");

        Self { player, facility }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        player.get_level_for(Construction) >= 9 && facility.get_property("blueprint") == 0
    }
}

impl<'a> CommandHandler<'a> for ActivateSetConstructionSiteCommand<'a> {
    fn perform_execute(&mut self) -> Option<Box<dyn Activity>> {
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        let blueprints =
            ConstructionBuildSiteSkill::available_blueprints_for(self.player, self.facility);

        if blueprints.is_empty() {
            return activity;
        }

        GameUpdate::send(
            Some(update_tx),
            GameUpdate::DisplayOptions(
                blueprints,
                ActionContinuation::SetConstructionSite,
                self.facility.id,
            ),
        );
        activity
    }
}

pub struct ActivateConstructionSiteAddCommand<'a> {
    player: &'a mut Player,
    facility: &'a mut Facility,
    timer: &'a mut Timer,
}

impl<'a> ActivateConstructionSiteAddCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        timer: &'a mut Timer,
    ) -> Self {
        let facility = facilities
            .get_mut(facility_id)
            .expect("unable to find facility");

        Self {
            player,
            facility,
            timer,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        ConstructionAddSkill::can_produce(player, facility)
    }
}

impl<'a> CommandHandler<'a> for ActivateConstructionSiteAddCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        ConstructionAddSkill::expiration(self.player)
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let level = self.player.get_level_for(Construction);

        let activity = ConstructionAddActivity::new(
            self.expiration(),
            self.player.id,
            level as u8,
            self.facility.id,
            Some(guard),
        );

        if ConstructionAddSkill::can_produce(self.player, self.facility) {
            Some(Box::new(activity))
        } else {
            None
        }
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

pub struct ConstructionAddActivity {
    expiration: u32,
    _player_inventory_id: u64,
    _player_level: u8,
    facility_id: u64,
    guard: Option<Guard>,
}

impl ConstructionAddActivity {
    fn new(
        expiration: u32,
        player_inventory_id: u64,
        player_level: u8,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            expiration,
            _player_inventory_id: player_inventory_id,
            _player_level: player_level,
            facility_id,
            guard,
        }
    }
}

impl Activity for ConstructionAddActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Building
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
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        ConstructionAddSkill::consume_from_inventory_for(player, facility, inventories, items);

        ConstructionAddSkill::produce_results_for(
            player,
            facility,
            inventories,
            rng,
            Some(update_sender),
            Some(command_sender.clone()),
        );

        if !ConstructionAddSkill::can_produce(player, facility) {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
