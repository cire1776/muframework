use super::*;
use crate::test_support::*;
use ItemClass::*;

#[test]
fn items_have_a_type() {
    let subject = test_item(BladeWeapon, "A large knife", 1);

    let _item_type = subject.item_type;
}
#[test]
fn items_class_is_provided_by_its_type() {
    let mut subject = test_item(BladeWeapon, "large knife", 1);

    let new_type = ItemType::new(Headwear, "hat");
    subject.item_type = new_type;

    assert_eq!(subject.class(), Headwear);
}

#[test]
fn items_raw_description_is_provided_by_its_type() {
    let mut subject = test_item(BladeWeapon, "large knife", 1);

    let new_type = ItemType::new(Headwear, "bold cap");
    subject.item_type = new_type;

    assert_eq!(subject.raw_description(), "bold cap");
}
