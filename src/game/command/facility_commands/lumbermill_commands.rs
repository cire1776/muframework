use super::*;
use LogType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LogType {
    Softwood,
    Hardwood,
}

impl ToString for LogType {
    fn to_string(&self) -> String {
        match self {
            LogType::Softwood => "Softwood".into(),
            LogType::Hardwood => "Hardwood".into(),
        }
    }
}

pub struct ActivateLumberMillCommand<'a> {
    log_type: LogType,
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateLumberMillCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            log_type: Self::determine_log_type(player),
            player,
            facility_id,
        }
    }

    fn determine_log_type(player: &Player) -> LogType {
        if player.is_endorsed_with(":wants_to_mill_hardwood") {
            Hardwood
        } else {
            Softwood
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
            && (player.is_endorsed_with(":wants_to_mill_softwood")
                || player.is_endorsed_with(":wants_to_mill_hardwood"))
    }
}

impl<'a> CommandHandler<'a> for ActivateLumberMillCommand<'a> {
    fn expiration(&self) -> u32 {
        match self.log_type {
            LogType::Softwood => 40,
            LogType::Hardwood => 60,
        }
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = LumbermillActivity::new(
            self.log_type,
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
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }
        activity
    }
}

pub struct LumbermillActivity {
    log_type: LogType,
    expiration: u32,
    player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl LumbermillActivity {
    fn new(
        log_type: LogType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            log_type,
            expiration,
            player_inventory_id,
            facility_id,
            _timer: timer,
            guard,
            update_sender,
            command_sender,
        }
    }
}

impl Activity for LumbermillActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(self.expiration * 1000, ui::pane::PaneTitle::Sawing),
        );
    }
    fn complete(
        &mut self,
        facilities: &mut FacilityList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
    ) {
        let facility = facilities
            .get_mut(self.facility_id)
            .expect("can't find facility");

        self.on_completion(
            self.player_inventory_id,
            facility,
            items,
            inventories,
            &self.update_sender,
            &self.command_sender,
        );
    }
    fn on_completion(
        &self,
        player_inventory_id: u64,
        _facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        use inflector::Inflector;

        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        let inventory = inventories
            .get_mut(&player_inventory_id)
            .expect("unable to find inventory");

        let wood = self.log_type.to_string().to_title_case();

        if !inventory.any_left_after_consuming(
            ItemClass::Material,
            format!("{} Log", wood),
            1,
            items,
        ) {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Material,
                format!("{} Plank", wood),
            ),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
