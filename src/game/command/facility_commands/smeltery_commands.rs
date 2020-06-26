use super::*;

pub enum SmeltingSkill {
    Tin,
    Copper,
    Bronze,
    Lead,
    Mercury,
    Iron,
    Tungsten,
    Cobalt,
    Nickel,
    Steel,
    Gold,
    Aluminum,
    Silver,
    Zinc,
    Platinum,
    StainlessSteel,
    Stellite,
    Titanium,
    Mythral,
}

impl SmeltingSkill {
    pub fn products() -> Vec<(&'static str, u8)> {
        vec![
            ("Tin", 1),              // simple
            ("Copper", 2),           // simple
            ("Bronze", 4),           // 50/50 compound
            ("Lead", 6),             // simple
            ("Mercury", 9),          // simple
            ("Iron", 12),            // simple
            ("Tungsten", 12),        // simple
            ("Cobalt", 15),          // simple
            ("Nickel", 18),          // simple
            ("Steel", 21),           // 3 steel 1 coal + fuel
            ("Gold", 24),            // mercury compound
            ("Aluminum", 27),        // simple
            ("Silver", 30),          // mercury compound
            ("Zinc", 33),            // simple
            ("Platinum", 36),        // mercury compound
            ("Stainless Steel", 39), // zinc compound
            ("Stellite", 40),        // 50/50 compound
            ("Titanium", 43),        // simple
            ("Mythral", 45),         // simple
        ]
    }
    pub fn products_for_player_level(player: &Player) -> Vec<&'static str> {
        let level = std::cmp::max(
            1,
            player.get_attribute(Attribute::SkillLevel("smelting".into()), 0),
        ) as u8;

        let products = Self::products()
            .iter()
            .take_while(|p| p.1 <= level)
            .map(|p| p.0)
            .collect();

        products
    }

    pub fn can_produce(product: String, inventory: &Inventory) -> bool {
        match &product[..] {
            "Tin" | "Copper" | "Lead" | "Mercury" | "Iron" | "Tungsten" | "Cobalt" | "Nickel" => {
                if !inventory.has_sufficient(ItemClass::Ore, format!("{} Ore", product), 4) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Coal", 1)
                {
                    return false;
                }
                true
            }
            "Bronze" => {
                if !inventory.has_sufficient(ItemClass::Ore, "Tin Ore", 2) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Copper Ore", 2) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Coal", 1)
                {
                    return false;
                }
                true
            }
            "Gold" | "Silver" | "Platinum" => {
                if !inventory.has_sufficient(ItemClass::Ore, format!("{} Ore", product), 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Mercury", 1) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
            "Aluminum" | "Zinc" | "Titanium" | "Mythral" => {
                if !inventory.has_sufficient(ItemClass::Ore, format!("{} Ore", product), 4) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
            "Steel" => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 2) {
                    return false;
                }
                true
            }
            "Stainless Steel" => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Zinc Bar", 1) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
            "Stellite" => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Tungsten Bar", 1) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
            _ => false,
        }
    }

    pub fn consume_from_inventory_for(
        product: String,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) {
        match &product[..] {
            "Tin" | "Copper" | "Lead" | "Mercury" | "Iron" | "Tungsten" | "Cobalt" | "Nickel" => {
                inventory.consume(ItemClass::Ore, format!("{} Ore", product), 4, items);

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Softwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Hardwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    inventory.consume(ItemClass::Ore, "Coal", 1, items);
                } else {
                    panic!("didn't have fuel")
                }
            }
            "Bronze" => {
                inventory.consume(ItemClass::Ore, "Tin Ore", 2, items);
                inventory.consume(ItemClass::Ore, "Copper Ore", 2, items);

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Softwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Hardwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    inventory.consume(ItemClass::Ore, "Coal", 1, items);
                } else {
                    panic!("didn't have fuel")
                }
            }
            "Gold" | "Silver" | "Platinum" => {
                inventory.consume(ItemClass::Ore, format!("{} Ore", product), 3, items);
                inventory.consume(ItemClass::Ore, "Mercury", 1, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            "Aluminum" | "Zinc" | "Titanium" | "Mythral" => {
                inventory.consume(ItemClass::Ore, format!("{} Ore", product), 4, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            "Steel" => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 3, items);
                inventory.consume(ItemClass::Material, "Coal", 2, items)
            }
            "Stainless Steel" => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 3, items);
                inventory.consume(ItemClass::Ore, "Zince", 1, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            "Stellite" => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 2, items);
                inventory.consume(ItemClass::Ore, "Tungsten Ore", 2, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            _ => {}
        }
    }
}

#[allow(dead_code)]
// TODO: change facility to reflect open chest status.
pub struct OpenSmelteryCommand<'a> {
    player: &'a mut Player,
    facility_id: u64,
    facilities: &'a FacilityList,
}

impl<'a> OpenSmelteryCommand<'a> {
    pub fn new(player: &'a mut Player, facility_id: u64, facilities: &'a FacilityList) -> Self {
        Self {
            player,
            facility_id,
            facilities,
        }
    }

    pub fn can_perform(_player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
    }
}

impl<'a> CommandHandler<'a> for OpenSmelteryCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<&std::sync::mpsc::Sender<Command>>,
    ) -> Option<Box<dyn Activity>> {
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &GameUpdateSender,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            GameUpdate::DisplayOptions(
                SmeltingSkill::products_for_player_level(self.player),
                ActionContinuation::Smeltery,
                self.facility_id,
            ),
        );
        activity
    }
}

pub struct ActivateSmelteryCommand<'a> {
    product: String,
    player: &'a mut Player,
    inventories: &'a mut InventoryList,
    facility_id: u64,
}

impl<'a> ActivateSmelteryCommand<'a> {
    pub fn new(
        player: &'a mut Player,
        facility_id: u64,
        product_index: u8,
        inventories: &'a mut InventoryList,
    ) -> Self {
        let product = SmeltingSkill::products()[(product_index - 1) as usize]
            .0
            .to_string();

        Self {
            product,
            player,
            facility_id,
            inventories,
        }
    }

    pub fn can_perform(_player: &Player, facility: &Facility) -> bool {
        !facility.is_in_use()
    }
}

impl<'a> CommandHandler<'a> for ActivateSmelteryCommand<'a> {
    fn expiration(&self) -> u32 {
        (60 + self
            .player
            .get_attribute(Attribute::SkillTime("smelting".into()), 0)) as u32
    }

    fn create_activity(
        &self,
        timer: timer::Timer,
        guard: Guard,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Option<Box<dyn Activity>> {
        let activity = SmeltingActivity::new(
            self.product.clone(),
            self.expiration(),
            self.player.id,
            self.facility_id,
            timer,
            Some(guard),
            update_sender,
            command_sender,
        );

        let inventory = self
            .inventories
            .get(&self.player.id)
            .expect("unable to get inventory.");

        if SmeltingSkill::can_produce(self.product.clone(), inventory) {
            Some(Box::new(activity))
        } else {
            None
        }
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        if let Some(activity) = &activity {
            activity.start(update_tx);
        }
        activity
    }
}

pub struct SmeltingActivity {
    product: String,
    expiration: u32,
    _player_inventory_id: u64,
    facility_id: u64,
    _timer: timer::Timer,
    guard: Option<Guard>,
    _update_sender: GameUpdateSender,
    _command_sender: CommandSender,
}

impl SmeltingActivity {
    fn new(
        product: String,
        expiration: u32,
        player_inventory_id: u64,
        facility_id: u64,
        timer: timer::Timer,
        guard: Option<Guard>,
        update_sender: GameUpdateSender,
        command_sender: CommandSender,
    ) -> Self {
        Self {
            product,
            expiration,
            _player_inventory_id: player_inventory_id,
            facility_id,
            _timer: timer,
            guard,
            _update_sender: update_sender,
            _command_sender: command_sender,
        }
    }
}

impl Activity for SmeltingActivity {
    fn activity_title(&self) -> ui::pane::PaneTitle {
        ui::pane::PaneTitle::Smelting
    }

    fn facility_id(&self) -> u64 {
        self.facility_id
    }

    fn expiration(&self) -> u32 {
        self.expiration
    }

    fn on_completion(
        &self,
        player_inventory_id: u64,
        _facility: &mut Facility,
        items: &mut ItemList,
        inventories: &mut InventoryList,
        _update_sender: &GameUpdateSender,
        command_sender: &CommandSender,
    ) -> RefreshInventoryFlag {
        let inventory = inventories
            .get_mut(&player_inventory_id)
            .expect("unable to find inventory");

        inventory.consume(ItemClass::Ore, format!("{} Ore", self.product), 4, items);

        let wood_type = if inventory.count_of(ItemClass::Material, "Softwood Log") >= 1 {
            "Softwood Log"
        } else if inventory.count_of(ItemClass::Material, "Hardwood Log") >= 1 {
            "Hardwood Log"
        } else {
            panic!("wood not found");
        };

        inventory.consume(ItemClass::Material, wood_type, 1, items);

        Command::send(
            Some(&command_sender),
            Command::SpawnItem(
                player_inventory_id,
                ItemClass::Material,
                format!("{} Bar", self.product),
            ),
        );

        if !SmeltingSkill::can_produce(self.product.clone(), inventory) {
            Command::send(Some(&command_sender), Command::ActivityAbort);
        }
        RefreshInventoryFlag::RefreshInventory
    }

    fn clear_guard(&mut self) {
        self.guard = None
    }
}
