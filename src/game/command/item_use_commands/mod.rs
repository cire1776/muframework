use super::*;

pub struct ActivateBookReadingCommand<'a> {
    book: Item,
    player: &'a mut Player,
    timer: &'a mut Timer,
}

impl<'a> ActivateBookReadingCommand<'a> {
    pub fn new(player: &'a mut Player, items: &'a ItemList, timer: &'a mut Timer) -> Self {
        let book_id = player
            .mounting_points
            .at(&MountingPoint::OnHand)
            .expect("unable to find mounted book.");
        let book = items.get_as_item(book_id).expect("unable to get book.");

        Self {
            book,
            player,
            timer,
        }
    }
}

impl<'a> CommandHandler<'a> for ActivateBookReadingCommand<'a> {
    fn timer(&mut self) -> Option<&mut Timer> {
        return Some(self.timer);
    }

    fn expiration(&self) -> u32 {
        IntellectualSkill::expiration(&self.book, self.player)
    }

    fn create_activity(&self, guard: Guard) -> Option<Box<dyn Activity>> {
        let level = self.player.get_level_for(Intellectual);

        let activity =
            BookReadingActivity::new(&self.book, self.expiration(), level as u8, Some(guard));

        if IntellectualSkill::can_produce(&self.book, self.player) {
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

pub struct BookReadingActivity {
    book: Item,
    expiration: u32,
    _level: u8,
    guard: Option<Guard>,
}

impl BookReadingActivity {
    pub fn new(book: &Item, expiration: u32, level: u8, guard: Option<Guard>) -> Self {
        Self {
            book: book.clone(),
            expiration,
            _level: level,
            guard,
        }
    }
}

impl Activity for BookReadingActivity {
    fn expiration(&self) -> u32 {
        self.expiration
    }
    fn activity_title(&self) -> ui::PaneTitle {
        ui::PaneTitle::Reading
    }
    fn facility_id(&self) -> u64 {
        0
    }
    fn clear_guard(&mut self) {
        self.guard = None
    }
    fn on_completion(
        &self,
        player: &mut Player,
        _facility: &mut Facility,
        _items: &mut ItemList,
        _inventories: &mut InventoryList,
        _rng: &mut Rng,
        _update_sender: &GameUpdateSender,
        _command_sender: CommandSender,
    ) -> RefreshInventoryFlag {
        IntellectualSkill::produce_results_for(&self.book, player);

        RefreshInventoryFlag::DontRefreshInventory
    }
}
