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
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        self.player.external_inventory = Some(self.external_inventory.to_vec());
        None
    }

    fn announce(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ExternalInventoryOpened(
                self.external_inventory.to_vec(),
                self.external_inventory.id(),
            ),
        );
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
    player: &'a mut Player,
    fruit_type: FruitType,
    facility_id: u64,
    facilities: &'a mut FacilityList,
    __items: &'a mut ItemList,
    inventory: &'a mut Inventory,
    mode: FruitPressMode,
}

impl<'a> ActivateFruitPressCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        items: &'a mut ItemList,
        inventories: &'a mut InventoryList,
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
            mode: FruitPressMode::Filling,
        }
    }

    pub fn can_perform(_facility: &Facility, inventory: &Inventory) -> bool {
        inventory.first().is_some()
    }
}

impl<'a> CommandHandler<'a> for ActivateFruitPressCommand<'a> {
    fn expiration(&self) -> u32 {
        60
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = FruitPressActivity::new(
            self.player.id,
            self.fruit_type,
            self.expiration(),
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );
        Some(Box::new(activity))
    }

    fn set_activity(&mut self, activity: Option<Box<dyn Activity>>) {
        self.player.activity = activity;
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

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

pub struct FruitPressActivity {
    player_inventory_id: u64,
    fruit_type: FruitType,
    expiration: u32,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl FruitPressActivity {
    pub fn new(
        player_inventory_id: u64,
        fruit_type: FruitType,
        expiration: u32,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            player_inventory_id,
            fruit_type,
            expiration,
            facility_id,
            _timer: timer,
            guard,
            update_sender,
            command_sender,
        }
    }
}

impl Activity for FruitPressActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(self.expiration * 1000, ui::pane::PaneTitle::Pressing),
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
        _player_inventory_id: u64,
        facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        let count = facility.get_property("output");
        facility.set_property("output", count + 1);
        if count + 1 == 10 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        let inventory = inventories
            .get_mut(&facility.id)
            .expect("unable to find inventory");

        let fruit = match self.fruit_type {
            FruitType::Apple => "Apple",
            FruitType::Olive => "Olive",
        };

        if !inventory.any_left_after_consuming(ItemClass::Food, fruit, 1, items) {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
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
}

impl<'a> ActivateFruitPressFillCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a mut FacilityList,
        __items: &'a mut ItemList,
        inventories: &'a mut InventoryList,
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
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        facility.get_property("output") > 0 && player.is_endorsed_with(":can_fill")
    }
}

impl<'a> CommandHandler<'a> for ActivateFruitPressFillCommand<'a> {
    fn expiration(&self) -> u32 {
        30
    }

    fn set_activity(&mut self, activity: Option<Box<dyn Activity>>) {
        self.player.activity = activity
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = FruitPressFillActivity::new(
            self.fruit_type,
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

    fn prepare_to_execute(&mut self) {
        self.mode = FruitPressMode::Filling;

        let facility = self
            .facilities
            .get_mut(self.facility_id)
            .expect("unable to find facility");

        let index = facility.get_property("item");
        self.fruit_type = FruitType::from(index);
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

pub struct FruitPressFillActivity {
    fruit_type: FruitType,
    expiration: u32,
    player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl FruitPressFillActivity {
    pub fn new(
        fruit_type: FruitType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            fruit_type,
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

impl Activity for FruitPressFillActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(self.expiration * 1000, ui::pane::PaneTitle::Filling),
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
        _player_inventory_id: u64,
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        let product = match self.fruit_type {
            FruitType::Apple => "Apple Juice",
            FruitType::Olive => "Olive Oil",
        }
        .to_string();

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(1, ItemClass::Food, product),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        let count = facility.get_property("output");
        facility.set_property("output", count - 1);
        if count - 1 == 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}
