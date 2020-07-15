use super::*;

pub struct ActivatePatchCommand<'a> {
    product: ProduceType,
    player: &'a mut Player,
    facility_id: u64,
    timer: &'a mut Timer,
}

impl<'a> ActivatePatchCommand<'a> {
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
            product: Self::determine_produce_type(facility),
            player,
            facility_id,
            timer,
        }
    }

    fn determine_produce_type(facility: &Facility) -> ProduceType {
        let product = facility.get_property("produce");
        ProduceType::from(product)
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && (player.is_endorsed_with(":can_pick"))
    }
}

impl<'a> CommandHandler<'a> for ActivatePatchCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        HarvestingSkill::expiration(self.product, self.player)
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = PatchActivity::new(
            self.product,
            self.expiration(),
            self.player.id,
            self.facility_id,
            Some(guard),
        );

        if HarvestingSkill::can_produce(self.product, self.player) {
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

pub struct PatchActivity {
    product: ProduceType,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    guard: Option<Guard>,
}

impl PatchActivity {
    fn new(
        product: ProduceType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            product,
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            guard,
        }
    }
}

impl Activity for PatchActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::PaneTitle::Harvesting
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
            // bye-bye
            Command::send(Some(command_sender.clone()), Command::ActivityAbort);
            Command::send(Some(command_sender), Command::DestroyFacility(facility.id));
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
