use super::*;
use ItemClass::*;

#[test]
fn items_have_a_type() {
    let subject = Item::spawn(BladeWeapon, "A large knife");

    let _item_type = subject.item_type;
}
#[test]
fn items_class_is_provided_by_its_type() {
    let mut subject = Item::spawn(BladeWeapon, "large knife");

    let new_type = ItemType::new(Headwear, "hat");
    subject.item_type = new_type;

    assert_eq!(subject.class(), Headwear);
}

#[test]
fn items_raw_description_is_provided_by_its_type() {
    let mut subject = Item::spawn(BladeWeapon, "large knife");

    let new_type = ItemType::new(Headwear, "bold cap");
    subject.item_type = new_type;

    assert_eq!(subject.raw_description(), "bold cap");
}
