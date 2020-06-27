use super::*;

pub struct OpenDoorCommand<'a> {
    x: i32,
    y: i32,
    obstacles: &'a mut BlockingMap,
    map: &'a mut TileMap,
}

impl<'a> OpenDoorCommand<'a> {
    pub fn new(
        x: i32,
        y: i32,
        obstacles: &'a mut BlockingMap,
        map: &'a mut TileMap,
    ) -> OpenDoorCommand<'a> {
        OpenDoorCommand {
            x,
            y,
            obstacles,
            map,
        }
    }
}

impl<'a> CommandHandler<'a> for OpenDoorCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        self.map
            .set_tile_at(self.x, self.y, tile_map::Tile::OpenDoor);
        self.obstacles.unblock_at(self.x, self.y);
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        update_tx
            .send(TileChangedAt(self.x, self.y, tile_map::Tile::OpenDoor))
            .unwrap();
        activity
    }
}

pub struct CloseDoorCommand<'a> {
    x: i32,
    y: i32,
    obstacles: &'a mut BlockingMap,
    map: &'a mut TileMap,
}

impl<'a> CloseDoorCommand<'a> {
    pub fn new(
        x: i32,
        y: i32,
        obstacles: &'a mut BlockingMap,
        map: &'a mut TileMap,
    ) -> CloseDoorCommand<'a> {
        CloseDoorCommand {
            x,
            y,
            obstacles,
            map,
        }
    }
}

impl<'a> CommandHandler<'a> for CloseDoorCommand<'a> {
    fn perform_execute(
        &mut self,
        _update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<CommandSender>,
    ) -> Option<Box<dyn Activity>> {
        self.map
            .set_tile_at(self.x, self.y, tile_map::Tile::ClosedDoor);
        self.obstacles.block_at(self.x, self.y);
        None
    }

    fn announce(
        &self,
        activity: Option<Box<dyn Activity>>,
        update_tx: &std::sync::mpsc::Sender<GameUpdate>,
    ) -> Option<Box<dyn Activity>> {
        update_tx
            .send(TileChangedAt(self.x, self.y, tile_map::Tile::ClosedDoor))
            .unwrap();
        activity
    }
}
