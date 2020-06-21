use super::*;
use std::fmt;
use std::ops::Index;

#[derive(Debug)]
pub struct AliasList {
    aliases: HashMap<String, u64>,
}

impl AliasList {
    /// returns a new, almost empty AliasList.
    ///   Does include the "player" alias as id 1.
    pub fn new(player_inventory_id: u64) -> Self {
        let mut result = Self {
            aliases: HashMap::new(),
        };
        result.insert("player", player_inventory_id);
        result
    }

    /// returns the length of the AliasList
    /// use muframework::game::*;
    /// let subject = AliasList::new();
    /// assert_eq!(subject.len(), 1);
    pub fn len(&self) -> usize {
        self.aliases.len()
    }

    /// inserts the id as alias
    /// # Examples:
    /// ```
    /// # use muframework::game::inventory::*;
    /// let mut subject = AliasList::new(1);
    /// subject.insert("fakealias", 337);
    /// assert_eq!(subject.get(&"fakealias").unwrap(), &337);
    /// ```
    pub fn insert<S: ToString>(&mut self, alias: S, id: u64) {
        self.aliases.insert(alias.to_string(), id);
    }

    pub fn insert_if_necessary<S: ToString>(&mut self, alias: Option<S>, id: u64) {
        if let Some(alias) = alias {
            self.insert(alias, id);
        }
    }

    /// returns the id for the given alias
    pub fn get<S: ToString>(&self, alias: &S) -> Option<&u64> {
        self.aliases.get(&alias.to_string())
    }
}

pub type InventoryList = HashMap<u64, Inventory>;

pub trait InventoryFilter {
    fn filter(&self, inventory: &Inventory, item: &Item) -> u8 {
        if !self.filter_type(&inventory, &item) {
            return 0;
        }
        self.filter_quantity(&inventory, &item)
    }
    fn filter_type(&self, _inventory: &Inventory, _item: &Item) -> bool {
        true
    }
    fn filter_quantity(&self, _inventory: &Inventory, _item: &Item) -> u8 {
        64
    }
}

pub struct Inventory {
    id: u64,
    permitted_types: Vec<ItemType>,
    item_filter: Option<Box<dyn InventoryFilter>>,
    pub items: HashMap<u64, Item>,
    pub prohibit_manual_extraction: bool,
}

impl Clone for Inventory {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            permitted_types: self.permitted_types.clone(),
            item_filter: None,
            items: self.items.clone(),
            prohibit_manual_extraction: false,
        }
    }
}

impl fmt::Debug for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("id", &self.id)
            .field("permitted_types", &self.permitted_types)
            .field("items", &self.items)
            .field(
                "prohibit_manual_extraction",
                &self.prohibit_manual_extraction,
            )
            .finish()
    }
}

impl Inventory {
    /// returns an inventory with the given id.  Does not store it in any
    ///    inventory list.
    pub fn new(id: u64) -> Inventory {
        Inventory {
            id,
            permitted_types: vec![],
            item_filter: None,
            items: HashMap::new(),
            prohibit_manual_extraction: false,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    /// returns a reference to a new inventory with the given id that has been stored in
    ///   the given inventory list.
    /// # Examples:
    /// ```
    /// use muframework::game::inventory::*;
    /// let mut inventories = InventoryList::new();
    /// let subject = Inventory::new_into_inventory_list(222, &mut inventories);
    /// assert_eq!(subject.id(), 222);
    /// assert_eq!(subject as *const Inventory, inventories.get(&222).unwrap());
    /// ```
    pub fn new_into_inventory_list<'a>(
        id: u64,
        inventories: &'a mut InventoryList,
    ) -> &'a mut Inventory {
        let result = Inventory::new(id);
        inventories.insert(result.id, result);
        inventories.get_mut(&id).unwrap()
    }

    /// returns true if the inventory is full, otherwise false.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.items.len() > 26
    }

    /// returns true if the inventory is empty.  false otherwise.
    /// # Examples:
    /// ```
    /// # use muframework::game::inventory::*;
    /// let subject = Inventory::new(1);
    /// assert!(subject.is_empty());
    /// ```
    /// ```
    /// # use muframework::game::inventory::*;
    /// # use muframework::game::items::*;
    /// let mut subject = Inventory::new(1);
    /// let item = Item::new(208, ItemType::new(ItemClass::Potion, "A green, odorous potion"),1);
    /// subject.force_accept(&item);
    /// assert!(!subject.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// returns the number of items in the inventory
    ///
    /// # Examples:
    /// ```
    /// # use muframework::game::inventory::Inventory;
    /// let inventory = Inventory::new(1);
    /// assert_eq!(inventory.count(), 0);
    /// ```
    /// ```
    /// # use muframework::game::inventory::*;
    /// # use muframework::game::items::*;
    /// let mut subject = Inventory::new(1);
    /// let mut items = ItemList::new(None);
    /// let item = Item::new(601, ItemType::new(ItemClass::Headwear,"baseball cap"),1);
    /// subject.accept(&item, &mut items);
    /// assert_eq!(subject.count(), 1);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn set_item_filter(&mut self, filter: Option<Box<dyn game::inventory::InventoryFilter>>) {
        let result = if filter.is_some() {
            Some(filter.unwrap())
        } else {
            None
        };

        self.item_filter = result;
    }

    pub fn permitted_type(&mut self, item_type: &ItemType) {
        self.permitted_types.push(item_type.clone());
    }

    pub fn quantity_permitted(&self, item: &Item) -> u8 {
        if self.permitted_types.is_empty() && self.item_filter.is_none() {
            return 64;
        }

        let mut quantity = 64;

        if let Some(item_filter) = &self.item_filter {
            quantity = item_filter.filter(self, item)
        };

        if self.permitted_types.is_empty() {
            quantity
        } else {
            if self.permitted_types.contains(&item.item_type) {
                0
            } else {
                quantity
            }
        }
    }

    /// adds an item to the inventory and sets the item to stored in the item list.
    /// # Arguments:
    /// * item - the item to be added
    /// * items - the master item list
    /// ```
    /// # use muframework::game::inventory::*;
    /// # use muframework::game::items::*;
    /// let mut subject = Inventory::new(1);
    /// let mut items = ItemList::new(None);
    /// let item = Item::new(1146, ItemType::new(ItemClass::Shoes, "Prada red papal shoes"),1);
    ///
    /// subject.accept(& item, &mut items);
    ///
    /// assert_eq!(subject.count(),1);
    /// ```
    #[inline]
    /// public for testing purposes.
    /// use accept_stack instead.
    pub fn accept(&mut self, item: &Item, items: &mut ItemList) {
        if self.is_full() {
            println!(
                "Your inventory is full.  Cannot pick up {}.",
                item.description()
            );
            return;
        }

        items.store(item, self.id);
        self.accept_permissible(item, items);
    }

    /// adds a stack of items to the inventory.  This should be the primary
    /// means of adding items to the inventory, because it handles stacks.
    /// # Arguments:
    /// * item - the item to be added
    /// * items - the master item list
    pub fn accept_stack(&mut self, item: &mut Item, items: &mut ItemList) {
        use std::cmp::min;

        // add test for non-stackable items
        if item.is_stackable() {
            for (_i, current_item) in self.items.iter_mut() {
                if !item.is_same_type_as(current_item) {
                    continue;
                }
                let limit = ItemClass::stack_limits(current_item.class());
                let quantity_to_move = min(limit - current_item.quantity, item.quantity);
                item.quantity -= quantity_to_move;
                current_item.quantity += quantity_to_move;
                items.update_item(item);
                items.update_item(current_item);
            }
        }

        if item.quantity > 0 {
            self.accept(&item, items);
        }
    }

    pub fn accept_stack_unmut(&mut self, item: &Item, items: &mut ItemList) {
        let mut item = item.clone();
        self.accept_stack(&mut item, items)
    }

    /// adds an item to the inventory from its id and sets the item to stored in the item list.
    /// # Arguments:
    /// * item_if - the id of the item to be added
    /// * items - the master item list
    /// ```
    /// # use muframework::game::inventory::*;
    /// # use muframework::game::items::*;
    /// let mut subject = Inventory::new(1);
    /// let mut items = ItemList::new(None);
    /// let item = Item::new(1146, ItemType::new(ItemClass::Shoes, "Prada red papal shoes"),1);
    /// items.store(&item,subject.id());
    ///
    /// subject.accept_by_id(item.id, &mut items);
    ///
    /// assert_eq!(subject.count(),1);
    /// ```
    /// ``` should_panic
    /// # use muframework::game::*;
    /// let mut subject = Inventory::new(1);
    /// let mut items = ItemList::new(None);
    ///
    /// subject.accept_by_id(404, &mut items);
    /// ```
    pub fn accept_by_id(&mut self, item_id: u64, items: &mut ItemList) {
        let item = items.get_as_item(item_id).expect("Item not found.");
        self.accept_stack_unmut(&item, items);
    }

    fn accept_permissible(&mut self, item: &Item, items: &mut ItemList) {
        let mut item = &mut item.clone();

        let quantity = std::cmp::max(self.quantity_permitted(item), item.quantity);

        if quantity < item.quantity {
            let new_item_state = items.get(item.id);

            let mut new_item = item.clone();
            new_item.quantity = item.quantity - quantity;

            match new_item_state {
                None => panic!("item not found"),
                Some(ItemState::Equipped(_, _)) => panic!("can't accept from equipped item"),
                Some(ItemState::Bundle(_, _, _)) => panic!("Can't accept from a bundle"),
                Some(ItemState::Stored(_, inventory_id)) => items.store(&new_item, inventory_id),
            }

            item.quantity = quantity;
        }

        self.force_accept(&item);
    }

    /// is public for testing purposes only.
    pub fn force_accept(&mut self, item: &Item) {
        self.items.insert(item.id, item.clone());
    }

    /// creates an item and places it in the inventory generating
    ///   an appropriate id.
    pub fn spawn_item<S: ToString, N: TryInto<u8>>(
        &mut self,
        class: ItemClass,
        description: S,
        quantity: N,
        items: &mut ItemList,
    ) {
        let quantity = quantity.try_into().ok().expect("must be convertible to u8");
        let mut item = Item::spawn_with_type(class, description, quantity, items);
        self.accept_stack(&mut item, items);
    }

    pub fn spawn_stack<S: ToString, N: TryInto<u8>>(
        &mut self,
        class: ItemClass,
        description: S,
        quantity: N,
        items: &mut ItemList,
    ) -> Item {
        let quantity = quantity.try_into().ok().expect("must be convertible to u8");

        let mut item = Item::spawn_with_type(class, description.to_string(), quantity, items);

        items.store(&item, self.id());
        self.force_accept(&mut item);

        item
    }

    pub fn spawn_by_type<S: ToString, N: TryInto<u8>>(
        &mut self,
        item_type_name: S,
        quantity: N,
        item_types: &ItemTypeList,
        items: &mut ItemList,
    ) -> Item {
        let mut quantity = quantity.try_into().ok().expect("must be convertible to u8");
        let item_type_name = item_type_name.to_string();

        let class = item_types[&item_type_name].class;
        let limit = ItemClass::stack_limits(class);
        if quantity > limit {
            self.spawn_by_type(item_type_name.clone(), quantity - limit, item_types, items);
            quantity = limit;
        }

        let mut item = Item::spawn_from_type(item_type_name, quantity, item_types);
        self.accept_stack(&mut item, items);
        item
    }

    pub fn update_item(&mut self, item_id: u64, new_quantity: u8, items: &mut ItemList) {
        let item = self
            .items
            .get_mut(&item_id)
            .expect("unable to find item in inventory");
        item.quantity = new_quantity;
        items.update_item(item)
    }

    /// release item and bundle it at x,y
    ///
    /// # Arguments:
    /// * x, y - the destination
    /// * item - the item to be released
    /// * items - the master item list
    pub fn release_item_at(&mut self, x: i32, y: i32, item: &Item, items: &mut ItemList) {
        let item_id = item.id;
        items.add_item_to_bundle_at(x, y, item);
        self.release_item(&item_id);
    }

    /// simply lets go of an item without placing it in a bundle in effect disposing of the item if it
    ///   has not been transferred already.
    pub fn release_item(&mut self, item_id: &u64) {
        self.items.remove(&item_id);
    }

    pub fn consume<S: ToString, N: TryInto<u8>>(
        &mut self,
        class: ItemClass,
        description: S,
        quantity: N,
        items: &mut ItemList,
    ) {
        let mut quantity = quantity.try_into().ok().expect("must be convertible to u8");

        if quantity > 1 {
            todo!("Not yet implemented for more than 1");
        }

        let target_type = items.item_types.find(class, description).clone();

        for (_, v) in self.items.iter_mut() {
            if quantity <= 0 {
                break;
            }

            if v.item_type == target_type {
                let delta = std::cmp::min(v.quantity, quantity);
                quantity -= delta;
                v.quantity -= delta;

                if v.quantity > 0 {
                    items.update_item(v);
                } else {
                    items.remove(v);
                }
            }
        }
    }

    pub fn any_left_after_consuming<S: ToString, N: TryInto<u8>>(
        &mut self,
        class: ItemClass,
        description: S,
        quantity: N,
        items: &mut ItemList,
    ) -> bool {
        let quantity = quantity
            .try_into()
            .ok()
            .expect("must be convertible to u8.");

        let target = items.item_types.find(class, description).clone();

        self.consume(target.class, &target.description, quantity, items);

        self.items.iter().any(|(_, i)| i.item_type == target)
    }

    pub fn split_stack<N: TryInto<u8>>(
        &mut self,
        quantity_to_transfer: N,
        item_id: u64,
        items: &mut ItemList,
    ) -> Result<Item, &'static str> {
        let quantity_to_transfer = quantity_to_transfer
            .try_into()
            .ok()
            .expect("Must be convertible to u8");

        let item = self[item_id].clone();

        if item.quantity == quantity_to_transfer {
            return Ok(item);
        }

        if quantity_to_transfer > item.quantity - 1 {
            return Err("quantity to transfer must be less than the item's quantity");
        }

        let new_item = self.spawn_stack(
            item.class(),
            item.raw_description(),
            quantity_to_transfer,
            items,
        );

        self.update_item(item.id, item.quantity - quantity_to_transfer, items);

        Ok(new_item)
    }
    /// returns true if inventory holds an item_id.
    pub fn holds(&mut self, item_id: u64) -> bool {
        self.items.contains_key(&item_id)
    }

    /// picks up an item from a bundle and puts it into the inventory.
    pub fn pick_up_item(&mut self, item_id: u64, items: &mut ItemList) {
        let possible_item = items.get(item_id);

        match possible_item {
            None => panic!("expected item_id to exist"),
            Some(ItemState::Stored(_item, _inventory_id)) => panic!("expected dropped item"),
            Some(ItemState::Bundle(mut item, _x, _y)) => self.accept_stack(&mut item, items),
            Some(ItemState::Equipped(_item, _inventory_id)) => {
                panic!("expected a non-equipped item")
            }
        }
    }

    pub fn first(&self) -> Option<Item> {
        let items: Vec<Item> = self.items.values().cloned().collect();
        items.get(0).map(|i| i.clone())
    }

    /// returns an vector of items.
    pub fn to_vec(&self) -> Vec<Item> {
        self.items.values().cloned().collect()
    }
}

impl IntoIterator for Inventory {
    type Item = self::Item;
    type IntoIter = std::vec::IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        let result: Vec<Item> = self.items.iter().map(|i| i.1.to_owned()).collect();
        result.into_iter()
    }
}

impl Index<u64> for Inventory {
    type Output = Item;
    fn index(&self, index: u64) -> &Self::Output {
        self.items.get(&index).expect("index not found")
    }
}

#[cfg(test)]
mod test_inventory {
    use super::*;
    #[allow(unused_imports)]
    use crate::test_support::*;

    #[test]
    fn to_iter() {
        let subject = Inventory::new(1);
        assert_eq!(subject.into_iter().len(), 0);
    }

    #[test]
    fn index() {
        let mut subject = Inventory::new(1);
        let item = Item::new(1, ItemType::new(ItemClass::Gloves, "some item"), 1);
        let mut items = ItemList::new(None);
        subject.accept(&item, &mut items);

        assert_eq!(subject[1], item);
    }

    #[test]
    #[should_panic(expected = "index not found")]
    fn index_should_panic_when_index_not_found() {
        let mut subject = Inventory::new(1);
        let item = Item::new(1, ItemType::new(ItemClass::Gloves, "some item"), 1);
        let mut items = ItemList::new(None);
        subject.accept(&item, &mut items);

        &subject[1777];
    }

    #[test]
    #[should_panic(expected = "index not found")]
    fn remove_item_removes_the_item() {
        let mut subject = Inventory::new(1);
        let item = Item::new(1777, ItemType::new(ItemClass::Gloves, "some item"), 1);
        let mut items = ItemList::new(None);
        subject.accept(&item, &mut items);

        subject.release_item(&1777);

        &subject[1777];
    }
}

#[cfg(test)]
mod inventory_spawn_item {
    use super::*;
    use crate::test_support::*;
    use ItemClass::*;

    #[test]
    fn it_stacks_correctly() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item = spawn_item_into_inventory(Potion, "Mtn Dew", 1, &mut subject, &mut items);

        subject.spawn_item(Potion, "Mtn Dew", 1, &mut items);

        assert_eq!(subject[item.id].quantity, 2);
    }
}

#[cfg(test)]
mod accept_stack {
    use super::*;
    use crate::test_support::*;
    use ItemClass::*;

    #[test]
    fn if_item_is_non_stackable_it_adds_it_to_inventory_as_new_stack() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);

        let mut item = test_item(Headwear, "Mtn Dew Cap", 1);

        subject.accept_stack(&mut item, &mut items);

        let mut new_item = test_item(Headwear, "Mtn Dew Cap", 1);

        subject.accept_stack(&mut new_item, &mut items);

        assert_eq!(item.quantity, 1);
        assert!(subject.holds(item.id));
        assert_eq!(new_item.quantity, 1);
        assert!(subject.holds(new_item.id));
    }

    #[test]
    fn adds_1_to_the_stack_when_given_an_item_of_1() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item = test_item(Potion, "Mtn Dew", 1);
        let item_id = item.id;

        let mut new_item = Item::spawn_stack(Potion, "Mtn Dew", 7);

        items.store(&item, subject.id());
        items.store(&new_item, subject.id());

        subject.accept(&item, &mut items);

        subject.accept_stack(&mut new_item, &mut items);

        assert_eq!(items.get_as_item(item_id).unwrap().quantity, 8);
        assert_eq!(new_item.quantity, 0);
    }

    #[test]
    fn only_first_in_multiple_stacks_changed_if_it_can_fit() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item1 = test_item(Potion, "Mtn Dew", 1);
        let item2 = Item::spawn_stack(Potion, "Mtn Dew", 2);

        let mut new_item = Item::spawn_stack(Potion, "Mtn Dew", 7);

        subject.accept(&item1, &mut items);
        subject.accept(&item2, &mut items);

        items.store(&new_item, subject.id());

        subject.accept_stack(&mut new_item, &mut items);

        // use iter to determine order in which items were found because hashmap iterator is non-deterministic.
        let mut iter = subject.items.iter();
        let (changed_id, changed_item) = iter.next().unwrap();
        let (_, unchanged_item) = iter.next().unwrap();

        let exp_quantity1: u8;
        let exp_quantity2: u8;

        if *changed_id == item1.id {
            exp_quantity1 = 8;
            exp_quantity2 = 2
        } else {
            exp_quantity1 = 9;
            exp_quantity2 = 1
        };

        // relative to inventory
        assert_eq!(changed_item.quantity, exp_quantity1);
        assert_eq!(unchanged_item.quantity, exp_quantity2);
        assert_eq!(new_item.quantity, 0);

        // relative to ItemList
        assert_eq!(
            items.get_as_item(changed_item.id).unwrap().quantity,
            exp_quantity1
        );
        assert_eq!(
            items.get_as_item(unchanged_item.id).unwrap().quantity,
            exp_quantity2
        );
    }

    #[test]
    fn restricts_stacking_to_same_type_of_item() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item1 = Item::spawn_stack(Potion, "Mtn Dew", 2);
        subject.accept(&item1, &mut items);

        let mut new_item = Item::spawn_stack(Food, "Apple", 7);
        subject.accept_stack(&mut new_item, &mut items);

        assert_eq!(item1.quantity, 2);
        assert!(subject.holds(new_item.id));
        assert_eq!(new_item.quantity, 7);
    }

    #[test]
    fn adds_new_stack_if_existing_stack_is_completely_full() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item1 = Item::spawn_stack(Potion, "Mtn Dew", 18);
        subject.accept(&item1, &mut items);

        let mut new_item = Item::spawn_stack(Food, "Apple", 7);
        subject.accept_stack(&mut new_item, &mut items);

        assert_eq!(item1.quantity, 18);
        assert!(subject.holds(new_item.id));
        assert_eq!(new_item.quantity, 7);
    }

    #[test]
    fn adds_new_stack_with_remainder_if_existing_stack_is_almost_full() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item1 = Item::spawn_stack(Potion, "Mtn Dew", 15);
        subject.accept(&item1, &mut items);

        let mut new_item = Item::spawn_stack(Potion, "Mtn Dew", 7);

        subject.accept_stack(&mut new_item, &mut items);

        assert_eq!(subject[item1.id].quantity, 16);
        assert!(subject.holds(new_item.id));
        assert_eq!(new_item.quantity, 6);
    }

    #[test]
    fn if_finishes_with_zero_stack_is_removed_from_inventory_and_items() {
        let mut items = ItemList::new(None);
        let mut subject = Inventory::new(1);
        let item1 = Item::spawn_stack(Potion, "Mtn Dew", 9);
        subject.accept(&item1, &mut items);

        let mut new_item = Item::spawn_stack(Potion, "Mtn Dew", 7);

        subject.accept_stack(&mut new_item, &mut items);

        assert_eq!(subject[item1.id].quantity, 16);
        assert!(!subject.holds(new_item.id));
        assert_eq!(new_item.quantity, 0);
        assert!(!items.holds(new_item.id));
    }
}
