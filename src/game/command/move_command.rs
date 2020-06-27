use super::*;

pub struct MoveCommand<'a> {
    character: &'a mut Player,
    facing: Direction,
    facing_changed: bool,
    x: i32,
    y: i32,
    obstacles: &'a mut BlockingMap,
}

impl<'a> MoveCommand<'a> {
    pub fn new(
        character: &'a mut Player,
        facing: Direction,
        x: i32,
        y: i32,
        obstacles: &'a mut BlockingMap,
    ) -> MoveCommand<'a> {
        MoveCommand {
            character,
            facing,
            facing_changed: false,
            x,
            y,
            obstacles,
        }
    }
}
impl<'a> CommandHandler<'a> for MoveCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        self.obstacles
            .unblock_at(self.character.x, self.character.y);
        self.obstacles.block_at(self.x, self.y);

        self.character.x = self.x;
        self.character.y = self.y;

        if self.character.facing != self.facing {
            self.facing_changed = true;
            self.character.facing = self.facing;
        }
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            CharacterMoved(self.character.id, self.character.x, self.character.y),
        );

        if self.facing_changed {
            GameUpdate::send(
                Some(update_tx),
                CharacterFacingChanged(self.character.id, self.facing),
            )
        };
        activity
    }
}

pub struct ChangeFacingCommand<'a> {
    pub player: &'a mut Player,
    pub facing: Direction,
}

impl<'a> ChangeFacingCommand<'a> {
    pub fn new(player: &'a mut Player, facing: Direction) -> ChangeFacingCommand<'a> {
        ChangeFacingCommand { player, facing }
    }
}

impl<'a> CommandHandler<'a> for ChangeFacingCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        self.player.facing = self.facing;
        None
    }
    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        GameUpdate::send(
            Some(update_tx),
            CharacterFacingChanged(self.player.id, self.facing),
        );
        activity
    }
}
