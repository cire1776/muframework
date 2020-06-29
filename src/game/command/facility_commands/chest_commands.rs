use super::*;

#[allow(dead_code)]
// TODO: change facility to reflect open chest status.
pub struct OpenChestCommand<'a> {
    x: i32,
    y: i32,
    player: &'a mut Player,
    external_inventory: &'a Inventory,
    facility_id: u64,
    facilities: &'a FacilityList,
}

impl<'a> OpenChestCommand<'a> {
    pub fn new(
        x: i32,
        y: i32,
        player: &'a mut Player,
        facility_id: u64,
        facilities: &'a FacilityList,
        inventories: &'a InventoryList,
    ) -> Self {
        let external_inventory = inventories.get(&facility_id).unwrap();
        Self {
            x,
            y,
            player,
            external_inventory,
            facility_id,
            facilities,
        }
    }
}

impl<'a> CommandHandler<'a> for OpenChestCommand<'a> {
    fn perform_execute(&mut self) -> Option<Box<dyn Activity>> {
        self.player.external_inventory = Some(self.external_inventory.to_vec());
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::ExternalInventoryOpened(
                self.external_inventory.to_vec(),
                self.external_inventory.id(),
            ),
        );
        activity
    }
}
