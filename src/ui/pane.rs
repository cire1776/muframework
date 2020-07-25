use super::*;
use window::ScreenObject;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PaneTitle {
    None,
    PickingApples,
    PickingOlives,
    Logging,
    Pressing,
    Filling,
    Digging,
    Mining,
    Sawing,
    NetFishing,
    Fishing,
    PlacingTrap,
    CollectingTrap,
    Smelting,
    Cooking,
    Reading,
    Harvesting,
    Building,
    ChestContents,
    SelectOutput,
}

impl ToString for PaneTitle {
    fn to_string(&self) -> String {
        let str = match self {
            PaneTitle::None => "",
            PaneTitle::PickingApples => "Picking Apples",
            PaneTitle::PickingOlives => "Picking Olives",
            PaneTitle::Logging => "Logging",
            PaneTitle::Pressing => "Pressing",
            PaneTitle::Filling => "Filling",
            PaneTitle::Digging => "Digging",
            PaneTitle::Mining => "Mining",
            PaneTitle::Sawing => "Sawing",
            PaneTitle::NetFishing => "Net Fishing",
            PaneTitle::Fishing => "Fishing",
            PaneTitle::PlacingTrap => "Placing Trap",
            PaneTitle::CollectingTrap => "Collecting Trap",
            PaneTitle::Smelting => "Smelting",
            PaneTitle::Cooking => "Cooking",
            PaneTitle::Reading => "Reading",
            PaneTitle::Harvesting => "Harvesting",
            PaneTitle::Building => "Building",
            PaneTitle::ChestContents => "Chest Contents",
            PaneTitle::SelectOutput => "Select Output",
        };

        str.into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pane {
    title: PaneTitle,

    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    pub selection: Option<u8>,
    max_selection: u8,
}

impl<'a> Pane {
    pub fn new(
        title: PaneTitle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        max_selection: u8,
    ) -> Self {
        Self {
            title,
            x,
            y,
            width,
            height,
            selection: None,
            max_selection,
        }
    }

    pub fn draw_frame<S: ToString>(
        &self,
        message: S,
        window: &dyn BasicWindow,
        context: &mut BTerm,
    ) {
        let (scroll_x, scroll_y) = window.scroll_x_y();
        window.draw_box(
            self.x + scroll_x,
            self.y + scroll_y,
            self.width,
            self.height,
            context,
        );
        window.draw_text(&self.title.to_string()[..], 1 + self.x, self.y, context);
        window.draw_text(
            &message.to_string()[..],
            1 + self.x,
            self.height + self.y,
            context,
        )
    }

    pub fn draw_text<S: ToString>(
        &self,
        text: S,
        x: i32,
        y: i32,
        window: &dyn BasicWindow,
        context: &mut BTerm,
    ) {
        let text = &text.to_string()[..];
        window.draw_text(text, self.x + x, self.y + y, context);
    }

    pub fn draw_inverted_text<S: ToString>(
        &self,
        text: S,
        x: i32,
        y: i32,
        window: &dyn BasicWindow,
        context: &mut BTerm,
    ) {
        let text = &text.to_string()[..];
        window.draw_inverted_text(text, self.x + x, self.y + y, context);
    }

    pub fn set_max_selection(&mut self, max: u8) {
        self.max_selection = max;

        if let Some(selection) = self.selection {
            if selection > max {
                let new_selection = if selection == 1 { None } else { Some(max) };

                self.selection = new_selection;
            }
        }
    }
}

impl ScreenObject for Pane {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn width(&self) -> i32 {
        self.width
    }
    fn height(&self) -> i32 {
        self.height
    }

    fn is_within_frame(&self, point: Point) -> bool {
        let (local_x, local_y) = (point.x - self.x(), point.y - self.y());
        local_x > 0 && local_y > 0 && local_x < self.width() && local_y < self.height()
    }
}

impl MouseReceiver for Pane {
    fn mouse_point(&self, _context: &mut BTerm) -> Point {
        todo!()
    }
    fn handle_left_click(&mut self, _x: i32, y: i32, _context: &mut BTerm) {
        let new_selection = (y - self.y) as u8;
        if new_selection > 0 && new_selection <= self.max_selection {
            self.selection = Some(new_selection);
        } else {
            self.selection = None;
        }
    }
}
