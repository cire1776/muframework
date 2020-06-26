use super::*;

// TODO: change facility to reflect open chest status.
pub struct OpenSmelteryCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
    _facilities: &'a FacilityList,
}

impl<'a> OpenSmelteryCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a FacilityList) -> Self {
        Self {
            player,
            facility_id,
            _facilities: facilities,
        }
    }

    pub fn can_perform(_player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
    }
}

impl<'a> CommandHandler<'a> for OpenSmelteryCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::DisplayOptions(
                SmeltingSkill::products_for_player_level(self.player),
                ActionContinuation::Smeltery,
                self.facility_id,
            ),
        );
        activity
    }
}

pub struct ActivateSmelteryCommand<'a> {
    product: SmeltingSkill,
    player: &'a mut Player,
    inventories: &'a mut InventoryList,
    facility_id: u64,
    timer: &'a mut extern_timer::Timer,
}

impl<'a> ActivateSmelteryCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        product_index: u8,
        inventories: &'a mut InventoryList,
        timer: &'a mut extern_timer::Timer,
    ) -> Self {
        let product = SmeltingSkill::products()[(product_index - 1) as usize].0;

        Self {
            product,
            player,
            facility_id,
            inventories,
            timer,
        }
    }

    pub fn can_perform(_player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
    }
}

impl<'a> CommandHandler<'a> for ActivateSmelteryCommand<'a> {
    fn timer(&self) -> Option<&extern_timer::Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (60 + self
            .player
            .get_attribute(Attribute::SkillTime("smelting".into()), 0)) as u32
    }

    fn create_activity(
        &self,
        timer: extern_timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = SmeltingActivity::new(
            self.product,
            self.expiration(),
            self.player.id,
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );

        let inventory = self
            .inventories
            .get(&self.player.id)
            .expect("unable to get inventory.");

        if SmeltingSkill::can_produce(self.product, inventory) {
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

pub struct SmeltingActivity {
    product: SmeltingSkill,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: extern_timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl SmeltingActivity {
    fn new(
        product: SmeltingSkill,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: extern_timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            product,
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

impl Activity for SmeltingActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Smelting
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn on_completion(
        &self,
        player_inventory_id: u64,
        _facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        let inventory = inventories
            .get_mut(&player_inventory_id)
            .expect("unable to find inventory");

        inventory.consume(
            ItemClass::Ore,
            format!("{} Ore", self.product.to_string()),
            4,
            items,
        );

        let wood_type = if inventory.count_of(ItemClass::Material, "Softwood Log") >= 1 {
            "Softwood Log"
        } else if inventory.count_of(ItemClass::Material, "Hardwood Log") >= 1 {
            "Hardwood Log"
        } else {
            panic!("wood not found");
        };

        inventory.consume(ItemClass::Material, wood_type, 1, items);

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Material,
                format!("{} Bar", self.product.to_string()),
            ),
        );

        if !SmeltingSkill::can_produce(self.product, inventory) {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }
        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
