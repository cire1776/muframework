use super::*;
use inventory::{AliasList, InventoryList};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum FacilityClass {
    ClosedChest,
    OpenChest,
    LockedChest,
    AppleTree,
    OliveTree,
    PineTree,
    OakTree,
    FruitPress,
    Lumbermill,
    Well,
    Vein,
    FishingSpot,
    Smeltery,
    Firepit,
    Patch,
    ConstructionSite,
}

impl FacilityClass {
    pub fn from_symbol(symbol: &str) -> FacilityClass {
        match symbol {
            "≡" => FacilityClass::ClosedChest,
            "▲r" => FacilityClass::AppleTree,
            "▲olive" => FacilityClass::OliveTree,
            "▲pine" => FacilityClass::PineTree,
            "▲oak" => FacilityClass::OakTree,
            "#press" => FacilityClass::FruitPress,
            "*lumbermill" => FacilityClass::Lumbermill,
            "▼well" => FacilityClass::Well,
            "#vein" => FacilityClass::Vein,
            "~fishing_spot" => FacilityClass::FishingSpot,
            "+smeltery" => FacilityClass::Smeltery,
            "^firepit" => FacilityClass::Firepit,
            "%patch" => FacilityClass::Patch,
            "+construction_site" => FacilityClass::ConstructionSite,
            _ => panic!("unknown FacilityClass symbol: {}", symbol),
        }
    }

    pub fn from_string<S: ToString>(string: S) -> Option<Self> {
        match &string.to_string()[..] {
            "closed_chest" => Some(FacilityClass::ClosedChest),
            "apple_tree" => Some(FacilityClass::AppleTree),
            "olive_tree" => Some(FacilityClass::OliveTree),
            "pine_tree" => Some(FacilityClass::PineTree),
            "oak_tree" => Some(FacilityClass::OakTree),
            "fruit_press" => Some(FacilityClass::FruitPress),
            "lumbermill" => Some(FacilityClass::Lumbermill),
            "well" => Some(FacilityClass::Well),
            "vein" => Some(FacilityClass::Vein),
            "fishing_spot" => Some(FacilityClass::FishingSpot),
            "smeltery" => Some(FacilityClass::Smeltery),
            "firepit" => Some(FacilityClass::Firepit),
            "patch" => Some(FacilityClass::Patch),
            "construction_site" => Some(FacilityClass::ConstructionSite),
            _ => None,
        }
    }
}

pub type NumericPropertyList = HashMap<String, i128>;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Facility {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub class: FacilityClass,
    pub description: String,
    pub inventory: Option<u64>,
    properties: Option<NumericPropertyList>,
    pub background_tile: tile_map::Tile,
}

impl<'a> Facility {
    pub fn new<T, U>(
        id: T,
        x: U,
        y: U,
        class: FacilityClass,
        description: String,
        inventories: &'a mut InventoryList,
    ) -> Facility
    where
        T: TryInto<u64>,
        U: TryInto<i32>,
    {
        let id = id.try_into().ok().expect("Must be able to convert to u64");
        let mut inventory_id: Option<u64> = None;

        if Facility::has_inventory(class) {
            inventory_id = Some(Inventory::new_into_inventory_list(id, inventories).id());
        }

        Facility {
            id,
            x: x.try_into().ok().expect("Must be able to convert to i32"),
            y: y.try_into().ok().expect("Must be able to convert to i32"),
            class,
            description,
            inventory: inventory_id,
            properties: None,
            background_tile: tile_map::Tile::Empty,
        }
    }

    pub fn variant(&self) -> u8 {
        use FacilityClass::*;

        match self.class {
            Well => self.get_property("fluid") as u8,
            Vein => (self.get_property("ore_type") - 1) as u8,
            ConstructionSite => (self.get_property("size")) as u8,
            _ => 0,
        }
    }

    pub fn has_inventory(class: FacilityClass) -> bool {
        [
            FacilityClass::ClosedChest,
            FacilityClass::OpenChest,
            FacilityClass::FruitPress,
            FacilityClass::FishingSpot,
        ]
        .contains(&class)
    }

    pub fn new_with_inventory<T, U>(
        id: T,
        x: U,
        y: U,
        class: FacilityClass,
        description: String,
        inventories: &'a mut InventoryList,
    ) -> Facility
    where
        T: TryInto<u64>,
        U: TryInto<i32>,
    {
        let id = id.try_into().ok().expect("Must be able to convert to u64");
        let inventory_id = id;

        let has_inventory = [FacilityClass::ClosedChest, FacilityClass::OpenChest].contains(&class);
        if !has_inventory {
            panic!("inventory assigned to facility without inventory");
        }

        Inventory::new_into_inventory_list(inventory_id, inventories);

        Facility {
            id,
            x: x.try_into().ok().expect("Must be able to convert to i32"),
            y: y.try_into().ok().expect("Must be able to convert to i32"),
            class,
            description,
            inventory: Some(inventory_id),
            properties: None,
            background_tile: tile_map::Tile::Empty,
        }
    }

    pub fn enable_properties(&mut self) {
        if self.properties == None {
            self.properties = Some(NumericPropertyList::new());
        }
    }

    pub fn set_property<S: ToString, N: TryInto<i128>>(&mut self, property_name: S, new_value: N) {
        match self.properties {
            Some(ref mut properties) => {
                let new_value = new_value
                    .try_into()
                    .ok()
                    .expect("must be convertible to i128");
                let value = properties
                    .entry(property_name.to_string())
                    .or_insert(new_value);
                *value = new_value;
            }
            None => panic!("properties not available for this facility"),
        }
    }

    pub fn get_property<S: ToString>(&self, property_name: S) -> i128 {
        if let Some(ref properties) = self.properties {
            *properties
                .get(&property_name.to_string())
                .or(Some(&0))
                .unwrap()
        } else {
            0
        }
    }

    pub fn increment_property<S: ToString>(&mut self, property_name: S) -> i128 {
        let value = self.get_property(property_name.to_string());
        let new_value = value + 1;
        self.set_property(property_name, new_value);
        new_value
    }

    pub fn decrement_property<S: ToString>(&mut self, property_name: S) -> i128 {
        let value = self.get_property(property_name.to_string());
        let new_value = value - 1;
        self.set_property(property_name, new_value);
        new_value
    }

    pub fn reawaken(&self, timer: &mut Timer) {
        self.setup_timers(timer)
    }
    pub fn setup_timers(&self, timer: &mut Timer) {
        match self.class {
            FacilityClass::AppleTree
            | FacilityClass::PineTree
            | FacilityClass::OliveTree
            | FacilityClass::OakTree => {
                timer.repeating_by_tick(
                    15 * 3600,
                    Command::FacilityMaintenance(self.id),
                    format!("regeneration for {:?}", self.id),
                );
            }
            _ => {}
        }
    }

    pub fn maintenance(&mut self) {
        match self.class {
            FacilityClass::AppleTree
            | FacilityClass::PineTree
            | FacilityClass::OliveTree
            | FacilityClass::OakTree => {
                self.increment_property("logs");
                println!("incrementing wood");
            }
            _ => {}
        }
        match self.class {
            FacilityClass::AppleTree | FacilityClass::OliveTree => {
                self.increment_property("fruit");
                println!("incrementing fruit");
            }
            _ => {}
        }
    }

    pub fn read_in_facilities(
        facilities: &Vec<String>,
        inventories: &mut InventoryList,
        timer: &mut Timer,
    ) -> (FacilityList, AliasList) {
        let mut aliases = AliasList::new(1);
        let mut result = FacilityList::new();

        let re =
            regex::Regex::new(r#"(?m)^(.+)\s(\d+)\s*,\s*(\d+)\s"([^"]*)"\s*(\w+)?(?:\{([^}]*)})?"#)
                .unwrap();

        let facilities_string = facilities.join("\n");

        for captures in re.captures_iter(&facilities_string) {
            let (facility, possible_alias) = Self::read_facility(&captures, inventories, timer);

            aliases.insert_if_necessary(possible_alias, facility.id);

            result.add(facility);
        }

        (result, aliases)
    }

    fn read_facility(
        captures: &'a regex::Captures,
        inventories: &'a mut InventoryList,
        timer: &'a mut Timer,
    ) -> (Facility, Option<&'a str>) {
        let symbol = captures.get(1).expect("unable to find symbol").as_str();

        let x = capture_coordinate(&captures, 2);
        let y = capture_coordinate(&captures, 3);
        let description = capture_string(&captures, 4);

        let inventory_alias: Option<&str> = captures.get(5).map(|m| m.as_str());

        let property_list_string = capture_optional_string(captures, 6).trim();

        let class = FacilityClass::from_symbol(symbol);
        let mut facility = Facility::new(NEXT_ID(), x, y, class, description.into(), inventories);

        Self::read_in_facility_properties(&mut facility, property_list_string);

        facility.setup_timers(timer);

        (facility, inventory_alias)
    }

    pub fn read_in_facility_properties(facility: &mut Facility, property_list_string: &str) {
        let re = regex::Regex::new(r#"(?m)^\s*property:\s*(\w+)\s*=>\s*(-?\d+)(?:\s*//.*)?$"#)
            .expect("unable to parse propertylist");

        for captures in re.captures_iter(property_list_string) {
            let property_name = capture_string(&captures, 1);
            let property_value = capture_string(&captures, 2).parse::<i128>().unwrap();

            facility.enable_properties();
            facility.set_property(property_name, property_value);
        }
    }

    pub fn is_in_use(&self) -> bool {
        self.get_property("is_in_use") != 0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FacilityList {
    facilities: HashMap<u64, Facility>,
}

impl FacilityList {
    pub fn new() -> FacilityList {
        FacilityList {
            facilities: HashMap::new(),
        }
    }

    /// returns the number of facilities in the list
    /// # Examples:
    /// ```
    /// # use muframework::game::facility::*;
    /// # use muframework::game::items::InventoryList;
    /// let mut inventories = &mut InventoryList::new();
    /// let mut subject = FacilityList::new();
    /// let facility = Facility::new(823,10,10,FacilityClass::AppleTree,"an old apple tree".into(),inventories);
    /// subject.add(facility);
    /// assert_eq!(subject.count(),1);
    /// ```
    pub fn count(&self) -> usize {
        self.facilities.len()
    }

    /// returns true if the facility list is empty.  false otherwise.
    /// # Examples:
    /// ```
    /// # use muframework::game::facility::*;
    /// let subject = FacilityList::new();
    /// assert!(subject.is_empty());
    /// ```
    /// ```
    /// # use muframework::game::facility::*;
    /// # use muframework::game::items::InventoryList;
    /// let mut subject = FacilityList::new();
    /// let inventories = &mut InventoryList::new();
    /// let facility = Facility::new(823,10,10,FacilityClass::AppleTree,"an old apple tree".into(),inventories);
    /// subject.add(facility);
    /// assert!(!subject.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.facilities.is_empty()
    }

    /// returns the specified by item_id
    /// # Examples:
    /// ```
    /// # use muframework::game::facility::*;
    /// # use muframework::game::items::InventoryList;
    /// let mut subject = FacilityList::new();
    /// let mut inventories = &mut InventoryList::new();
    /// let facility = Facility::new(820,10,10,FacilityClass::AppleTree,"an old apple tree".into(), inventories);
    /// subject.add(facility.clone());
    /// assert_eq!(subject.get(820),Some(&facility));
    /// ```
    pub fn get<T: TryInto<u64>>(&self, item_id: T) -> Option<&Facility> {
        self.facilities.get(
            &item_id
                .try_into()
                .ok()
                .expect("Must be able to convert to i32"),
        )
    }

    pub fn get_mut<T: TryInto<u64>>(&mut self, item_id: T) -> Option<&mut Facility> {
        self.facilities.get_mut(
            &item_id
                .try_into()
                .ok()
                .expect("Must be able to convert to i32"),
        )
    }

    pub fn at(&self, x: i32, y: i32) -> Option<&Facility> {
        self.facilities
            .iter()
            .find(|(_, f)| f.x == x && f.y == y)
            .map(|(_, f)| f)
    }

    /// adds a facility to the list.
    /// # Examples:
    /// ```
    /// # use muframework::game::facility::*;
    /// # use muframework::game::items::InventoryList;
    /// let mut subject = FacilityList::new();
    /// let mut inventories = &mut InventoryList::new();
    /// let facility = Facility::new(820,10,10,FacilityClass::AppleTree,"an old apple tree".into(),inventories);
    /// subject.add(facility);
    /// assert_eq!(subject.count(),1);
    /// ```
    pub fn add(&mut self, facility: Facility) {
        self.facilities.insert(facility.id, facility.clone());
    }

    pub fn remove(&mut self, facility_id: u64) {
        self.facilities.remove(&facility_id);
    }
}

impl Index<u64> for FacilityList {
    type Output = Facility;

    fn index(&self, index: u64) -> &Self::Output {
        &self.facilities.index(&index)
    }
}

impl IndexMut<u64> for FacilityList {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        if let None = self.facilities.get_mut(&index) {
            self.facilities.insert(
                index,
                Facility {
                    id: index,
                    x: 0,
                    y: 0,
                    class: FacilityClass::ClosedChest,
                    description: "".into(),
                    inventory: Some(u64::MAX),
                    properties: None,
                    background_tile: tile_map::Tile::Empty,
                },
            );
        }
        self.facilities.get_mut(&index).unwrap()
    }
}

impl FacilityList {
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u64, Facility> {
        self.facilities.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, u64, game::facility::Facility> {
        self.facilities.iter_mut()
    }
}

#[cfg(test)]
mod inventory_aliases {
    use super::*;

    #[test]
    fn no_aliases_are_created_if_none_specified() {
        let mut timer = Timer::new(None);
        let mut inventories = InventoryList::new();
        let facility_src = vec![r#"▲r 9,9 "An old Apple Tree" "#.into()];
        let (_facilities, aliases) =
            Facility::read_in_facilities(&facility_src, &mut inventories, &mut timer);
        // remember to include "player" alias
        assert_eq!(aliases.len(), 1);
    }
    #[test]
    fn multiple_aliases_are_created_when_multiple_are_specified() {
        let mut timer = Timer::new(None);
        let mut inventories = InventoryList::new();
        let facility_src = vec![
            r#"▲r 9,9 "An old Apple Tree" alias1"#.into(),
            r#"▲r 10,10 "An old Apple Tree" alias2"#.into(),
            r#"▲r 11,11 "An old Apple Tree" alias3"#.into(),
        ];
        let (facilities, aliases) =
            Facility::read_in_facilities(&facility_src, &mut inventories, &mut timer);

        // remember to include "player" alias
        assert_eq!(aliases.len(), 4);

        let mut ids: Vec<u64> = facilities.facilities.iter().map(|(id, _f)| *id).collect();
        ids.sort();

        for (index, id) in ids.iter().enumerate() {
            assert_eq!(aliases.get(&format!("alias{}", index + 1)).unwrap(), id);
        }
    }

    #[test]
    fn read_in_stored_items_using_empty_alias_list() {
        let mut inventories = InventoryList::new();
        let items: &mut ItemList = &mut ItemList::new(None);
        items.item_types = ItemTypeList::new();
        items.item_types.insert(
            "green_leather_cap",
            ItemType::new(ItemClass::Headwear, "Green Leather cap"),
        );
        let aliases = AliasList::new(1);
        let stored_items = vec![r#"player green_leather_cap"#.into()];

        Item::read_in_stored_items(&stored_items, aliases, items, &mut inventories);

        assert_eq!(items.count(), 1);

        let inventory = inventories.get(&1).unwrap();
        assert_eq!(inventory.count(), 1);
    }

    #[test]
    fn read_in_stored_items_populated_alias_list() {
        let mut inventories = InventoryList::new();
        let items: &mut ItemList = &mut ItemList::new(None);
        items.item_types = ItemTypeList::new();
        items.item_types.insert(
            "green_leather_cap",
            ItemType::new(ItemClass::Headwear, "Green Leather cap"),
        );
        items.item_types.insert(
            "purple_leather_cap",
            ItemType::new(ItemClass::Headwear, "Purple Leather cap"),
        );
        items.item_types.insert(
            "black_leather_cap",
            ItemType::new(ItemClass::Headwear, "Black Leather cap"),
        );

        let mut aliases = AliasList::new(1);
        aliases.insert("alias1", 502);

        let stored_items = vec![
            r#"alias1 green_leather_cap"#.into(),
            r#"player purple_leather_cap"#.into(),
            r#"alias1 black_leather_cap"#.into(),
        ];

        Item::read_in_stored_items(&stored_items, aliases, items, &mut inventories);

        assert_eq!(items.count(), 3);

        let inventory = inventories.get(&1).unwrap();
        assert_eq!(inventory.count(), 1);
        let inventory = inventories.get(&502).unwrap();
        assert_eq!(inventory.count(), 2);
    }
}
#[cfg(test)]
mod chests {
    use super::*;

    #[test]
    fn inventory_is_created_along_with_facility() {
        let mut inventories = InventoryList::new();
        let subject = Facility::new_with_inventory(
            1247,
            10,
            10,
            FacilityClass::ClosedChest,
            "A Chest".into(),
            &mut inventories,
        );

        assert_eq!(subject.inventory, Some(1247));

        if let Some(inventory) = inventories.get(&1247) {
            assert_eq!(inventory.id(), 1247);
            assert!(inventory.is_empty());
        } else {
            panic!("inventory not found");
        }
    }
}

#[cfg(test)]
mod properties {
    use super::*;

    #[test]
    fn increment_increments() {
        let mut inventories = InventoryList::new();

        let mut subject = Facility::new(
            NEXT_ID(),
            10,
            10,
            FacilityClass::FruitPress,
            "a fruit press".to_string(),
            &mut inventories,
        );

        subject.enable_properties();

        assert_eq!(subject.increment_property("test"), 1);
        assert_eq!(subject.get_property("test"), 1);
    }

    #[test]
    fn decrement_decrements() {
        let mut inventories = InventoryList::new();

        let mut subject = Facility::new(
            NEXT_ID(),
            10,
            10,
            FacilityClass::FruitPress,
            "a fruit press".to_string(),
            &mut inventories,
        );

        subject.enable_properties();
        subject.set_property("test", 1);
        assert_eq!(subject.decrement_property("test"), 0);
        assert_eq!(subject.get_property("test"), 0);
    }
}

#[cfg(test)]
mod facility_list {
    use super::*;

    #[test]
    fn at_returns_the_facility_at_its_coordinates() {
        let mut inventories = InventoryList::new();
        let mut subject = FacilityList::new();
        let facility = Facility::new(
            2,
            10,
            10,
            FacilityClass::LockedChest,
            "description".to_string(),
            &mut inventories,
        );
        subject.add(facility.clone());

        assert_eq!(subject.at(10, 10), Some(&facility));
    }

    #[test]
    fn remove_does_nothing_if_given_a_bogus_id() {
        let mut subject = FacilityList::new();
        subject.remove(1776);
    }

    #[test]
    fn remove_removes_an_existing_facility() {
        let mut inventories = InventoryList::new();
        let mut subject = FacilityList::new();
        let facility = Facility::new(
            1776,
            10,
            10,
            FacilityClass::Well,
            "description".to_string(),
            &mut inventories,
        );
        subject.add(facility.clone());

        subject.remove(1776);

        assert!(subject.get(1776).is_none());
    }
}
