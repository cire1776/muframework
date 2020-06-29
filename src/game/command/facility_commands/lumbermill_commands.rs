use super::*;
use LogType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LogType {
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
    timer: &'a mut Timer,
}

impl<'a> ActivateLumberMillCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, timer: &'a mut Timer) -> Self {
        Self {
            log_type: Self::determine_log_type(player),
            player,
            facility_id,
            timer,
        }
    }

    fn determine_log_type(player: &Player) -> LogType {
        let wood_type = player
            .get_endorsement_component(":wants_to_mill")
            .expect("couldnt find wood type.");

        match &wood_type[..] {
            "hardwood" => Hardwood,
            "softwood" => Softwood,
            _ => panic!("unknown wood type: {}", wood_type),
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":wants_to_mill")
    }
}

impl<'a> CommandHandler<'a> for ActivateLumberMillCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (match self.log_type {
            LogType::Softwood => 40,
            LogType::Hardwood => 60,
        } + self
            .player
            .get_attribute(Attribute::SkillTime("construction".into()), 0)) as u32
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = LumbermillActivity::new(
            self.log_type,
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

pub struct LumbermillActivity {
    log_type: LogType,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    guard: Option<Guard>,
}

impl LumbermillActivity {
    fn new(
        log_type: LogType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            log_type,
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            guard,
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
        player: &mut Player,
        facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let level = player.get_attribute(Attribute::SkillLevel("construction".into()), 0) as u8;

        {
            let inventory = inventories
                .get_mut(&player.inventory_id())
                .expect("unable to find inventory");

            if !ConstructionSkill::can_produce(self.log_type, level, inventory) {
                Command::send(Some(command_sender.clone()), Command::ActivityAbort);
            }
        }

        {
            ConstructionSkill::consume_from_inventory_for(
                self.log_type,
                player,
                inventories,
                items,
            );
        }
        let (class, description) =
            ConstructionSkill::produce_results_for(self.log_type, player, rng);

        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(player.inventory_id(), class, description),
        );

        let chance_of_breakage = facility.get_property("chance_of_breakage");
        if rng.fails(0, chance_of_breakage, "lumbermill_breaks") {
            Command::send(
                Some(command_sender.clone()),
                Command::DestroyFacility(facility.id),
            );
            Command::send(Some(command_sender), Command::ActivityAbort);
            return RefreshInventoryFlag::RefreshInventory;
        }

        {
            let inventory = inventories
                .get_mut(&player.inventory_id())
                .expect("unable to find inventory");

            let level = player.get_attribute(Attribute::SkillLevel("construction".into()), 0) as u8;
            if !ConstructionSkill::can_produce(self.log_type, level, inventory) {
                Command::send(Some(command_sender), Command::ActivityAbort);
            }
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
