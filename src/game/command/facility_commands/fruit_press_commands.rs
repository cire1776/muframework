use super::*;

enum FruitPressMode {
    Pressing,
    Filling,
}
pub struct FruitPressInventoryFilter {}

impl FruitPressInventoryFilter {
    pub fn new() -> Self {
        Self {}
    }
}
impl game::inventory::InventoryFilter for FruitPressInventoryFilter {
    fn filter_type(&self, inventory: &Inventory, item: &Item) -> bool {
        let accepted_types = if inventory.is_empty() {
            vec![
                ItemType::new(ItemClass::Food, "Apple"),
                ItemType::new(ItemClass::Food, "Olive"),
            ]
        } else {
            vec![inventory.first().unwrap().item_type]
        };

        accepted_types.contains(&item.item_type)
    }

    fn filter_quantity(&self, inventory: &Inventory, __item: &Item) -> u8 {
        let existing_items = inventory.first();

        match existing_items {
            None => return 64,
            Some(existing_items) => 64 - existing_items.quantity,
        }
    }
}

pub struct OpenFruitPressCommand<'a> {
    player: &'a mut Player,
    external_inventory: &'a Inventory,
    __facility_id: u64,
    __item_types: &'a ItemTypeList,
    __facilities: &'a FacilityList,
}

impl<'a> OpenFruitPressCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        __facilities: &'a FacilityList,
        __item_types: &'a ItemTypeList,
        inventories: &'a mut InventoryList,
    ) -> Self {
        let external_inventory = inventories.get_mut(&facility_id).unwrap();
        external_inventory.prohibit_manual_extraction = true;

        external_inventory.set_item_filter(Some(Box::new(FruitPressInventoryFilter::new())));

        Self {
            player,
            external_inventory,
            __facility_id: facility_id,
            __item_types,
            __facilities,
        }
    }

    pub fn can_perform(_player: &Player, _facility: &Facility) -> bool {
        true
    }
}

impl<'a> CommandHandler<'a> for OpenFruitPressCommand<'a> {
    fn expiration(&self) -> u32 {
        0
    }
    fn perform_execute(&mut self) -> Option<Box<dyn Activity>> {
        self.player.external_inventory = Some(self.external_inventory.to_vec());
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ExternalInventoryOpened(
                self.external_inventory.to_vec(),
                self.external_inventory.id(),
            ),
        );

        activity
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FruitType {
    Apple = 1,
    Olive,
}

impl FruitType {
    pub fn from<N: TryInto<u8>>(index: N) -> FruitType {
        match index.try_into().ok().expect("must be convertible to u8") {
            1 => FruitType::Apple,
            2 => FruitType::Olive,
            _ => panic!("unknown fruit type"),
        }
    }

    pub fn from_string<S: ToString>(string: S) -> FruitType {
        match &string.to_string().to_lowercase()[..] {
            "apple" => FruitType::Apple,
            "olive" => FruitType::Olive,
            _ => panic!("unknown fruit type"),
        }
    }
    #[allow(unreachable_patterns)]
    pub fn to_index(&self) -> u8 {
        match self {
            FruitType::Apple => 1,
            FruitType::Olive => 2,
            _ => panic!("unknown fruit type"),
        }
    }
}

pub struct ActivateFruitPressCommand<'a> {
    mode: FruitPressMode,
    fruit_type: FruitType,
    player: &'a mut Player,
    facility_id: u64,
    facilities: &'a mut FacilityList,
    __items: &'a mut ItemList,
    inventory: &'a mut Inventory,
    timer: &'a mut Timer,
}

impl<'a> ActivateFruitPressCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        items: &'a mut ItemList,
        inventories: &'a mut InventoryList,
        timer: &'a mut Timer,
    ) -> Self {
        let inventory = inventories
            .get_mut(&facility_id)
            .expect("unable to find inventory");
        Self {
            player,
            fruit_type: FruitType::Apple,
            facility_id,
            facilities,
            __items: items,
            inventory,
            timer,
            mode: FruitPressMode::Filling,
        }
    }

    pub fn can_perform(_facility: &Facility, inventory: &Inventory) -> bool {
        inventory.first().is_some()
    }
}

impl<'a> CommandHandler<'a> for ActivateFruitPressCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (60 + self.player.get_attribute(Attribute::SkillTime(Cooking), 0)) as u32
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = FruitPressActivity::new(
            self.player.id,
            self.fruit_type,
            self.expiration(),
            self.facility_id,
            Some(guard),
        );
        Some(Box::new(activity))
    }

    fn prepare_to_execute(&mut self) {
        self.mode = FruitPressMode::Pressing;

        let facility = self
            .facilities
            .get_mut(self.facility_id)
            .expect("unable to get facility");

        let item = self.inventory.first().expect("unable to get inventory");

        let fruit_value = FruitType::from_string(&item.raw_description()).to_index();

        facility.set_property("item", fruit_value as i128);

        self.fruit_type = FruitType::from_string(item.raw_description());
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

pub struct FruitPressActivity {
    _player_inventory_id: u64,
    fruit_type: FruitType,
    expiration: u32,
    facility_id: u64,
    guard: Option<Guard>,
}

impl FruitPressActivity {
    pub fn new(
        player_inventory_id: u64,
        fruit_type: FruitType,
        expiration: u32,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            _player_inventory_id: player_inventory_id,
            fruit_type,
            expiration,
            facility_id,
            guard,
        }
    }
}

impl Activity for FruitPressActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Pressing
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
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
        if facility.increment_property("output") == 10 {
            Command::send(Some(command_sender.clone()), Command::ActivityAbort);
        }

        let inventory = inventories
            .get_mut(&facility.id)
            .expect("unable to find inventory");

        let fruit = match self.fruit_type {
            FruitType::Apple => "Apple",
            FruitType::Olive => "Olive",
        };

        let xp_gain = match self.fruit_type {
            FruitType::Apple => 5,
            FruitType::Olive => 8,
        };

        player.increment_xp(Cooking, xp_gain, rng, Some(&update_sender));

        if !inventory.any_left_after_consuming(ItemClass::Food, fruit, 1, items) {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::DontRefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

pub struct ActivateFruitPressFillCommand<'a> {
    player: &'a mut Player,
    fruit_type: FruitType,
    facility_id: u64,
    facilities: &'a mut FacilityList,
    __items: &'a mut ItemList,
    __inventory: &'a mut Inventory,
    mode: FruitPressMode,
    timer: &'a mut Timer,
}

impl<'a> ActivateFruitPressFillCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        __items: &'a mut ItemList,
        inventories: &'a mut InventoryList,
        timer: &'a mut Timer,
    ) -> Self {
        let inventory = inventories
            .get_mut(&facility_id)
            .expect("unable to find inventory");
        Self {
            player,
            fruit_type: FruitType::Apple,
            facility_id,
            facilities,
            __items,
            __inventory: inventory,
            mode: FruitPressMode::Filling,
            timer,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        facility.get_property("output") > 0 && player.is_endorsed_with(":can_fill")
    }
}

impl<'a> CommandHandler<'a> for ActivateFruitPressFillCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        (30 + self.player.get_attribute(Attribute::SkillTime(Cooking), 0)) as u32
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let activity = FruitPressFillActivity::new(
            self.fruit_type,
            self.expiration(),
            self.player.id,
            self.facility_id,
            Some(guard),
        );
        Some(Box::new(activity))
    }

    fn prepare_to_execute(&mut self) {
        self.mode = FruitPressMode::Filling;

        let facility = self
            .facilities
            .get_mut(self.facility_id)
            .expect("unable to find facility");

        let index = facility.get_property("item");
        self.fruit_type = FruitType::from(index);
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

pub struct FruitPressFillActivity {
    fruit_type: FruitType,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    guard: Option<Guard>,
}

impl FruitPressFillActivity {
    pub fn new(
        fruit_type: FruitType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        guard: Option<Guard>,
    ) -> Self {
        Self {
            fruit_type,
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            guard,
        }
    }
}

impl Activity for FruitPressFillActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Filling
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn on_completion(
        &self,
        player: &mut Player,
        facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _game_data: &mut GameData,
        _rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        let product = match self.fruit_type {
            FruitType::Apple => "Apple Juice",
            FruitType::Olive => "Olive Oil",
        }
        .to_string();

        Command::send(
            Some(command_sender.clone()),
            Command::SpawnItem(1, ItemClass::Food, product),
        );

        let inventory = inventories.get_mut(&player.inventory_id()).unwrap();
        if !inventory.any_left_after_consuming(ItemClass::Material, "Glass Bottle", 1, items) {
            Command::send(Some(command_sender.clone()), Command::ActivityAbort);
        }

        if facility.decrement_property("output") == 0 {
            Command::send(Some(command_sender), Command::ActivityAbort);
        }

        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}
