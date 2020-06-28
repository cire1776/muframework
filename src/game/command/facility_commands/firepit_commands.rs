use super::*;

pub struct ActivateFirepitCommand<'a> {
    fish_type: FishType,
    player: &'a mut Player,
    facility_id: u64,
    inventories: &'a mut InventoryList,
    timer: &'a mut Timer,
}

impl<'a> ActivateFirepitCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        inventories: &'a mut InventoryList,
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
        (60 + self
            .player
            .get_attribute(Attribute::SkillTime("cooking".into()), 0)) as u32
    }

    fn create_activity(
        &self,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let level = self
            .player
            .get_attribute(Attribute::SkillLevel("cooking".into()), 0);

        let activity = FirepitActivity::new(
            self.fish_type,
            self.expiration(),
            self.player.id,
            level as u8,
            self.facility_id,
            Some(guard),
            update_sender,
            command_sender,
        );

        let inventory = self
            .inventories
            .get(&self.player.id)
            .expect("unable to get inventory.");

        if CookingSkill::can_produce(self.fish_type, level as u8, inventory) {
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
    player_level: u8,
    facility_id: u64,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl FirepitActivity {
    fn new(
        fish_type: FishType,
        expiration: u32,
        player_inventory_id: u64,
        player_level: u8,
        facility_id: u64,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            fish_type,
            expiration,
            _player_inventory_id: player_inventory_id,
            player_level,
            facility_id,
            guard,
            _update_sender: update_sender,
            _command_sender: command_sender,
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
        rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to find inventory");

        CookingSkill::consume_from_inventory_for(self.fish_type, inventory, items);

        let (class, description) = CookingSkill::output_for(self.fish_type, self.player_level, rng);
        Command::send(
            Some(command_sender),
            Command::SpawnItem(player.inventory_id(), class, description),
        );

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
