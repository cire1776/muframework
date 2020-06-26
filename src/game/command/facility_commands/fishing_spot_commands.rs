use super::*;
use rand::Rng;
use FishType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FishType {
    Shrimp,
    Frog,
    Mackeral,
    Crab,
    Catfish,
    Salmon,
    Bass,
    Oyster,
    Flounder,
    Haddock,
    Swordfish,
    Eel,
    Sardines,
    SandbarShark,
    Pike,
    Lobster,
    Tuna,
    StripedMarlin,
    Herring,
    Trout,
    Snapper,
    RedTrout,
    RedHerring,
    Cod,
    Sturgeon,
    GiantCatfish,
    Grouper,
    BlackSeaBass,
}

impl FishType {
    pub fn from(value: u8) -> Option<FishType> {
        match value {
            1 => Some(Shrimp),
            2 => Some(Frog),
            3 => Some(Mackeral),
            4 => Some(Crab),
            5 => Some(Catfish),
            6 => Some(Salmon),
            7 => Some(Bass),
            8 => Some(Oyster),
            9 => Some(Flounder),
            10 => Some(Haddock),
            11 => Some(Swordfish),
            12 => Some(Eel),
            13 => Some(Sardines),
            14 => Some(SandbarShark),
            15 => Some(Pike),
            16 => Some(Lobster),
            17 => Some(Tuna),
            18 => Some(StripedMarlin),
            19 => Some(Herring),
            20 => Some(Trout),
            21 => Some(Snapper),
            22 => Some(RedTrout),
            23 => Some(RedHerring),
            24 => Some(Cod),
            25 => Some(Sturgeon),
            26 => Some(GiantCatfish),
            27 => Some(Grouper),
            28 => Some(BlackSeaBass),
            _ => None,
        }
    }
}

impl ToString for FishType {
    fn to_string(&self) -> String {
        match self {
            Shrimp => "Shrimp",
            Frog => "Frog",
            Mackeral => "Mackeral",
            Crab => "Crab",
            Catfish => "Catfish",
            Salmon => "Salmon",
            Bass => "Bass",
            Oyster => "Oyster",
            Flounder => "Flounder",
            Haddock => "Haddock",
            Swordfish => "Swordfish",
            Eel => "Eel",
            Sardines => "Sardine",
            SandbarShark => "Sandbar Shark",
            Pike => "Pike",
            Lobster => "Lobster",
            Tuna => "Tuna",
            StripedMarlin => "Striped Marlin",
            Herring => "Herring",
            Trout => "Trout",
            Snapper => "Snapper",
            RedTrout => "Red Trout",
            RedHerring => "Red Herring",
            Cod => "Cod",
            Sturgeon => "Sturgeon",
            GiantCatfish => "Giant Catfish",
            Grouper => "Grouper",
            BlackSeaBass => "Black Sea Bass",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FishingSpotProperties {
    facility: Facility,
}

impl<'a> FishingSpotProperties {
    pub fn new(facility: Facility) -> Self {
        Self { facility }
    }
    pub fn body_of_waster(&self) -> u64 {
        self.facility.get_property("body_of_waster") as u64
    }

    pub fn net_products(&self) -> (FishType, Option<FishType>) {
        let fish_type_1 = FishType::from(self.facility.get_property("net_1_product") as u8);
        let fish_type_2 = FishType::from(self.facility.get_property("net_2_product") as u8);
        (fish_type_1.expect("unable to find fish"), fish_type_2)
    }

    pub fn net_product_chance(&self) -> u8 {
        (100 - self.facility.get_property("net_2_chance")) as u8
    }

    pub fn net_timer(&self) -> u32 {
        self.facility.get_property("net_timer") as u32
    }

    pub fn rod_products(&self) -> (FishType, Option<FishType>) {
        let fish_type_1 = FishType::from(self.facility.get_property("rod_1_product") as u8);
        let fish_type_2 = FishType::from(self.facility.get_property("rod_2_product") as u8);
        (fish_type_1.expect("unable to find fish"), fish_type_2)
    }

    pub fn rod_product_chance(&self) -> u8 {
        (100 - self.facility.get_property("rod_2_chance")) as u8
    }

    pub fn rod_timer(&self) -> u32 {
        self.facility.get_property("rod_timer") as u32
    }

    pub fn trap_products(&self) -> (FishType, Option<FishType>) {
        let fish_type_1 = FishType::from(self.facility.get_property("trap_1_product") as u8);
        let fish_type_2 = FishType::from(self.facility.get_property("trap_2_product") as u8);
        (fish_type_1.expect("unable to find fish"), fish_type_2)
    }

    pub fn trap_product_chance(&self) -> u8 {
        (100 - self.facility.get_property("trap_2_chance")) as u8
    }

    pub fn trap_timer(&self) -> u32 {
        self.facility.get_property("trap_timer") as u32
    }

    pub fn trap_spawn(&self) -> u8 {
        self.facility.get_property("trap_spawn") as u8
    }

    pub fn trap_cooldown(&self) -> u32 {
        self.facility.get_property("trap_cooldown") as u32
    }

    pub fn fish_type(
        product_1: FishType,
        product_2: Option<FishType>,
        product_chance: u8,
    ) -> FishType {
        if product_2.is_none() {
            product_1
        } else {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0, 100) < product_chance {
                product_1
            } else {
                product_2.unwrap()
            }
        }
    }
}

pub struct ActivateNetFishingCommand<'a> {
    fishing_spot_properties: FishingSpotProperties,
    player: &'a mut Player,
    facility: &'a mut Facility,
}

impl<'a> ActivateNetFishingCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a mut FacilityList) -> Self {
        let facility = facilities
            .get_mut(facility_id)
            .expect("unable to find facility");
        Self {
            fishing_spot_properties: FishingSpotProperties::new(facility.clone()),
            player,
            facility,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":can_net_fish")
    }
}

impl<'a> CommandHandler<'a> for ActivateNetFishingCommand<'a> {
    fn expiration(&self) -> u32 {
        let base_time = self.fishing_spot_properties.net_timer();
        let modifier = self
            .player
            .get_attribute(Attribute::SkillTime("fishing".into()), 0);

        (base_time as i64 + modifier as i64) as u32
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let (product1, product2) = self.fishing_spot_properties.net_products();

        let activity = NetFishingActivity::new(
            product1,
            product2,
            self.fishing_spot_properties.net_product_chance(),
            self.expiration(),
            self.player.id,
            self.facility.id,
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

pub struct NetFishingActivity {
    product_1: FishType,
    product_2: Option<FishType>,
    product_chance: u8,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl<'a> NetFishingActivity {
    fn new(
        product_1: FishType,
        product_2: Option<FishType>,
        product_chance: u8,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            product_1,
            product_2,
            product_chance,
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

impl<'a> Activity for NetFishingActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::NetFishing
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
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        let fish_type =
            FishingSpotProperties::fish_type(self.product_1, self.product_2, self.product_chance);

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Ingredient,
                fish_type.to_string(),
            ),
        );

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}

pub struct ActivateFishingCommand<'a> {
    fishing_spot_properties: FishingSpotProperties,
    player: &'a mut Player,
    facility: &'a mut Facility,
}

impl<'a> ActivateFishingCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a mut FacilityList) -> Self {
        let facility = facilities
            .get_mut(facility_id)
            .expect("unable to find facility");
        Self {
            fishing_spot_properties: FishingSpotProperties::new(facility.clone()),
            player,
            facility,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":can_fish")
    }
}

impl<'a> CommandHandler<'a> for ActivateFishingCommand<'a> {
    fn expiration(&self) -> u32 {
        let base_time = self.fishing_spot_properties.rod_timer();
        let modifier = self
            .player
            .get_attribute(Attribute::SkillTime("fishing".into()), 0);

        (base_time as i64 + modifier as i64) as u32
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let (product1, product2) = self.fishing_spot_properties.rod_products();

        let activity = FishingActivity::new(
            product1,
            product2,
            self.fishing_spot_properties.rod_product_chance(),
            self.expiration(),
            self.player.id,
            self.facility.id,
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

pub struct FishingActivity {
    product_1: FishType,
    product_2: Option<FishType>,
    product_chance: u8,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl<'a> FishingActivity {
    fn new(
        product_1: FishType,
        product_2: Option<FishType>,
        product_chance: u8,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            product_1,
            product_2,
            product_chance,
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

impl<'a> Activity for FishingActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Fishing
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
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        let fish_type =
            FishingSpotProperties::fish_type(self.product_1, self.product_2, self.product_chance);

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Ingredient,
                fish_type.to_string(),
            ),
        );

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}

pub struct ActivatePlaceFishingTrapCommand<'a> {
    fishing_spot_properties: FishingSpotProperties,
    player: &'a mut Player,
    facility: &'a mut Facility,
}

impl<'a> ActivatePlaceFishingTrapCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a mut FacilityList) -> Self {
        let facility = facilities
            .get_mut(facility_id)
            .expect("unable to find facility");
        Self {
            fishing_spot_properties: FishingSpotProperties::new(facility.clone()),
            player,
            facility,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":can_place_fishing_trap")
    }
}

impl<'a> CommandHandler<'a> for ActivatePlaceFishingTrapCommand<'a> {
    fn expiration(&self) -> u32 {
        let base_time = self.fishing_spot_properties.trap_timer();
        let modifier = self
            .player
            .get_attribute(Attribute::SkillTime("fishing".into()), 0);

        (base_time as i64 + modifier as i64) as u32
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = PlacingFishingTrapActivity::new(
            self.fishing_spot_properties.clone(),
            self.expiration(),
            self.player.id,
            self.facility.id,
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

pub struct PlacingFishingTrapActivity {
    fishing_spot_properties: FishingSpotProperties,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl<'a> PlacingFishingTrapActivity {
    fn new(
        fishing_spot_properties: FishingSpotProperties,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            fishing_spot_properties,
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

impl<'a> Activity for PlacingFishingTrapActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::PlacingTrap
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn on_completion(
        &self,
        _player_inventory_id: u64,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        use std::time::{SystemTime, UNIX_EPOCH};

        let cooldown = self.fishing_spot_properties.trap_cooldown() as u128;
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            + (cooldown * 1000);

        facility.set_property("trap_expiration", expiration as i128);
        facility.set_property("is_in_use", 1);

        Command::send(
            Some(command_sender),
            Command::TransferEquipmentToInventory(MountingPoint::OnHand, facility.id),
        );

        Command::send(Some(command_sender), Command::ActivityAbort);

        RefreshInventoryFlag::DontRefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}

pub struct ActivateCollectFishingTrapCommand<'a> {
    fishing_spot_properties: FishingSpotProperties,
    player: &'a mut Player,
    facility: &'a mut Facility,
}

impl<'a> ActivateCollectFishingTrapCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a mut FacilityList) -> Self {
        let facility = facilities
            .get_mut(facility_id)
            .expect("unable to find facility");
        Self {
            fishing_spot_properties: FishingSpotProperties::new(facility.clone()),
            player,
            facility,
        }
    }

    pub fn can_perform(_player: &Player, facility: &Facility) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};

        let expiration = facility.get_property("trap_expiration");

        if expiration == 0 {
            return false;
        }

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i128
            - expiration
            >= 0
    }
}

impl<'a> CommandHandler<'a> for ActivateCollectFishingTrapCommand<'a> {
    fn expiration(&self) -> u32 {
        let base_time = self.fishing_spot_properties.trap_timer();
        let modifier = self
            .player
            .get_attribute(Attribute::SkillTime("fishing".into()), 0);

        (base_time as i64 + modifier as i64) as u32
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = CollectFishingTrapActivity::new(
            self.fishing_spot_properties.clone(),
            self.expiration(),
            self.player.id,
            self.facility.id,
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

pub struct CollectFishingTrapActivity {
    fishing_spot_properties: FishingSpotProperties,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl<'a> CollectFishingTrapActivity {
    fn new(
        fishing_spot_properties: FishingSpotProperties,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            fishing_spot_properties,
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

impl<'a> Activity for CollectFishingTrapActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::CollectingTrap
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
        facility: &mut Facility,
        _items: &mut ItemList,
        inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        Command::send(Some(command_sender), Command::ActivityAbort);

        facility.set_property("trap_expiration", 0);
        facility.set_property("is_in_use", 0);

        let mut rng = rand::thread_rng();
        let max_spawn = facility.get_property("trap_spawn");
        let spawns = rng.gen_range(1, max_spawn + 1);

        for _ in 0..spawns {
            let (product_1, product_2) = self.fishing_spot_properties.trap_products();
            let fish_type = FishingSpotProperties::fish_type(
                product_1,
                product_2,
                self.fishing_spot_properties.trap_product_chance(),
            );
            Command::send(
                Some(&command_sender),
                Command::SpawnItem(
                    player_inventory_id,
                    ItemClass::Ingredient,
                    fish_type.to_string(),
                ),
            );
        }

        let spot_inventory = inventories
            .get_mut(&facility.id)
            .expect("uable to get spot inventory");

        let item = spot_inventory.first().expect("unable to find trap.");

        Command::send(
            Some(command_sender),
            Command::TransferItem(item.id, facility.id, player_inventory_id),
        );

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
