use super::*;

pub struct PickupCommand<'a> {
    item_id: u64,
    inventory: &'a mut Inventory,
    items: &'a mut ItemList,
}

impl<'a> PickupCommand<'a> {
    pub fn new(
        item_id: u64,
        inventory: &'a mut Inventory,
        items: &'a mut ItemList,
    ) -> PickupCommand<'a> {
        PickupCommand {
            item_id,
            inventory,
            items,
        }
    }
}

impl<'a> CommandHandler<'a> for PickupCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        self.inventory.pick_up_item(self.item_id, &mut self.items);
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(Some(update_tx), GameUpdate::ItemRemoved(self.item_id));
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::InventoryUpdated(self.inventory.to_vec()),
        );
        activity
    }
}

pub struct DropCommand<'a> {
    item: &'a Item,
    x: i32,
    y: i32,
    inventory: &'a mut Inventory,
    items: &'a mut ItemList,
}

impl<'a> DropCommand<'a> {
    pub fn new(
        item: &'a Item,
        x: i32,
        y: i32,
        inventory: &'a mut Inventory,
        items: &'a mut ItemList,
    ) -> DropCommand<'a> {
        DropCommand {
            item,
            x,
            y,
            inventory,
            items,
        }
    }
}

impl<'a> CommandHandler<'a> for DropCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        self.inventory
            .release_item_at(self.x, self.y, &self.item, &mut self.items);
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ItemAdded {
                id: self.item.id,
                x: self.x,
                y: self.y,
                description: self.item.description(),
                class: self.item.class(),
            },
        );
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::InventoryUpdated(self.inventory.to_vec()),
        );
        activity
    }
}

pub struct EquipCommand<'a> {
    item: &'a Item,
    player: &'a mut Player,
    item_class_specifiers: &'a ItemClassSpecifierList,
    inventory: &'a mut Inventory,
    items: &'a mut ItemList,
}

impl<'a> EquipCommand<'a> {
    pub fn new(
        item: &'a Item,
        player: &'a mut Player,
        item_class_specifiers: &'a ItemClassSpecifierList,
        inventory: &'a mut Inventory,
        items: &'a mut ItemList,
    ) -> EquipCommand<'a> {
        EquipCommand {
            item,
            player,
            item_class_specifiers,
            inventory,
            items,
        }
    }
}

impl<'a> CommandHandler<'a> for EquipCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        {
            let player_mounting_points = &mut self.player.mounting_points;

            player_mounting_points.mount(
                self.item,
                self.item_class_specifiers,
                self.inventory,
                &mut self.items,
            );
        }
        let player_mounting_points = self.player.mounting_points.clone();

        self.player.clear_endorsements();
        player_mounting_points.endorse(self.player, &self.items);
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        let equipment_list: Vec<Item> = (&self.player.mounting_points).to_vec_of_items(&self.items);

        GameUpdate::send(
            Some(update_tx),
            GameUpdate::EquipmentUpdated(equipment_list),
        );
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::InventoryUpdated(self.inventory.to_vec()),
        );
        activity
    }
}

pub struct UnequipCommand<'a> {
    item_id: u64,
    inventory: &'a mut Inventory,
    player: &'a mut Player,
    items: &'a mut ItemList,
}

impl<'a> UnequipCommand<'a> {
    pub fn new(
        item_id: u64,
        inventory: &'a mut Inventory,
        player: &'a mut Player,
        items: &'a mut ItemList,
    ) -> UnequipCommand<'a> {
        UnequipCommand {
            item_id,
            inventory,
            player,
            items,
        }
    }
}

impl<'a> CommandHandler<'a> for UnequipCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        self.player
            .mounting_points
            .unmount_item_by_id(self.item_id, self.inventory, self.items);

        self.player.clear_endorsements();
        self.player
            .mounting_points
            .clone()
            .endorse(&mut self.player, self.items);
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        let equipment_list: Vec<Item> = self.player.mounting_points.to_vec_of_items(&self.items);

        GameUpdate::send(
            Some(update_tx),
            GameUpdate::EquipmentUpdated(equipment_list),
        );
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::InventoryUpdated(self.inventory.to_vec()),
        );
        activity
    }
}

fn transfer_an_item<'a>(
    item: &'a Item,
    source_id: u64,
    destination_id: u64,
    inventories: &'a mut InventoryList,
    items: &'a mut ItemList,
) -> bool {
    {
        let src_inventory = inventories.get(&source_id).unwrap();
        if src_inventory.prohibit_manual_extraction {
            return false;
        }
    }

    {
        let dest_inventory = &mut (inventories.get_mut(&destination_id).unwrap());
        if dest_inventory.is_full() || dest_inventory.quantity_permitted(&item) == 0 {
            return false;
        }
        dest_inventory.accept_stack_unmut(&item, items);
    }

    {
        let src_inventory = &mut (inventories.get_mut(&source_id).unwrap());
        src_inventory.release_item(&item.id);
    }
    true
}

fn announce_transfer(
    inventories: &InventoryList,
    source_id: u64,
    destination_id: u64,
    update_tx: &GameUpdateSender,
) {
    let dest_inventory = inventories.get(&destination_id).unwrap();
    let src_inventory = inventories.get(&source_id).unwrap();

    if destination_id == 1 {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::InventoryUpdated(dest_inventory.to_vec()),
        );
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ExternalInventoryUpdated(src_inventory.to_vec()),
        );
    } else {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ExternalInventoryUpdated(dest_inventory.to_vec()),
        );
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::InventoryUpdated(src_inventory.to_vec()),
        );
    }
}

pub struct TransferItemCommand<'a> {
    item: &'a Item,
    source_id: u64,
    destination_id: u64,
    inventories: &'a mut InventoryList,
    items: &'a mut ItemList,
}

impl<'a> TransferItemCommand<'a> {
    pub fn new(
        item: &'a Item,
        source_id: u64,
        destination_id: u64,
        inventories: &'a mut InventoryList,
        items: &'a mut ItemList,
    ) -> Self {
        Self {
            item,
            source_id,
            destination_id,
            inventories,
            items,
        }
    }
}

impl<'a> CommandHandler<'a> for TransferItemCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        transfer_an_item(
            self.item,
            self.source_id,
            self.destination_id,
            self.inventories,
            self.items,
        );
        None
    }
    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        announce_transfer(
            self.inventories,
            self.source_id,
            self.destination_id,
            update_tx,
        );
        activity
    }
}

pub struct TransferAllCommand<'a> {
    source_id: u64,
    destination_id: u64,
    inventories: &'a mut InventoryList,
    items: &'a mut ItemList,
}

impl<'a> TransferAllCommand<'a> {
    pub fn new(
        source_id: u64,
        destination_id: u64,
        inventories: &'a mut InventoryList,
        items: &'a mut ItemList,
    ) -> Self {
        Self {
            source_id,
            destination_id,
            inventories,
            items,
        }
    }
}

impl<'a> CommandHandler<'a> for TransferAllCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        let src_inventory = self.inventories.get_mut(&self.source_id).unwrap();

        for (_id, item) in &src_inventory.items.clone() {
            if !transfer_an_item(
                &item,
                self.source_id,
                self.destination_id,
                &mut self.inventories,
                &mut self.items,
            ) {
                break;
            };
        }
        None
    }
    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        announce_transfer(
            self.inventories,
            self.source_id,
            self.destination_id,
            update_tx,
        );
        activity
    }
}

#[cfg(test)]
mod pickup_command {
    use super::*;

    #[test]
    fn execute_sends_two_game_update_messages() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        items.add_new_item_to_bundle_at(10, 15, 1776, ItemClass::Potion, "A Red Bubbling Potion");

        let mut command = PickupCommand::new(1776, &mut inventory, &mut items);
        let (sender, receiver) = std::sync::mpsc::channel();

        command.execute(Some(&sender), None);

        let mut response = receiver.recv().unwrap();
        if let ItemRemoved(item_id) = response {
            assert_eq! {1776, item_id}
        } else {
            panic!(format!("response not appropriate: {:?}", response));
        }
        response = receiver.recv().unwrap();
        if let InventoryUpdated(items) = response {
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].id, 1776);
        } else {
            panic!("response not appropriate");
        }
    }

    #[test]
    fn execute_adds_item_to_inventory() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        items.add_new_item_to_bundle_at(10, 15, 1776, ItemClass::Potion, "A Red Bubbling Potion");

        let mut command = PickupCommand::new(1776, &mut inventory, &mut items);

        command.execute(None, None);

        assert_eq!(inventory.count(), 1);
    }

    #[test]
    fn execute_adds_item_to_item_list() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        items.add_new_item_to_bundle_at(10, 15, 1776, ItemClass::Potion, "A Red Bubbling Potion");

        let mut command = PickupCommand::new(1776, &mut inventory, &mut items);

        command.execute(None, None);

        println!("{:?}", items);

        assert_eq!(items.count(), 1);
        assert_eq!(
            items.get(1776),
            Some(ItemState::Stored(
                Item {
                    id: 1776,
                    quantity: 1,
                    item_type: ItemType::new(ItemClass::Potion, "A Red Bubbling Potion"),
                },
                1
            ))
        );
    }
}

#[cfg(test)]
mod drop_command {
    use super::*;

    #[test]
    fn execute_sends_two_messages() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        let mut item = Item::new(
            1776,
            ItemType::new(ItemClass::Potion, "A Red Bubbling Potion"),
            1,
        );
        inventory.accept_stack(&mut item, &mut items);

        let mut command = DropCommand::new(&item, 10, 15, &mut inventory, &mut items);

        let (sender, receiver) = std::sync::mpsc::channel();

        command.execute(Some(&sender), None);

        let mut response = receiver.recv().unwrap();
        if let ItemAdded {
            id: item_id,
            x,
            y,
            description,
            class,
        } = response
        {
            assert_eq! {1776, item_id}
            assert_eq!(x, 10);
            assert_eq!(y, 15);
            assert_eq!(description, item.description());
            assert_eq!(class, item.class());
        } else {
            panic!(format!("response not appropriate: {:?}", response));
        }
        response = receiver.recv().unwrap();
        if let InventoryUpdated(items) = response {
            assert_eq!(items.len(), 0);
        } else {
            panic!("response not appropriate");
        }
    }

    #[test]
    fn execute_adds_a_bundle_at_location() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        let mut item = Item::new(
            1776,
            ItemType::new(ItemClass::Potion, "A Red Bubbling Potion"),
            1,
        );
        inventory.accept_stack(&mut item, &mut items);

        let mut command = DropCommand::new(&item, 10, 15, &mut inventory, &mut items);

        command.execute(None, None);

        let item_state = items.get(item.id);
        if let Some(ItemState::Bundle(new_item, x, y)) = item_state {
            assert_eq!(new_item, item);
            assert_eq!(x, 10);
            assert_eq!(y, 15);
        }
    }
    #[test]
    fn execute_adds_removes_item_from_the_inventory() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        let mut item = Item::new(
            1776,
            ItemType::new(ItemClass::Potion, "A Red Bubbling Potion"),
            1,
        );
        inventory.accept_stack(&mut item, &mut items);

        let mut command = DropCommand::new(&item, 10, 15, &mut inventory, &mut items);

        command.execute(None, None);

        assert_eq!(inventory.count(), 0);
    }
}

#[cfg(test)]
mod support {
    use super::*;

    #[allow(dead_code)]
    pub fn test_item(
        description: &str,
        class: ItemClass,
        id: u64,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) -> Item {
        let mut item = Item {
            id,
            quantity: 1,
            item_type: ItemType::new(class, description),
        };
        items[id] = ItemState::Stored(item.clone(), inventory.id());
        inventory.accept_stack(&mut item, items);

        item
    }

    pub fn test_item_from_type(
        item_type: &ItemType,
        id: u64,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) -> Item {
        let mut item = Item {
            id,
            quantity: 1,
            item_type: item_type.clone(),
        };
        items[id] = ItemState::Stored(item.clone(), inventory.id());
        inventory.accept_stack(&mut item, items);

        item
    }
}

#[cfg(test)]
mod unequip_command {
    use super::*;
    use support::*;

    #[test]
    fn execute_unmounts_item_that_occupies_single_mounting_point() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        let item_class_specifiers = ItemClassSpecifier::initialize();

        let item = test_item(
            "a hat",
            ItemClass::Headwear,
            159,
            &mut inventory,
            &mut items,
        );

        let mut player = Player::new();

        let item_mounting_points = vec![&MountingPoint::Head];

        player
            .mounting_points
            .mount(&item, &item_class_specifiers, &mut inventory, &mut items);

        let mut subject = UnequipCommand::new(item.id, &mut inventory, &mut player, &mut items);

        subject.execute(None, None);

        assert_eq!(inventory.count(), 1);
        assert_eq!(items.count(), 1);
        assert_eq!(
            items.get(item.id),
            Some(ItemState::Stored(item, inventory.id()))
        );
        assert!(player.mounting_points.is_empty(&item_mounting_points[0]))
    }

    #[test]
    fn announce_sends_two_messages() {
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);

        let item = test_item(
            "a fuzzy hat",
            ItemClass::Headwear,
            422,
            &mut inventory,
            &mut items,
        );

        let mut player = Player::new();

        let mut subject = UnequipCommand::new(item.id, &mut inventory, &mut player, &mut items);

        let (sender, receiver) = std::sync::mpsc::channel();
        subject.execute(Some(&sender), None);

        match receiver.try_recv() {
            Ok(GameUpdate::EquipmentUpdated(equipment_list)) => assert_eq!(equipment_list, vec![]),
            _ => panic!("Unexpected response"),
        }

        match receiver.try_recv() {
            Ok(GameUpdate::InventoryUpdated(inventory_list)) => {
                assert_eq!(inventory_list, vec![item])
            }
            _ => panic!("Unexpected response"),
        }
    }

    #[test]
    fn old_endorsements_are_removed() {
        let mut inventory = Inventory::new(1);
        let mut items = &mut ItemList::new(None);
        let item_class_specifiers = ItemClassSpecifier::initialize();

        let mut item_type = ItemType::new(ItemClass::Tool, "Reed Basket");
        item_type.add_endorsement(":picks_nose");

        let item = test_item_from_type(&item_type, 159, &mut inventory, &mut items);

        let mut player = Player::new();

        player
            .mounting_points
            .mount(&item, &item_class_specifiers, &mut inventory, items);

        let mounting_points = player.mounting_points.clone();
        mounting_points.endorse(&mut player, items);

        let mut subject = UnequipCommand::new(item.id, &mut inventory, &mut player, &mut items);
        subject.execute(None, None);

        assert!(!player.is_endorsed_with(":picks_nose"));
    }
}
