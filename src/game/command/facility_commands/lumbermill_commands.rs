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
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
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
            _player_inventory_id: player_inventory_id,
            facility_id,
            _timer: timer,
            guard,
            _update_sender: update_sender,
            _command_sender: command_sender,
        }
    }
}

impl Activity for LumbermillActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Sawing
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
        use inflector::Inflector;

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

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
