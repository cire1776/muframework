pub use super::*;
use regex::Regex;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use ItemClass::*;

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
            "↓" => Dagger,
            "^" => Headwear,
            "!" => BladeWeapon,
            "¡" => Potion,
            "♠" => Tool,
            _ => panic!("unknown item class"),
        }
    }

    pub fn from_name<S: ToString>(name: S) -> ItemClass {
        match &name.to_string()[..] {
            "bladeweapon" => BladeWeapon,
            "dagger" => Dagger,
            "shield" => Shield,
            "softarmor" => SoftArmor,
            "pants" => Pants,
            "gloves" => Gloves,
            "shoes" => Shoes,
            "headwear" => Headwear,
            "tool" => Tool,
            "potion" => Potion,
            "food" => Food,
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
pub trait StaticData: 'static {}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ItemType {
    class: ItemClass,
    description: String,
    endorsements: Vec<String>,
}

impl ItemType {
    pub fn new<S: ToString>(class: ItemClass, description: S) -> Self {
        Self {
            class,
            description: description.to_string(),
            endorsements: vec![],
        }
    }

    pub fn add_endorsement<S: ToString>(&mut self, endorsement: S) {
        self.endorsements.push(endorsement.to_string());
    }

    pub fn read_in_item_types(items: &mut Vec<String>) -> ItemTypeList {
        let mut result = ItemTypeList::new();
        let long_string = items.join("\n");

        let re = Regex::new(r#"(?m)^(\w+)\s+(\w+)\s*\s"([^"]+)"\s*(?:\{([^}]*)*\})?(?:\s*//.*)?$"#)
            .unwrap();

        for captures in re.captures_iter(&long_string[..]) {
            let type_name = capture_string(&captures, 1);
            let class_name = capture_string(&captures, 2);
            let description = capture_string(&captures, 3);
            let attributes = capture_optional_string(&captures, 4).trim();

            let class = ItemClass::from_name(class_name);

            let mut new_type = ItemType::new(class, description);

            Self::read_in_type_attributes_for(&mut new_type, attributes);
            result.insert(type_name.to_string(), new_type);
        }

        result
    }

    fn read_in_type_attributes_for(new_type: &mut ItemType, attributes: &str) {
        if attributes.is_empty() {
            return;
        }

        let re = Regex::new(r#"^(endorsement):\s+(:\w+|\d+)(?:\s*//.*)?$"#).unwrap();

        let captures = re.captures(attributes).expect("unable to parse attribute");

        let attribute_name = capture_string(&captures, 1);
        let attribute_value = capture_string(&captures, 2);

        match attribute_name {
            "endorsement" => new_type.add_endorsement(attribute_value),
            _ => panic!("unrecognized attribute: {}", attribute_name),
        }
    }
}

impl StaticData for ItemType {}

pub type ItemTypeList = HashMap<String, ItemType>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Item {
    pub id: u64,
    pub quantity: u8,
    pub item_type: ItemType,
}

impl Item {
    pub fn new<N: TryInto<u8>>(id: u64, item_type: ItemType, quantity: N) -> Item {
        Item {
            id,
            quantity: quantity.try_into().ok().expect("must be convertible to u8"),
            item_type,
        }
    }
    pub fn class(&self) -> ItemClass {
        self.item_type.class
    }

    pub fn raw_description(&self) -> String {
        self.item_type.description.clone()
    }

    pub fn description(&self) -> String {
        use inflector::string::pluralize::to_plural;
        use inflector::string::singularize::to_singular;

        let vowels = ["A", "E", "I", "O", "U"];

        let prefix = if self.quantity == 1 {
            if vowels.contains(&&self.raw_description().to_uppercase()[0..1]) {
                "An".to_string()
            } else {
                "A".to_string()
            }
        } else {
            format!("{}", self.quantity).to_string()
        };

        let inflected_description = if self.quantity == 1 {
            to_singular(&self.raw_description()[..])
        } else {
            to_plural(&self.raw_description()[..])
        };

        format!("{} {}", prefix, inflected_description).clone()
    }

    pub fn endorse(&self,player: &mut Player) {
        for endorsement in &self.item_type.endorsements {
            player.endorse_with(endorsement);
        }
    }

    /// creates a new item assigning it its Id as appropriate.
    pub fn spawn<S: ToString>(class: ItemClass, description: S) -> Item {
        Self::spawn_stack(class, description, 1)
    }

    pub fn spawn_from_type<S: ToString, N: TryInto<u8>>(
        item_type_name: S,
        quantity: N,
        item_types: &ItemTypeList,
    ) -> Item {
        let item_type = &item_types[&item_type_name.to_string()];

        Self::new(
            NEXT_ITEM_ID(),
            item_type.clone(),
            quantity.try_into().ok().expect("must be convertible to u8"),
        )
    }

    pub fn spawn_stack<S: ToString, N: TryInto<u8>>(
        class: ItemClass,
        description: S,
        quantity: N,
    ) -> Item {
        let description = description.to_string();
        Self::new(
            NEXT_ITEM_ID(),
            ItemType::new(class, description),
            quantity.try_into().ok().expect("must be convertible to u8"),
        )
    }
    pub fn read_in_items(items: &Vec<String>, item_types: ItemTypeList) -> ItemList {
        let mut result = ItemList::new(Some(item_types));

        let re = Regex::new("(?m)^(.)\\s(\\d+)\\s*,\\s*(\\d+)\\s(\\w+)").unwrap();

        for string in items {
            let captures = re.captures(string).unwrap();
            let symbol = capture_symbol(&captures, 1);

            let x = capture_coordinate(&captures, 2);
            let y = capture_coordinate(&captures, 3);

            let item_type_name = capture_string(&captures, 4);

            let _class = ItemClass::from_symbol(symbol);

            let item = Item::spawn_from_type(item_type_name, 1, &result.item_types);
            result.bundle(&item, x, y);
        }

        result
    }

    pub fn read_in_stored_items(
        stored_items: &Vec<String>,
        aliases: AliasList,
        items: &mut ItemList,
        inventories: &mut InventoryList,
    ) {
        let re = Regex::new("(?m)^(\\w+)\\s(\\w+)").unwrap();

        for string in stored_items {
            let captures = re.captures(&string).unwrap();

            let destination_alias = capture_string(&captures, 1);
            let item_type_name = capture_string(&captures, 2);

            let destination_id = *aliases.get(&destination_alias).unwrap();

            let mut inventory = inventories.get_mut(&destination_id);
            if let None = inventory {
                inventory = Some(Inventory::new_into_inventory_list(
                    destination_id,
                    inventories,
                ));
            }
            if let Some(inventory) = inventory {
                let mut item = Item::spawn_from_type(item_type_name, 1, &items.item_types);
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
        ItemClass::stack_limits(self.class()) > 1
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
        self.item_type == other.item_type
    }
}

#[derive(Debug, Clone)]
pub struct ItemList {
    items: HashMap<u64, ItemState>,
    pub item_types: ItemTypeList,
}

impl ItemList {
    pub fn new(item_types: Option<ItemTypeList>) -> ItemList {
        let item_types = if let Some(item_types) = item_types {
            item_types
        } else {
            ItemTypeList::new()
        };

        ItemList {
            items: HashMap::new(),
            item_types,
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
    /// let subject = ItemList::new(None);
    /// assert!(subject.is_empty());
    /// ```
    /// ```
    /// # use muframework::game::items::*;
    /// let mut subject = ItemList::new(None);
    /// let item = Item::new(208, ItemType::new(ItemClass::Potion, "A green, odorous potion"),1);
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
    /// let mut subject = ItemList::new(None);
    /// let mut item = Item::new(600, ItemType::new(ItemClass::BladeWeapon, "A rusty sword"),1);
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
    /// let mut subject = ItemList::new(None);
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
        let item = Item::new(id, ItemType::new(class, description), 1);
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
                        id: index,
                        quantity: 1,
                        item_type: ItemType::new(ItemClass::Gloves, "description"),
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
        let mut subject = ItemList::new(None);

        let item = Item {
            id: 1776,
            quantity: 1,
            item_type: ItemType::new(Headwear, "blah"),
        };
        subject[1776] = ItemState::Equipped(item.clone(), 1);

        assert_eq!(subject[1776], ItemState::Equipped(item, 1));
    }
}

#[cfg(test)]
mod update_item {
    use super::*;

    #[test]
    fn it_updates_a_stored_item() {
        let item = Item::spawn(Potion, "Coca-Cola");
        let item_id = item.id;
        let new_item = Item::new(item.id, ItemType::new(item.class(), item.description()), 8);
        let mut subject = ItemList::new(None);

        subject.add(ItemState::Stored(item, item_id));

        subject.update_item(&new_item);

        let result = subject.get_as_item(item_id).expect("item not found");
        assert_eq!(result.quantity, 8);
    }
    #[test]
    fn it_updates_a_bundle_item() {
        let item = Item::spawn(Potion, "Coca-Cola");
        let item_id = item.id;
        let new_item = Item::new(item.id, ItemType::new(item.class(), item.description()), 8);

        let mut subject = ItemList::new(None);

        subject.add(ItemState::Bundle(item, 10, 10));

        subject.update_item(&new_item);

        let result = subject.get_as_item(item_id).expect("item not found");
        assert_eq!(result.quantity, 8);
    }
}
