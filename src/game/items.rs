pub use super::*;
use regex::Regex;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum ItemClass {
    BladeWeapon,
    Dagger,
    Shield,
    SoftArmor,
    Pants,
    Gloves,
    Shoes,
    Headwear,
    Tool,
    Potion,
    Food,
}

impl ItemClass {
    pub fn from_symbol<S: ToString>(symbol: S) -> ItemClass {
        match &symbol.to_string()[..] {
            "↓" => ItemClass::Dagger,
            "^" => ItemClass::Headwear,
            "!" => ItemClass::BladeWeapon,
            "¡" => ItemClass::Potion,
            "♠" => ItemClass::Tool,
            _ => panic!("unknown item class"),
        }
    }

    pub fn stack_limits(class: ItemClass) -> u8 {
        use ItemClass::*;

        match class {
            Food => 64,
            Potion => 16,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ItemState {
    Equipped(Item, u64),    // (item, inventory_id)
    Stored(Item, u64),      // (item, inventory_id)
    Bundle(Item, i32, i32), // (item, x, y)
}

impl ItemState {
    pub fn bundle(item: Item, x: i32, y: i32) -> ItemState {
        ItemState::Bundle(item, x, y)
    }
    pub fn extract_item(item_state: &ItemState) -> Item {
        match item_state {
            ItemState::Equipped(item, _) => item,
            ItemState::Stored(item, _) => item,
            ItemState::Bundle(item, _, _) => item,
        }
        .to_owned()
    }
    pub fn is_bundled_at(&self, x: i32, y: i32) -> bool {
        if let ItemState::Bundle(_item, item_x, item_y) = self {
            *item_x == x && *item_y == y
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Item {
    pub id: u64,
    pub description: String,
    pub class: ItemClass,
    pub quantity: u8,
}

impl Item {
    pub fn new<S: ToString, N: TryInto<u8>>(
        id: u64,
        class: ItemClass,
        description: S,
        quantity: N,
    ) -> Item {
        Item {
            id,
            class,
            description: description.to_string(),
            quantity: quantity.try_into().ok().expect("must be convertible to u8"),
        }
    }

    pub fn description(&self) -> String {
        use inflector::string::pluralize::to_plural;
        use inflector::string::singularize::to_singular;

        let vowels = ["A", "E", "I", "O", "U"];

        let prefix = if self.quantity == 1 {
            if vowels.contains(&&self.description.to_uppercase()[0..1]) {
                "An".to_string()
            } else {
                "A".to_string()
            }
        } else {
            format!("{}", self.quantity).to_string()
        };

        let inflected_description = if self.quantity == 1 {
            to_singular(&self.description[..])
        } else {
            to_plural(&self.description[..])
        };

        format!("{} {}", prefix, inflected_description).clone()
    }

    /// creates a new item assigning it its Id as appropriate.
    pub fn spawn<S: ToString>(class: ItemClass, description: S) -> Item {
        Self::spawn_stack(class, description, 1)
    }

    pub fn spawn_stack<S: ToString, N: TryInto<u8>>(
        class: ItemClass,
        description: S,
        quantity: N,
    ) -> Item {
        Self::new(
            NEXT_ITEM_ID(),
            class,
            description,
            quantity.try_into().ok().expect("must be convertible to u8"),
        )
    }
    pub fn read_in_items(items: &Vec<String>) -> ItemList {
        let mut result = ItemList::new();

        let re = Regex::new("(?m)^(.)\\s(\\d+)\\s*,\\s*(\\d+)\\s\"([^\"]*)\"").unwrap();

        for string in items {
            let captures = re.captures(string).unwrap();
            let symbol = capture_symbol(&captures, 1);

            let x = capture_coordinate(&captures, 2);
            let y = capture_coordinate(&captures, 3);

            let description = capture_string(&captures, 4);
            let class = ItemClass::from_symbol(symbol);

            let item_id = NEXT_ITEM_ID();

            result.add(ItemState::Bundle(
                Item {
                    id: item_id,
                    class,
                    description: description.to_string(),
                    quantity: 1,
                },
                x,
                y,
            ));
        }

        result
    }

    pub fn read_in_stored_items(
        stored_items: &Vec<String>,
        aliases: AliasList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
    ) {
        let re = Regex::new("(?m)^(.)\\s(.+)\\s\"([^\"]*)\"").unwrap();

        for string in stored_items {
            let captures = re.captures(&string).unwrap();

            let symbol = capture_symbol(&captures, 1);
            let destination_alias = capture_string(&captures, 2);
            let description = capture_string(&captures, 3);

            let class = ItemClass::from_symbol(symbol);

            let destination_id = *aliases.get(&destination_alias).unwrap();

            let mut inventory = inventories.get_mut(&destination_id);
            if let None = inventory {
                inventory = Some(Inventory::new_into_inventory_list(
                    destination_id,
                    inventories,
                ));
            }
            if let Some(inventory) = inventory {
                let id = NEXT_ITEM_ID();
                let mut item = Item::new(id, class, description, 1);
                inventory.accept_stack(&mut item, items);
            } else {
                panic!("unable to find or create inventory")
            }
        }
    }

    /// returns true if the item can stack.
    /// # Examples:
    /// ```
    /// # use muframework::game::items::*;
    /// let item = Item::spawn(ItemClass::BladeWeapon, "Broadsword");
    /// assert!(!item.is_stackable());
    /// ```
    /// ```
    /// # use muframework::game::items::*;
    /// let item = Item::spawn_stack(ItemClass::Potion, "A Brown Potion",10);
    /// assert!(item.is_stackable());
    /// ```
    pub fn is_stackable(&self) -> bool {
        ItemClass::stack_limits(self.class) > 1
    }

    /// returns true if the two items are the same type.
    /// # Examples:
    /// ```
    /// use muframework::game::items::*;
    /// let item1 = Item::spawn(ItemClass::Potion, "a bubbly brew");
    /// let item2 = Item::spawn(ItemClass::Potion, "a pink potion");
    /// assert!(!item1.is_same_type_as(&item2));
    /// ```
    /// ```
    /// use muframework::game::items::*;
    /// let item = Item::spawn(ItemClass::Potion, "a bubbly brew");
    /// assert!(item.is_same_type_as(&item));
    /// ```

    // use description field and not method to avoid quantity inflections
    pub fn is_same_type_as(&self, other: &Item) -> bool {
        self.class == other.class && self.description == other.description
    }
}

#[derive(Debug, Clone)]
pub struct ItemList {
    next_id: u64,
    items: HashMap<u64, ItemState>,
}

impl ItemList {
    pub fn new() -> ItemList {
        ItemList {
            next_id: NEXT_ITEM_ID(),
            items: HashMap::new(),
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// returns true if the item list is empty.  false otherwise.
    /// # Examples:
    /// ```
    /// # use muframework::game::items::*;
    /// let subject = ItemList::new();
    /// assert!(subject.is_empty());
    /// ```
    /// ```
    /// # use muframework::game::items::*;
    /// let mut subject = ItemList::new();
    /// let item = Item::new(208, ItemClass::Potion, "A green, odorous potion",1);
    /// subject.store(&item, 1);
    /// assert!(!subject.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// returns the item state specified by item_id
    pub fn get(&self, item_id: u64) -> Option<ItemState> {
        self.items.get(&item_id).map(|i| i.clone())
    }

    /// retuns the item specified by item_id or None
    pub fn get_as_item(&self, item_id: u64) -> Option<Item> {
        let item_state = self.items.get(&item_id);

        if let Some(ref item_state) = item_state {
            Some(ItemState::extract_item(item_state))
        } else {
            None
        }
    }

    /// returns true if the ItemList has an element with the given id.
    #[inline]
    pub fn holds(&self, item_id: u64) -> bool {
        self.items.contains_key(&item_id)
    }

    pub fn find_nth_at(&self, x: i32, y: i32, n: i32) -> Option<&ItemState> {
        let item_state = self
            .items
            .iter()
            .filter(|(_i, item_state)| item_state.is_bundled_at(x, y))
            .nth(n as usize);

        match item_state {
            None => None,
            Some((_, item_state)) => Some(item_state),
        }
    }
    /// creates a bundle (or adds to an existing one) at the location given
    ///   for the given item.
    /// # Arguments
    /// * x,y  - the coordinates
    /// * item - the item to be added
    /// # Examples:
    /// ```
    /// # use muframework::game::items::*;
    /// let mut subject = ItemList::new();
    /// let mut item = Item::new(600, ItemClass::BladeWeapon, "A rusty sword",1);
    /// subject.add_item_to_bundle_at(10,15, &item);
    /// ```

    pub fn add_item_to_bundle_at(&mut self, x: i32, y: i32, item: &Item) {
        let bundle = ItemState::Bundle(item.to_owned(), x, y);
        self.add(bundle)
    }

    /// creates a bundle (or adds to an existing one) at the location given
    ///   for the with the given attributes.
    /// # Arguments
    /// * x,y  - the coordinates
    /// * id - the id to be added
    /// * class - the item_class of the new item
    /// * description - the description of the new item
    /// # Examples:
    /// ```
    /// # use muframework::game::items::*;
    /// let mut subject = ItemList::new();
    /// subject.add_new_item_to_bundle_at(10,15,600, ItemClass::Headwear, "A holey baseball cap" );
    /// ```
    pub fn add_new_item_to_bundle_at(
        &mut self,
        x: i32,
        y: i32,
        id: u64,
        class: ItemClass,
        description: &str,
    ) {
        let item = Item::new(id, class, description, 1);
        self.add_item_to_bundle_at(x, y, &item);
    }

    pub fn add(&mut self, item_state: ItemState) {
        let item = match item_state.clone() {
            ItemState::Bundle(i, _x, _y) => i,
            ItemState::Stored(i, _) => i,
            ItemState::Equipped(i, _) => i,
        };

        self.items.insert(item.id, item_state);
    }

    /// updates the item's item_state to reflect item.
    pub fn update_item(&mut self, item: &Item) {
        if let Some(item_state) = self.get(item.id) {
            match item_state {
                ItemState::Bundle(_i, x, y) => self.bundle(item, x, y),
                ItemState::Stored(_i, inventory_id) => self.store(item, inventory_id),
                ItemState::Equipped(_, _) => panic!("can't update equipment"),
            };
        } else {
            // do nothing if the item is not in the item list
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u64, ItemState> {
        self.items.iter()
    }

    pub fn bundle(&mut self, item: &Item, x: i32, y: i32) {
        self.items
            .insert(item.id, ItemState::Bundle(item.clone(), x, y));
    }

    pub fn store(&mut self, item: &Item, inventory_id: u64) {
        self.items
            .insert(item.id, ItemState::Stored(item.clone(), inventory_id));
    }

    pub fn equip(&mut self, item: &Item, inventory_id: u64) {
        self.items
            .insert(item.id, ItemState::Equipped(item.clone(), inventory_id));
    }
}

impl Index<u64> for ItemList {
    type Output = ItemState;

    fn index(&self, index: u64) -> &Self::Output {
        &self.items.index(&index)
    }
}

impl IndexMut<u64> for ItemList {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        if let None = self.items.get_mut(&index) {
            self.items.insert(
                index,
                ItemState::Stored(
                    Item {
                        description: "bogus item".to_string(),
                        class: ItemClass::Gloves,
                        id: index,
                        quantity: 1,
                    },
                    1,
                ),
            );
        }
        self.items.get_mut(&index).unwrap()
    }
}

#[cfg(test)]
mod item_list_index_mut {
    use super::*;

    #[test]
    fn test_returns_properly_when_given_a_non_existent_element() {
        let mut subject = ItemList::new();

        let item = Item {
            description: "blah".to_string(),
            class: ItemClass::Headwear,
            id: 1776,
            quantity: 1,
        };
        subject[1776] = ItemState::Equipped(item.clone(), 1);

        assert_eq!(subject[1776], ItemState::Equipped(item, 1));
    }
}

#[cfg(test)]
mod update_item {
    use super::*;
    use items::ItemClass::*;

    #[test]
    fn it_updates_a_stored_item() {
        let item = Item::spawn(Potion, "Coca-Cola");
        let item_id = item.id;
        let new_item = Item::new(item.id, item.class, item.description.clone(), 8);
        let mut subject = ItemList::new();

        subject.add(ItemState::Stored(item, item_id));

        subject.update_item(&new_item);

        let result = subject.get_as_item(item_id).expect("item not found");
        assert_eq!(result.quantity, 8);
    }
    #[test]
    fn it_updates_a_bundle_item() {
        let item = Item::spawn(Potion, "Coca-Cola");
        let item_id = item.id;
        let new_item = Item::new(item.id, item.class, item.description.clone(), 8);

        let mut subject = ItemList::new();

        subject.add(ItemState::Bundle(item, 10, 10));

        subject.update_item(&new_item);

        let result = subject.get_as_item(item_id).expect("item not found");
        assert_eq!(result.quantity, 8);
    }
}
