use super::*;
use inventory::{AliasList, InventoryList};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FacilityClass {
    ClosedChest,
    OpenChest,
    LockedChest,
    AppleTree,
    PineTree,
    OakTree,
}

impl FacilityClass {
    pub fn from_symbol(symbol: &str) -> FacilityClass {
        match symbol {
            "≡" => FacilityClass::ClosedChest,
            "▲r" => FacilityClass::AppleTree,
            _ => panic!("unknown FacilityClass symbol"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Facility {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub class: FacilityClass,
    pub description: String,
    pub inventory: Option<u64>,
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

        let has_inventory = [FacilityClass::ClosedChest, FacilityClass::OpenChest].contains(&class);
        if has_inventory {
            inventory_id = Some(Inventory::new_into_inventory_list(id, inventories).id());
        }

        Facility {
            id,
            x: x.try_into().ok().expect("Must be able to convert to i32"),
            y: y.try_into().ok().expect("Must be able to convert to i32"),
            class,
            description,
            inventory: inventory_id,
        }
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
        }
    }

    fn read_facility(
        captures: &'a regex::Captures,
        inventories: &'a mut InventoryList,
    ) -> (Facility, Option<&'a str>) {
        let symbol = captures.get(1).expect("unable to find symbol").as_str();

        let x = capture_coordinate(&captures, 2);
        let y = capture_coordinate(&captures, 3);
        let description = capture_string(&captures, 4);

        let inventory_alias: Option<&str> = captures.get(5).map(|m| m.as_str());

        let class = FacilityClass::from_symbol(symbol);

        (
            Facility::new(NEXT_ID(), x, y, class, description.into(), inventories),
            inventory_alias,
        )
    }

    fn read_facility_from_string(
        re: &regex::Regex,
        string: &str,
        aliases: &mut AliasList,
        result: &mut FacilityList,
        inventories: &mut InventoryList,
    ) {
        let captures = re.captures(string).unwrap();
        let (facility, possible_alias) = Self::read_facility(&captures, inventories);

        aliases.insert_if_necessary(possible_alias, facility.id);

        result.add(facility);
    }

    pub fn read_in_facilities(
        facilities: &Vec<String>,
        inventories: &mut InventoryList,
    ) -> (FacilityList, AliasList) {
        let mut aliases = AliasList::new(1);
        let mut result = FacilityList::new();

        let re = regex::Regex::new("(?m)^(.+)\\s(\\d+)\\s*,\\s*(\\d+)\\s\"([^\"]*)\"\\s*(\\w+)?")
            .unwrap();

        for string in facilities {
            Self::read_facility_from_string(&re, string, &mut aliases, &mut result, inventories);
        }

        (result, aliases)
    }

    pub fn is_in_use(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
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
}

#[cfg(test)]
mod inventory_aliases {
    use super::*;

    #[test]
    fn no_aliases_are_created_if_none_specified() {
        let mut inventories = InventoryList::new();
        let facility_src = vec![r#"▲r 9,9 "An old Apple Tree" "#.into()];
        let (_facilities, aliases) = Facility::read_in_facilities(&facility_src, &mut inventories);
        // remember to include "player" alias
        assert_eq!(aliases.len(), 1);
    }
    #[test]
    fn multiple_aliases_are_created_when_multiple_are_specified() {
        let mut inventories = InventoryList::new();
        let facility_src = vec![
            r#"▲r 9,9 "An old Apple Tree" alias1"#.into(),
            r#"▲r 10,10 "An old Apple Tree" alias2"#.into(),
            r#"▲r 11,11 "An old Apple Tree" alias3"#.into(),
        ];
        let (facilities, aliases) = Facility::read_in_facilities(&facility_src, &mut inventories);

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
        let items: &mut ItemList = &mut ItemList::new();
        let aliases = AliasList::new(1);
        let stored_items = vec![r#"^ player "A Green Leather Cap""#.into()];

        Item::read_in_stored_items(&stored_items, aliases, items, &mut inventories);

        assert_eq!(items.count(), 1);

        let inventory = inventories.get(&1).unwrap();
        assert_eq!(inventory.count(), 1);
    }
    #[test]
    fn read_in_stored_items_populated_alias_list() {
        let mut inventories = InventoryList::new();
        let items: &mut ItemList = &mut ItemList::new();
        let mut aliases = AliasList::new(1);
        aliases.insert("alias1", 502);

        let stored_items = vec![
            r#"^ alias1 "A Green Leather Cap""#.into(),
            r#"^ player "A Purple Leather Cap""#.into(),
            r#"^ alias1 "A Black Leather Cap""#.into(),
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
