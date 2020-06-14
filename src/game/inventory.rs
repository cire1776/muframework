use super::*;
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

#[derive(Debug, Clone)]
pub struct Inventory {
    id: u64,
    pub items: HashMap<u64, Item>,
}

impl Inventory {
    /// returns an inventory with the given id.  Does not store it in any
    ///    inventory list.
    pub fn new(id: u64) -> Inventory {
        Inventory {
            id,
            items: HashMap::new(),
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
    /// let item = Item::new(208, ItemClass::Potion, "A green, odorous potion",1);
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
    /// let mut items = ItemList::new();
    /// let item = Item::new(601, ItemClass::Headwear,"baseball cap",1);
    /// subject.accept(&item, &mut items);
    /// assert_eq!(subject.count(), 1);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    // fn can_hold(&self, item: &Item) -> bool {
    //     if !self.is_full() {
    //         return true;
    //     }

    //     if !item.is_stackable() {
    //         self.any_open_stacks_for(item)
    //         return false;
    //     }
    //     println!(
    //         "Your inventory is full.  Cannot pick up {}.",
    //         item.description()
    //     );
    // }

    /// adds an item to the inventory and sets the item to stored in the item list.
    /// # Arguments:
    /// * item - the item to be added
    /// * items - the master item list
    /// ```
    /// # use muframework::game::inventory::*;
    /// # use muframework::game::items::*;
    /// let mut subject = Inventory::new(1);
    /// let mut items = ItemList::new();
    /// let item = Item::new(1146, ItemClass::Shoes, "Prada red papal shoes",1);
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
        self.force_accept(item);
    }

    pub fn accept_stack(&mut self, item: &mut Item, items: &mut ItemList) {
        use std::cmp::min;

        // add test for non-stackable items
        if item.is_stackable() {
            for (_i, current_item) in self.items.iter_mut() {
                if !item.is_same_type_as(current_item) {
                    continue;
                }
                let limit = ItemClass::stack_limits(current_item.class);
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
    /// let mut items = ItemList::new();
    /// let item = Item::new(1146, ItemClass::Shoes, "Prada red papal shoes",1);
    /// items.store(&item,subject.id());
    ///
    /// subject.accept_by_id(item.id, &mut items);
    ///
    /// assert_eq!(subject.count(),1);
    /// ```
    /// ``` should_panic
    /// # use muframework::game::*;
    /// let mut subject = Inventory::new(1);
    /// let mut items = ItemList::new();
    ///
    /// subject.accept_by_id(404, &mut items);
    /// ```
    pub fn accept_by_id(&mut self, item_id: u64, items: &mut ItemList) {
        let item = items.get_as_item(item_id).expect("Item not found.");
        self.accept(&item, items);
    }

    /// is public for testing purposes only.
    pub fn force_accept(&mut self, item: &Item) {
        self.items.insert(item.id, item.clone());
    }

    /// creates an item and places it in the inventory generating
    ///   an appropriate id.
    pub fn spawn_item<S: ToString>(
        &mut self,
        class: ItemClass,
        description: S,
        items: &mut ItemList,
    ) {
        let mut item = Item::spawn(class, description);
        self.accept_stack(&mut item, items);
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

    // simply lets go of an item without placing it in a bundle in effect disposing of the item if it
    //   has not been transferred already.
    pub fn release_item(&mut self, item_id: &u64) {
        self.items.remove(&item_id);
    }

    pub fn holds(&mut self, item_id: u64) -> bool {
        self.items.contains_key(&item_id)
    }

    pub fn pick_up_item(&mut self, item_id: u64, items: &mut ItemList) {
        let possible_item = items.get(item_id);

        match possible_item {
            None => panic!("expected item_id to exist"),
            Some(ItemState::Stored(_item, _inventory_id)) => panic!("expected dropped item"),
            Some(ItemState::Bundle(item, _x, _y)) => self.accept(&item, items),
            Some(ItemState::Equipped(_item, _inventory_id)) => {
                panic!("expected a non-equipped item")
            }
        }
    }

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

    #[test]
    fn to_iter() {
        let subject = Inventory::new(1);
        assert_eq!(subject.into_iter().len(), 0);
    }

    #[test]
    fn index() {
        let mut subject = Inventory::new(1);
        let item = Item::new(1, ItemClass::Gloves, "some item", 1);
        let mut items = ItemList::new();
        subject.accept(&item, &mut items);

        assert_eq!(subject[1], item);
    }

    #[test]
    #[should_panic(expected = "index not found")]
    fn index_should_panic_when_index_not_found() {
        let mut subject = Inventory::new(1);
        let item = Item::new(1, ItemClass::Gloves, "some item", 1);
        let mut items = ItemList::new();
        subject.accept(&item, &mut items);

        &subject[1777];
    }

    #[test]
    #[should_panic(expected = "index not found")]
    fn remove_item_removes_the_item() {
        let mut subject = Inventory::new(1);
        let item = Item::new(1777, ItemClass::Gloves, "some item", 1);
        let mut items = ItemList::new();
        subject.accept(&item, &mut items);

        subject.release_item(&1777);

        &subject[1777];
    }
}

#[cfg(test)]
mod inventory_spawn_item {
    use super::*;
    use ItemClass::*;

    #[test]
    fn it_stacks_correctly() {
        let mut items = ItemList::new();
        let mut subject = Inventory::new(1);
        let item = Item::spawn(Potion, "Mtn Dew");

        items.store(&item, subject.id());
        subject.accept(&item, &mut items);

        subject.spawn_item(Potion, "Mtn Dew", &mut items);

        assert_eq!(subject[item.id].quantity, 2);
    }
}

#[cfg(test)]
mod accept_stack {
    use super::*;
    use ItemClass::*;

    #[test]
    fn if_item_is_non_stackable_it_adds_it_to_inventory_as_new_stack() {
        let mut items = ItemList::new();
        let mut subject = Inventory::new(1);

        let mut item = Item::spawn(Headwear, "Mtn Dew Cap");

        subject.accept_stack(&mut item, &mut items);

        assert_eq!(item.quantity, 1);
        assert!(subject.holds(item.id));
    }

    #[test]
    fn adds_1_to_the_stack_when_given_an_item_of_1() {
        let mut items = ItemList::new();
        let mut subject = Inventory::new(1);
        let item = Item::spawn(Potion, "Mtn Dew");
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
        let mut items = ItemList::new();
        let mut subject = Inventory::new(1);
        let item1 = Item::spawn(Potion, "Mtn Dew");
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
        let mut items = ItemList::new();
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
        let mut items = ItemList::new();
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
        let mut items = ItemList::new();
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
        let mut items = ItemList::new();
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

// TODO: test spawn_stack to ensure it enforces stacking limits
