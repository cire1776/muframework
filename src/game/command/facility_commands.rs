extern crate chrono;
extern crate timer;

pub use super::*;
pub use std::sync::mpsc::channel;

#[allow(dead_code)]
// TODO: change facility to reflect open chest status.
pub struct OpenChestCommand<'a> {
    x: i32,
    y: i32,
    player: &'a mut Player,
    external_inventory: &'a Inventory,
    facility_id: u64,
    facilities: &'a FacilityList,
}

impl<'a> OpenChestCommand<'a> {
    pub fn new(
        x: i32,
        y: i32,
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a FacilityList,
        inventories: &'a InventoryList,
    ) -> Self {
        let external_inventory = inventories.get(&facility_id).unwrap();
        Self {
            x,
            y,
            player,
            external_inventory,
            facility_id,
            facilities,
        }
    }
}

impl<'a> CommandHandler for OpenChestCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        self.player.external_inventory = Some(self.external_inventory.to_vec());
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
pub enum TreeUse {
    Picking,
    Logging,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TreeType {
    Apple,
    Oak,
    Olive,
    Pine,
}

impl TreeType {
    #[inline]
    pub fn from_facility_class(class: FacilityClass) -> TreeType {
        match class {
            FacilityClass::AppleTree => TreeType::Apple,
            FacilityClass::OliveTree => TreeType::Olive,
            _ => panic!("{:?} is not a recognized tree", class),
        }
    }
}

pub struct ActivateTreePickingCommand<'a> {
    tree_type: TreeType,
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateTreePickingCommand<'a> {
    pub fn new(tree_type: TreeType, player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            tree_type,
            player,
            facility_id,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
            && player.is_endorsed_with(":can_pick")
            && match facility.class {
                FacilityClass::AppleTree | FacilityClass::OliveTree => true,
                _ => false,
            }
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> Box<dyn Activity> {
        let command_sender = command_sender.clone();
        let update_sender = update_sender.clone();

        let activity = TreePickingActivity::new(
            self.tree_type,
            self.expiration(),
            self.player.inventory_id(),
            self.facility_id,
            timer,
            guard,
            update_sender,
            command_sender,
        );
        Box::new(activity)
    }
}

impl<'a> CommandHandler for ActivateTreePickingCommand<'a> {
    fn expiration(&self) -> u32 {
        match self.tree_type {
            TreeType::Apple => 60,
            TreeType::Olive => 90,
            _ => panic!("Non-fruit tree supplied"),
        }
    }

    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        // currently base timer is the same for all fruit trees
        let base_time = self.expiration();

        let guard =
            timer.schedule_repeating(chrono::Duration::seconds(base_time as i64), move || {
                Command::send(Some(&command_sender), Command::ActivityComplete);
            });

        let activity = self.create_activity(timer, guard, &update_sender, command_tx.unwrap());
        self.player.activity = Some(activity);
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

#[allow(dead_code)]
pub struct TreePickingActivity {
    tree_type: TreeType,
    expiration: u32,
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> TreePickingActivity {
    pub fn new(
        tree_type: TreeType,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            tree_type,
            expiration,
            player_inventory_id,
            facility_id,
            timer,
            guard: Some(guard),
            update_sender,
            command_sender,
        }
    }
}

impl<'a> Activity for TreePickingActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        let title = match self.tree_type {
            TreeType::Apple => ui::pane::PaneTitle::PickingApples,
            TreeType::Olive => ui::pane::PaneTitle::PickingOlives,
            _ => panic!("Non-fruit tree specified"),
        };

        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(self.expiration * 1000, title),
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
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        let item_class: ItemClass;
        let item_description: &str;

        match self.tree_type {
            TreeType::Apple => {
                item_class = ItemClass::Food;
                item_description = "Apple"
            }
            TreeType::Olive => {
                item_class = ItemClass::Food;
                item_description = "Olive"
            }
            _ => panic!("{:?} is not a fruit tree.", self.tree_type),
        }

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, item_class, item_description.into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        let count = facility.get_property("fruit");
        facility.set_property("fruit", count - 1);
        if facility.get_property("fruit") <= 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

pub struct ActivateTreeLoggingCommand<'a> {
    tree_type: TreeType,
    player: &'a mut Player,
    facility_id: u64,
}

impl<'a> ActivateTreeLoggingCommand<'a> {
    pub fn new(tree_type: TreeType, player: &'a mut Player, facility_id: u64) -> Self {
        Self {
            tree_type,
            player,
            facility_id,
        }
    }

    pub fn can_perform(player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use() && player.is_endorsed_with(":can_chop")
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> Box<dyn Activity> {
        let command_sender = command_sender.clone();
        let update_sender = update_sender.clone();

        let activity = TreeLoggingActivity::new(
            self.tree_type,
            self.player.inventory_id(),
            self.facility_id,
            timer,
            guard,
            update_sender,
            command_sender,
        );
        Box::new(activity)
    }
}

impl<'a> CommandHandler for ActivateTreeLoggingCommand<'a> {
    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        // currently base timer is the same for all logging
        let base_time = 60;

        let guard = timer.schedule_repeating(chrono::Duration::seconds(base_time), move || {
            Command::send(Some(&command_sender), Command::ActivityComplete);
        });

        let activity = self.create_activity(timer, guard, &update_sender, command_tx.unwrap());
        self.player.activity = Some(activity);
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

#[allow(dead_code)]
pub struct TreeLoggingActivity {
    tree_type: TreeType,
    player_inventory_id: u64,
    facility_id: u64,
    timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl<'a> TreeLoggingActivity {
    pub fn new(
        tree_type: TreeType,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            tree_type,
            player_inventory_id,
            facility_id,
            timer,
            guard: Some(guard),
            update_sender,
            command_sender,
        }
    }
}

impl<'a> Activity for TreeLoggingActivity {
    fn start(&self, update_tx: &GameUpdateSender) {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ActivityStarted(60000, ui::pane::PaneTitle::Logging),
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
        facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) {
        GameUpdate::send(Some(&update_sender), GameUpdate::ActivityExpired());

        #[allow(unreachable_patterns)]
        let wood_type = match self.tree_type {
            TreeType::Apple | TreeType::Olive | TreeType::Oak => "Hardwood Log",
            TreeType::Pine => "Softwood Log",
            _ => panic!("unknown tree type"),
        };

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(player_inventory_id, ItemClass::Material, wood_type.into()),
        );

        Command::send(Some(&command_sender), Command::RefreshInventory);

        let count = facility.get_property("logs");
        facility.set_property("logs", count - 1);
        if facility.get_property("logs") <= 0 {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

pub struct OpenFruitPressCommand<'a> {
    player: &'a mut Player,
    external_inventory: &'a Inventory,
    __facility_id: u64,
    __item_types: &'a ItemTypeList,
    __facilities: &'a FacilityList,
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

    pub fn can_perform(_player: &Player, facility: &Facility) -> bool {
        facility.get_property("output") == 0
    }
}

impl<'a> CommandHandler for OpenFruitPressCommand<'a> {
    fn expiration(&self) -> u32 {
        0
    }
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        self.player.external_inventory = Some(self.external_inventory.to_vec());
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
}

pub struct ActivateFruitPressCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
    facilities: &'a mut FacilityList,
    items: &'a mut ItemList,
    inventory: &'a mut Inventory,
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
            facility_id,
            facilities,
            items,
            inventory,
        }
    }

    fn is_ready_to_press(_facility: &Facility, inventory: &Inventory) -> bool {
        inventory.first().is_some()
    }

    fn is_able_to_fill(player: &Player, facility: &Facility, items: &ItemList) -> bool {
        let possible_item = player.mounting_points.at(&MountingPoint::AtReady);
        if let Some(equipped_item_id) = possible_item {
            if items.does_item_match_description(equipped_item_id, "Glass Bottle") {
                facility.get_property("output") > 0
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    pub fn can_perform(
        player: &Player,
        facility: &Facility,
        items: &ItemList,
        inventory: &Inventory,
    ) -> bool {
        if Self::is_able_to_fill(player, facility, items) {
            return true;
        }
        Self::is_ready_to_press(facility, inventory)
    }

    fn create_press_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> Box<dyn Activity> {
        let command_sender = command_sender.clone();
        let update_sender = update_sender.clone();

        let activity = FruitPressActivity::new(
            self.player.id,
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );
        Box::new(activity)
    }

    fn create_fill_activity(
        &self,
        fruit_type: FruitType,
        timer: timer::Timer,
        guard: Guard,
        update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> Box<dyn Activity> {
        let command_sender = command_sender.clone();
        let update_sender = update_sender.clone();

        let activity = FruitPressFillActivity::new(
            fruit_type,
            self.player.id,
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );
        Box::new(activity)
    }

    pub fn activate_press(
        &mut self,
        // facility: &Facility,
        // items: &ItemList,
        // inventory: &Inventory,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        println!("press activted");
        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        let facility = self
            .facilities
            .get_mut(self.facility_id)
            .expect("unable to get facility");

        let item = self.inventory.first().expect("unable to get inventory");
        let fruit_value: u8 = match &item.raw_description()[..] {
            "Apple" => 1,
            "Olive" => 2,
            _ => panic!("unknown fruit type: {}", item.raw_description()),
        };

        facility.set_property("item", fruit_value);

        // currently the base time is the same for all presses
        let base_time = 60;

        let guard = timer.schedule_repeating(chrono::Duration::seconds(base_time), move || {
            Command::send(Some(&command_sender), Command::ActivityComplete);
        });

        let activity =
            self.create_press_activity(timer, guard, &update_sender, command_tx.unwrap());
        self.player.activity = Some(activity);
    }

    pub fn activate_fill(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        println!("fill activted");

        let timer = timer::Timer::new();

        // unwrap senders to avoid thread sending problems
        let command_sender = command_tx.unwrap().clone();
        let update_sender = update_tx.unwrap().clone();

        let facility = self
            .facilities
            .get_mut(self.facility_id)
            .expect("unable to find facility");

        let index = facility.get_property("item");
        let fruit_type = FruitType::from(index);

        // currently the base time is the same for all fills
        let base_time = 30;

        let guard = timer.schedule_repeating(chrono::Duration::seconds(base_time), move || {
            Command::send(Some(&command_sender), Command::ActivityComplete);
        });

        let activity = self.create_fill_activity(
            fruit_type,
            timer,
            guard,
            &update_sender,
            command_tx.unwrap(),
        );
        self.player.activity = Some(activity);
    }
}

impl<'a> CommandHandler for ActivateFruitPressCommand<'a> {
    fn perform_execute(
        &mut self,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) {
        let facility = self
            .facilities
            .get(self.facility_id)
            .expect("unable to get facility");

        if Self::is_able_to_fill(self.player, facility, self.items) {
            self.activate_fill(update_tx, command_tx);
            return;
        }

        if Self::is_ready_to_press(facility, self.inventory) {
            self.activate_press(update_tx, command_tx);
        }
    }

    fn announce(&self, update_tx: &std::sync::mpsc::Sender<GameUpdate>) {
        if let Some(activity) = &self.player.activity {
            activity.start(update_tx);
        }
    }
}

pub struct FruitPressActivity {
    player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    update_sender: GameUpdateSender,
    command_sender: CommandSender,
}

impl FruitPressActivity {
    pub fn new(
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            player_inventory_id,
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
            GameUpdate::ActivityStarted(60000, ui::pane::PaneTitle::Pressing),
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

        let inventory = inventories
            .get_mut(&facility.id)
            .expect("unable to find inventory");

        if !inventory.any_left_after_consuming(ItemClass::Food, "Apple", 1, items) {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }

        self.start(&update_sender);
    }

    fn clear_guard(&mut self) {
        self.guard = None;
    }
}

pub struct FruitPressFillActivity {
    fruit_type: FruitType,
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
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            fruit_type,
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
            GameUpdate::ActivityStarted(30000, ui::pane::PaneTitle::Filling),
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
