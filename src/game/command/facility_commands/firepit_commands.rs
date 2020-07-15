use super::*;

pub struct ActivateFirepitCommand<'a> {
    fish_type: FishType,
    player: &'a mut Player,
    facility_id: u64,
    inventories: &'a mut InventoryList,
    items: &'a mut ItemList,
    timer: &'a mut Timer,
}

impl<'a> ActivateFirepitCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        inventories: &'a mut InventoryList,
        items: &'a mut ItemList,
        timer: &'a mut Timer,
    ) -> Self {
        let component = player
            .get_endorsement_component(":wants_to_cook")
            .expect("unable to get component");

        let fish_type = FishType::from_string(component.clone());

        Self {
            fish_type,
            player,
            facility_id,
            inventories,
            items,
            timer,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":wants_to_cook")
    }
}

impl<'a> CommandHandler<'a> for ActivateFirepitCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        CookingSkill::expiration(self.fish_type, self.player)
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let level = self.player.get_level_for(Cooking);

        let activity = FirepitActivity::new(
            self.fish_type,
            self.expiration(),
            self.player.id,
            level as u8,
            self.facility_id,
            Some(guard),
        );

        let inventory = self
            .inventories
            .get(&self.player.id)
            .expect("unable to get inventory.");

        if CookingSkill::can_produce(self.fish_type, self.player, inventory, self.items) {
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

pub struct FirepitActivity {
    fish_type: FishType,
    expiration: u32,
    _player_inventory_id: u64,
    _player_level: u8,
    facility_id: u64,
    guard: Option<Guard>,
}

impl FirepitActivity {
    fn new(
        fish_type: FishType,
        expiration: u32,
        player_inventory_id: u64,
        player_level: u8,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            fish_type,
            expiration,
            _player_inventory_id: player_inventory_id,
            _player_level: player_level,
            facility_id,
            guard,
        }
    }
}

impl Activity for FirepitActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Cooking
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
        _facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _game_data: &mut GameData,
        rng: &mut Rng,
        update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        CookingSkill::consume_from_inventory_for(self.fish_type, player, inventories, items);

        let (class, description) =
            CookingSkill::produce_results_for(self.fish_type, player, rng, Some(update_sender));

        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(player.inventory_id(), class, description),
        );
        let inventory = inventories
            .get(&player.inventory_id())
            .expect("unable to get player inventory.");

        if !CookingSkill::can_produce(self.fish_type, player, inventory, items) {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
