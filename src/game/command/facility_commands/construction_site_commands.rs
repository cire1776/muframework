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
