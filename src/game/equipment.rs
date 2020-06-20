use super::*;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum MountingPoint {
    Head,
    Face,
    Neck,
    Shoulders,
    Chest,
    Waist,
    Legs,
    Feet,
    Arms,
    LeftWrist,
    RightWrist,
    Hands,
    OnHand,
    OffHand,
    AtReady,
}

static ALL_MOUNTING_POINTS: [MountingPoint; 15] = [
    MountingPoint::Head,
    MountingPoint::Face,
    MountingPoint::Neck,
    MountingPoint::Shoulders,
    MountingPoint::Chest,
    MountingPoint::Waist,
    MountingPoint::Legs,
    MountingPoint::Feet,
    MountingPoint::Arms,
    MountingPoint::LeftWrist,
    MountingPoint::RightWrist,
    MountingPoint::Hands,
    MountingPoint::OnHand,
    MountingPoint::OffHand,
    MountingPoint::AtReady,
];

#[derive(Debug, Clone)]
pub struct MountingPointMap {
    mounts: HashMap<MountingPoint, Option<u64>>,
}

impl MountingPointMap {
    pub fn new() -> MountingPointMap {
        let mut result = MountingPointMap {
            mounts: HashMap::new(),
        };

        for mounting_point in ALL_MOUNTING_POINTS.iter() {
            result.mounts.insert(*mounting_point, None);
        }
        result
    }

    pub fn is_empty(&self, mp: &MountingPoint) -> bool {
        match self.mounts.get(&mp).expect("mounting point not found") {
            None => true,
            _ => false,
        }
    }

    /// returns the item id of the item mounted at the given mounting point or none
    /// Examples:
    /// ```
    /// # use muframework::game::equipment::*;
    /// let subject = MountingPointMap::new();
    /// assert!(subject.at(&MountingPoint::Head) == None);
    /// ```
    /// ```
    /// # use muframework::game::equipment::*;
    /// # use muframework::game::items::*;
    /// let mut subject = MountingPointMap::new();
    /// let item_class_specifiers = ItemClassSpecifier::initialize();
    /// let inventory = &mut Inventory::new(1002);
    /// let items = &mut ItemList::new(None);
    /// let hat = Item::new(958, ItemType::new(ItemClass::Headwear, "a trusty cap"),1);
    /// items.store(&hat,1002);
    ///
    /// subject.mount(&hat, &item_class_specifiers, inventory, items);
    ///
    /// assert_eq!(subject.at(&MountingPoint::Head), Some(hat.id));
    /// ```
    pub fn at(&self, mounting_point: &MountingPoint) -> Option<u64> {
        self.mounts[mounting_point]
    }

    pub fn to_vec(&self) -> Vec<u64> {
        let mut result: Vec<u64> = self
            .mounts
            .values()
            .filter(|mp| if let None = mp { false } else { true })
            .map(|f| f.unwrap())
            .collect();

        result.sort();
        result.dedup();
        result
    }

    pub fn to_vec_of_items(&self, items: &ItemList) -> Vec<Item> {
        self.to_vec()
            .iter()
            .map(|i| ItemState::extract_item(&items[*i]))
            .collect()
    }

    pub fn endorse(&self, player: &mut Player, items: &ItemList) {
        let all_equipment = self.to_vec_of_items(items);

        for equipment in all_equipment {
            equipment.endorse(player);
        }
    }

    // TODO: allow for multiple mounting points
    fn perform_mount(&mut self, item: &Item, mounting_points: &Vec<&MountingPoint>) {
        if mounting_points.is_empty() {
            return;
        }

        let new_item_id = item.id;
        self.mounts.insert(*mounting_points[0], Some(new_item_id));
    }

    // TODO: accept multiple mounting points
    pub fn mount(
        &mut self,
        item: &Item,
        // mounting_points: &Vec<&MountingPoint>,
        item_class_specifiers: &ItemClassSpecifierList,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) {
        let mounting_points = ItemClassSpecifier::mounting_points_for(&item, item_class_specifiers);

        if mounting_points.is_empty() {
            return;
        }

        let previous = self
            .mounts
            .get(mounting_points[0])
            .expect("previous not found");

        if let Some(previous_id) = previous {
            Self::unmount_previous_item(*previous_id, inventory, items)
        }

        if let ItemState::Stored(new_item, _inventory_id) = items[item.id].clone() {
            self.mount_new_item(&new_item, &mounting_points, inventory, items)
        };
    }

    fn unmount_previous_item(previous_id: u64, inventory: &mut Inventory, items: &mut ItemList) {
        if let ItemState::Equipped(previous_item, inventory_id) = &items[previous_id].to_owned() {
            items[previous_id] = ItemState::Stored(previous_item.clone(), *inventory_id);
            inventory.accept_stack_unmut(&previous_item, items)
        } else {
            panic!("Previous item not equipped.")
        }
    }

    fn mount_new_item(
        &mut self,
        new_item: &Item,
        mounting_points: &Vec<&MountingPoint>,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) {
        let new_item_id = new_item.id;
        self.perform_mount(&new_item, mounting_points);
        items[new_item_id] = ItemState::Equipped(new_item.clone(), inventory.id());
        inventory.release_item(&new_item_id);
    }

    pub fn unmount_item_by_id(
        &mut self,
        item_id: u64,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) {
        if self.mounts.iter().any(|(_mp, item)| item == &Some(item_id)) {
            self.force_unmount_item_by_id(item_id);
            inventory.accept_by_id(item_id, items);
        }
    }

    fn force_unmount_item_by_id(&mut self, item_id: u64) {
        for (_, id) in self.mounts.iter_mut() {
            // set item_id elements to None
            if *id == Some(item_id) {
                *id = None;
            }
        }
    }

    pub fn unmount(
        &mut self,
        mounting_points: &Vec<&MountingPoint>,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) {
        for mounting_point in mounting_points {
            let mounted_item = self.mounts.insert(**mounting_point, None);
            if let Some(Some(item_id)) = mounted_item {
                println!("unmounted: {:?}", item_id);
                let mut item = items.get_as_item(item_id).unwrap();
                inventory.accept_stack(&mut item, items);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemClassSpecifier {
    pub mounting_points: HashSet<MountingPoint>,
}

pub type ItemClassSpecifierList = HashMap<ItemClass, ItemClassSpecifier>;

impl ItemClassSpecifier {
    pub fn initialize() -> ItemClassSpecifierList {
        let mut item_class_specifiers = ItemClassSpecifierList::new();

        item_class_specifiers.insert(
            ItemClass::BladeWeapon,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::OnHand].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Dagger,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::OnHand].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Shield,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::OffHand].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::SoftArmor,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::Chest].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Pants,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::Legs].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Gloves,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::Hands].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Shoes,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::Feet].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Headwear,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::Head].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Tool,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::OnHand].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Potion,
            ItemClassSpecifier {
                mounting_points: [].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Food,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::AtReady].iter().cloned().collect(),
            },
        );
        item_class_specifiers.insert(
            ItemClass::Material,
            ItemClassSpecifier {
                mounting_points: [MountingPoint::AtReady].iter().cloned().collect(),
            },
        );
        item_class_specifiers
    }
    pub fn to_vec<'a>(&'a self) -> Vec<&'a MountingPoint> {
        self.mounting_points.iter().collect()
    }

    pub fn mounting_points_for<'a>(
        item: &Item,
        item_class_specifiers: &'a ItemClassSpecifierList,
    ) -> Vec<&'a MountingPoint> {
        item_class_specifiers[&item.class()].to_vec()
    }
}

#[cfg(test)]
mod mounting_point_map {
    use super::*;

    #[test]
    fn mount_mounts_item_at_specified_mounting_point() {
        let mut subject = MountingPointMap::new();
        let mut items = ItemList::new(None);
        let item_class_specifiers = ItemClassSpecifier::initialize();
        let item = Item {
            id: 1,
            quantity: 1,
            item_type: ItemType::new(ItemClass::Headwear, "hat"),
        };
        items[1] = ItemState::Stored(item.clone(), 1);

        let mut inventory = Inventory::new(1);

        subject.mount(&item, &item_class_specifiers, &mut inventory, &mut items);

        assert!(!subject.is_empty(&MountingPoint::Head));
    }

    #[test]
    fn mount_changes_items_state_from_equipped_to_stored_and_vice_versa() {
        let mut subject = MountingPointMap::new();

        let mut items = ItemList::new(None);

        let item = Item {
            id: 1,
            quantity: 1,
            item_type: ItemType::new(ItemClass::Headwear, "hat"),
        };
        items[1] = ItemState::Stored(item.clone(), 1);

        let old_item = Item {
            id: 2,
            quantity: 1,
            item_type: ItemType::new(ItemClass::Headwear, "old hat"),
        };
        items[2] = ItemState::Stored(old_item.clone(), 1);

        let mut inventory = Inventory::new(1);
        let item_class_specifiers = ItemClassSpecifier::initialize();

        subject.mount(
            &old_item,
            &item_class_specifiers,
            &mut inventory,
            &mut items,
        );

        assert!(!subject.is_empty(&MountingPoint::Head));

        subject.mount(&item, &item_class_specifiers, &mut inventory, &mut items);

        assert_eq!(items[1], ItemState::Equipped(item, 1));
        assert_eq!(items[2], ItemState::Stored(old_item, 1));
    }

    #[test]
    fn mount_items_unequipped_are_addded_to_the_inventory() {
        let mut subject = MountingPointMap::new();

        let mut items = ItemList::new(None);
        let mut inventory = Inventory::new(1);

        let item_class_specifiers = ItemClassSpecifier::initialize();

        let mut item = Item {
            id: 1,
            quantity: 1,
            item_type: ItemType::new(ItemClass::Headwear, "hat"),
        };
        items[1] = ItemState::Stored(item.clone(), 1);
        inventory.accept_stack(&mut item, &mut items);

        let mut old_item = Item {
            id: 2,
            quantity: 1,
            item_type: ItemType::new(ItemClass::Headwear, "old hat"),
        };
        items[2] = ItemState::Stored(old_item.clone(), 1);
        inventory.accept_stack(&mut old_item, &mut items);

        subject.mount(
            &old_item,
            &item_class_specifiers,
            &mut inventory,
            &mut items,
        );

        assert!(!subject.is_empty(&MountingPoint::Head));

        subject.mount(&item, &item_class_specifiers, &mut inventory, &mut items);

        assert_eq!(inventory[2], old_item);
    }

    #[test]
    #[should_panic(expected = "index not found")]
    fn mount_items_equipped_are_removed_from_the_inventory() {
        let mut subject = MountingPointMap::new();

        let mut items = ItemList::new(None);
        let mut inventory = Inventory::new(1);

        let item_class_specifiers = ItemClassSpecifier::initialize();
        let item = test_item(
            "a worn leather hat",
            ItemClass::Headwear,
            1,
            &mut items,
            &mut inventory,
        );

        let old_item = test_item(
            "A worn hat",
            ItemClass::Headwear,
            2,
            &mut items,
            &mut inventory,
        );

        subject.mount(
            &old_item,
            &item_class_specifiers,
            &mut inventory,
            &mut items,
        );

        assert!(!subject.is_empty(&MountingPoint::Head));

        subject.mount(&item, &item_class_specifiers, &mut inventory, &mut items);

        &inventory[1];
    }

    #[test]
    fn to_vec_returns_an_empty_array_when_nothing_equipped() {
        let subject = MountingPointMap::new();

        let result = subject.to_vec();

        assert_eq!(result, vec![]);
    }

    fn test_item(
        description: &str,
        class: ItemClass,
        id: u64,
        items: &mut ItemList,
        inventory: &mut Inventory,
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

    #[test]
    fn to_vec_returns_an_array_with_multiple_items_when_multiple_item_are_equipped() {
        let mut subject = MountingPointMap::new();

        let mut items = ItemList::new(None);
        let mut inventory = Inventory::new(1);

        let item1 = test_item(
            "A hat".into(),
            ItemClass::Headwear,
            1,
            &mut items,
            &mut inventory,
        );
        let item2 = test_item(
            "gloves".into(),
            ItemClass::Gloves,
            2,
            &mut items,
            &mut inventory,
        );
        let item3 = test_item(
            "sword".into(),
            ItemClass::BladeWeapon,
            3,
            &mut items,
            &mut inventory,
        );

        subject.perform_mount(&item1, &vec![&MountingPoint::Head]);
        subject.perform_mount(&item2, &vec![&MountingPoint::Hands]);
        subject.perform_mount(&item3, &vec![&MountingPoint::OnHand]);

        let result = subject.to_vec();
        assert_eq!(result, vec![item1.id, item2.id, item3.id]);
    }
}

#[cfg(test)]
mod mounting_point_map_unmount {
    use super::*;

    fn test_item(
        description: &str,
        class: ItemClass,
        id: u64,
        items: &mut ItemList,
        inventory: &mut Inventory,
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
    #[test]
    fn given_no_mounting_points() {
        let mut subject = MountingPointMap::new();
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);

        let item = Item::new(232, ItemType::new(ItemClass::Headwear, "An Amazon Cap"), 1);
        subject.perform_mount(&item, &vec![&MountingPoint::Head]);

        subject.unmount(&vec![], &mut inventory, &mut items);

        assert!(items.is_empty());
        assert!(inventory.is_empty());
    }

    #[test]
    fn given_an_empty_mounting_point() {
        let mut subject = MountingPointMap::new();
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);

        let item = Item::new(232, ItemType::new(ItemClass::Headwear, "An Amazon Cap"), 1);
        subject.perform_mount(&item, &vec![&MountingPoint::Head]);

        subject.unmount(&vec![&MountingPoint::OnHand], &mut inventory, &mut items);

        assert!(items.is_empty());
        assert!(inventory.is_empty());
    }
    #[test]
    fn given_an_item_in_the_mounting_point() {
        let mut subject = MountingPointMap::new();
        let mut inventory = Inventory::new(1);
        let mut items = ItemList::new(None);
        let item_mounting_points = &vec![&MountingPoint::Head];
        let item = test_item(
            "A hat".into(),
            ItemClass::Headwear,
            1,
            &mut items,
            &mut inventory,
        );
        subject.perform_mount(&item, item_mounting_points);

        subject.unmount(item_mounting_points, &mut inventory, &mut items);

        assert!(subject.is_empty(item_mounting_points[0]));
        assert!(!items.is_empty());
        assert!(!inventory.is_empty());
    }
}

#[cfg(test)]
mod at_ready {
    use super::*;

    #[test]
    fn all_mounting_points_has_at_ready() {
        assert!(ALL_MOUNTING_POINTS.contains(&MountingPoint::AtReady));
    }

    #[test]
    fn food_mounts_at_at_ready() {
        let mut subject = MountingPointMap::new();
        let item_class_specifiers = ItemClassSpecifier::initialize();
        let inventory = &mut Inventory::new(1114);
        let mut item_types = ItemTypeList::new();
        item_types.insert("Olive", ItemType::new(ItemClass::Food, "Olive"));
        let items = &mut ItemList::new(Some(item_types.clone()));
        let food_item = Item::spawn_from_type("Olive", 1, &item_types);
        items.store(&food_item, 1114);
        subject.mount(&food_item, &item_class_specifiers, inventory, items);

        assert_eq!(subject.at(&MountingPoint::AtReady), Some(food_item.id));
    }
}
