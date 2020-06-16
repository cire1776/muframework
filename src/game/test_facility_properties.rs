use super::*;
use FacilityClass::*;

#[test]
fn get_property_returns_0_even_if_properties_not_set() {
    let mut inventories = InventoryList::new();
    let subject = Facility::new(
        1,
        10,
        10,
        OakTree,
        "Has a yellow ribbon".into(),
        &mut inventories,
    );
    assert_eq!(subject.get_property("wood"), 0);
}

#[test]
#[should_panic(expected = "properties not available for this facility")]
fn set_property_panics_if_called_before_properties_set() {
    let mut inventories = InventoryList::new();
    let mut subject = Facility::new(
        1,
        10,
        10,
        OakTree,
        "Has a yellow ribbon".into(),
        &mut inventories,
    );
    subject.set_property("wood", 1776);
}
#[test]
fn get_property_returns_0_if_properties_set_but_property_uninitialized() {
    let mut inventories = InventoryList::new();
    let mut subject = Facility::new(
        1,
        10,
        10,
        OakTree,
        "Has a yellow ribbon".into(),
        &mut inventories,
    );
    subject.enable_properties();

    assert_eq!(subject.get_property("wood"), 0);
}

#[test]
fn get_property_and_set_property_returns_previously_set_value_if_properties_set() {
    let mut inventories = InventoryList::new();
    let mut subject = Facility::new(
        1,
        10,
        10,
        OakTree,
        "Has a yellow ribbon".into(),
        &mut inventories,
    );
    subject.enable_properties();

    subject.set_property("wood", 1776);

    assert_eq!(subject.get_property("wood"), 1776);
}
