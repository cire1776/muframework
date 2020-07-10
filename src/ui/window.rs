use super::*;
pub trait ScreenObject {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn rect(&self) -> Rect {
        Rect {
            x1: self.x(),
            y1: self.y(),
            x2: self.x() + self.width(),
            y2: self.y() + self.height(),
        }
    }

    fn is_within_frame(&self, point: Point) -> bool {
        point.x > 0
            && point.y > 0
            && point.x < (self.x() + self.width())
            && point.y < (self.y() + self.height())
    }
}

pub trait BasicWindow: ScreenObject {
    fn active_pane(&self) -> Option<Pane> {
        None
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        let (mut local_x, mut local_y) = self.local_x_y(x, y);
        local_x -= self.x();
        local_y -= self.y();

        local_x >= 0 && local_x < self.width() - 1 && local_y >= 0 && local_y < self.height() - 1
    }

    fn local_x_y(&self, x: i32, y: i32) -> (i32, i32) {
        (self.local_x(x), self.local_y(y))
    }

    fn local_x(&self, x: i32) -> i32 {
        x - self.scroll_x() + self.x()
    }

    fn local_y(&self, y: i32) -> i32 {
        y - self.scroll_y() + self.y()
    }

    fn is_in_central_region(&self, x: i32, y: i32) -> bool {
        !(self.is_above_central_region(y)
            || self.is_below_central_region(y)
            || self.is_left_of_central_region(x)
            || self.is_right_of_central_region(x))
    }
    fn is_left_of_central_region(&self, x: i32) -> bool {
        self.local_x(x) < (self.width() as f64 * 0.2) as i32
    }

    fn is_right_of_central_region(&self, x: i32) -> bool {
        self.local_x(x) > (self.width() as f64 * 0.8) as i32
    }

    fn is_above_central_region(&self, y: i32) -> bool {
        self.local_y(y) < (self.height() as f64 * 0.2) as i32
    }

    fn is_below_central_region(&self, y: i32) -> bool {
        self.local_y(y) > (self.height() as f64 * 0.8) as i32
    }

    fn scroll_x(&self) -> i32;
    fn scroll_y(&self) -> i32;
    fn scroll_x_y(&self) -> (i32, i32) {
        (self.scroll_x(), self.scroll_y())
    }
    fn set_scroll(&mut self, x: i32, y: i32);

    fn max_scroll(&self) -> (i32, i32);
    fn set_max_scroll(&mut self, width: i32, height: i32);
    fn move_focus_towards(&mut self, x: i32, y: i32) {
        let dx = if self.is_right_of_central_region(x) {
            1
        } else if self.is_left_of_central_region(x) {
            -1
        } else {
            0
        };

        let dy = if self.is_above_central_region(y) {
            -1
        } else if self.is_below_central_region(y) {
            1
        } else {
            0
        };

        self.scroll_by(dx, dy);
    }
    fn scroll_to(&mut self, x: i32, y: i32) {
        self.scroll_by(x - self.scroll_x(), y - self.scroll_y());
    }
    fn scroll_by(&mut self, dx: i32, dy: i32) {
        let (max_scroll_x, max_scroll_y) = self.max_scroll();
        let (mut dx, mut dy) = (dx, dy);

        if self.scroll_x() + dx < 0 || self.scroll_x() + dx > max_scroll_x {
            dx = 0;
        }
        if self.scroll_y() + dy < 0 || self.scroll_y() + dy > max_scroll_y {
            dy = 0;
        }

        self.set_scroll(self.scroll_x() + dx, self.scroll_y() + dy);
    }
    fn set(&self, context: &mut BTerm, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        if !self.is_in_bounds(x, y) {
            return;
        }
        let (local_x, local_y) = self.local_x_y(x, y);

        context.set(local_x + 1, local_y + 1, fg, bg, glyph);
    }

    fn internal_draw_frame(&self, context: &mut BTerm, message: &str) {
        context.draw_box_double(
            self.x(),
            self.y(),
            self.width(),
            self.height(),
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );

        self.draw_static_text(
            message,
            (self.width() as f64 * 0.80) as i32,
            self.height(),
            context,
        )
    }

    fn draw_frame(&self, context: &mut BTerm, message: &str) {
        self.internal_draw_frame(context, message);
    }

    fn draw_box(&self, x: i32, y: i32, width: i32, height: i32, context: &mut BTerm) {
        let (local_x, local_y) = self.local_x_y(x, y);
        context.draw_box(
            local_x,
            local_y,
            width,
            height,
            RGB::named(WHITE),
            RGBA::from_u8(0, 0, 0, 128),
        );
    }
    fn draw_text(&self, string: &str, x: i32, y: i32, context: &mut BTerm) {
        let (local_x, local_y) = self.local_x_y(x, y);
        context.print(local_x, local_y, string);
    }

    fn draw_inverted_text(&self, string: &str, x: i32, y: i32, context: &mut BTerm) {
        let (local_x, local_y) = self.local_x_y(x, y);

        context.print_color(
            local_x,
            local_y,
            RGB::named(BLACK),
            RGB::named(WHITE),
            string,
        );
    }

    fn draw_static_text(&self, string: &str, x: i32, y: i32, context: &mut BTerm) {
        let local_x = x + self.x();
        let local_y = y + self.y();
        context.print(local_x, local_y, string);
    }
}

pub trait MouseReceiver: ScreenObject {
    fn mouse_point(&self, context: &mut BTerm) -> Point;

    fn handle_left_click(&mut self, x: i32, y: i32, context: &mut BTerm);
}

impl MouseReceiver for dyn BasicWindow {
    fn mouse_point(&self, context: &mut BTerm) -> Point {
        let result = context.mouse_point();

        result - Point::constant(self.x(), self.y())
    }
    fn handle_left_click(&mut self, _x: i32, _y: i32, _context: &mut BTerm) {}
}
#[derive(Debug)]
pub struct Window {
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    scroll_x: i32,
    scroll_y: i32,

    max_scroll_x: i32,
    max_scroll_y: i32,
}

impl Window {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
        Window {
            x,
            y,
            width,
            height,

            scroll_x: 0,
            scroll_y: 0,

            max_scroll_x: 0,
            max_scroll_y: 0,
        }
    }
}

impl ScreenObject for Window {
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
}

impl BasicWindow for Window {
    fn scroll_x(&self) -> i32 {
        self.scroll_x
    }

    fn scroll_y(&self) -> i32 {
        self.scroll_y
    }

    fn set_scroll(&mut self, x: i32, y: i32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    fn set_max_scroll(&mut self, width: i32, height: i32) {
        use std::cmp::max;
        self.max_scroll_x = max(width - self.width + 1, 0);
        self.max_scroll_y = max(height - self.height + 1, 0);
    }

    fn max_scroll(&self) -> (i32, i32) {
        (self.max_scroll_x, self.max_scroll_y)
    }
}

impl MouseReceiver for Window {
    fn mouse_point(&self, context: &mut BTerm) -> Point {
        let result = context.mouse_point();

        result - Point::constant(self.x(), self.y())
    }

    fn handle_left_click(&mut self, _x: i32, _y: i32, _context: &mut BTerm) {
        println!("mouse clicked in general window");
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MapWindowMode {
    Normal,
    ExternalInventory,
    ToolTip,
    DisplayOptions,
}
pub struct MapWindow {
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    scroll_x: i32,
    scroll_y: i32,

    max_scroll_x: i32,
    max_scroll_y: i32,

    pub window_mode: MapWindowMode,

    pub selection: Option<u8>,

    pub active_pane: Option<Pane>,
}

impl MapWindow {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> MapWindow {
        MapWindow {
            x,
            y,
            width,
            height,

            scroll_x: 0,
            scroll_y: 0,

            max_scroll_x: 0,
            max_scroll_y: 0,

            window_mode: MapWindowMode::Normal,
            selection: None,

            active_pane: None,
        }
    }
}

impl ScreenObject for MapWindow {
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
}

impl BasicWindow for MapWindow {
    fn active_pane(&self) -> Option<Pane> {
        self.active_pane
    }

    fn scroll_x(&self) -> i32 {
        self.scroll_x
    }

    fn scroll_y(&self) -> i32 {
        self.scroll_y
    }

    fn set_scroll(&mut self, x: i32, y: i32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    fn set_max_scroll(&mut self, width: i32, height: i32) {
        use std::cmp::max;
        self.max_scroll_x = max(width - self.width + 1, 0);
        self.max_scroll_y = max(height - self.height + 1, 0);
    }

    fn max_scroll(&self) -> (i32, i32) {
        (self.max_scroll_x, self.max_scroll_y)
    }

    fn draw_text(&self, string: &str, x: i32, y: i32, context: &mut BTerm) {
        let local_x = x + self.x();
        let local_y = y + self.y();

        let mut fg = RGB::named(rltk::WHITE);
        let mut bg = RGB::named(rltk::BLACK);

        if self.selection == Some((y - 1) as u8) {
            let temp = fg;
            fg = bg;
            bg = temp;
        }

        context.print_color(local_x, local_y, fg, bg, string);
    }
}

impl MouseReceiver for MapWindow {
    fn mouse_point(&self, context: &mut BTerm) -> Point {
        let result = context.mouse_point();

        result - Point::constant(self.x(), self.y())
    }

    fn handle_left_click(&mut self, x: i32, y: i32, context: &mut BTerm) {
        if self.active_pane() == None {
            println!("mouse clicked in map window");
        } else {
            let mut pane = self.active_pane().unwrap();
            if pane.is_within_frame(Point::constant(x, y)) {
                pane.handle_left_click(x, y, context);
                self.active_pane = Some(pane);
            } else {
                println!("clicked outside of active pane")
            }
        }
    }
}
#[derive(Debug)]
pub struct InventoryWindow {
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    scroll_x: i32,
    scroll_y: i32,

    max_scroll_x: i32,
    max_scroll_y: i32,

    pub window_mode: InventoryWindowMode,

    pub inventory_items: Vec<String>,
    pub equipment: Vec<String>,

    pub selected_item: Option<u8>,
    pub selected_equipment: Option<u8>,
    pub max_selection_items: u8,
    pub max_selection_equipment: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InventoryWindowMode {
    Inventory,
    Equipment,
}

impl InventoryWindow {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> InventoryWindow {
        InventoryWindow {
            x,
            y,
            width,
            height,

            scroll_x: 0,
            scroll_y: 0,

            max_scroll_x: 0,
            max_scroll_y: 0,

            window_mode: InventoryWindowMode::Inventory,

            inventory_items: vec![],
            equipment: vec![],

            selected_item: None,
            selected_equipment: None,
            max_selection_items: 0,
            max_selection_equipment: 0,
        }
    }

    pub fn set_inventory_items(&mut self, items: Vec<String>, equipment: Vec<String>) {
        self.inventory_items = items;
        self.equipment = equipment;
    }
}

impl ScreenObject for InventoryWindow {
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
}
impl BasicWindow for InventoryWindow {
    fn scroll_x(&self) -> i32 {
        self.scroll_x
    }

    fn scroll_y(&self) -> i32 {
        self.scroll_y
    }

    fn set_scroll(&mut self, x: i32, y: i32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    fn set_max_scroll(&mut self, width: i32, height: i32) {
        use std::cmp::max;
        self.max_scroll_x = max(width - self.width + 1, 0);
        self.max_scroll_y = max(height - self.height + 1, 0);
    }

    fn max_scroll(&self) -> (i32, i32) {
        (self.max_scroll_x, self.max_scroll_y)
    }

    fn draw_text(&self, string: &str, x: i32, y: i32, context: &mut BTerm) {
        let local_x = x + self.x();
        let local_y = y + self.y();

        let mut fg = RGB::named(rltk::WHITE);
        let mut bg = RGB::named(rltk::BLACK);

        let selection = match self.window_mode {
            InventoryWindowMode::Inventory => self.selected_item,
            InventoryWindowMode::Equipment => self.selected_equipment,
        };
        if selection == Some((y - 2) as u8) {
            let temp = fg;
            fg = bg;
            bg = temp;
        }

        context.print_color(local_x, local_y, fg, bg, string);
    }

    fn draw_frame(&self, context: &mut BTerm, message: &str) {
        self.internal_draw_frame(context, message);

        let fg;
        let bg;

        if self.window_mode == InventoryWindowMode::Equipment {
            fg = RGB::named(rltk::WHITE);
            bg = RGB::named(rltk::BLACK);
        } else {
            fg = RGB::named(rltk::BLACK);
            bg = RGB::named(rltk::WHITE);
        }

        context.print_color(self.local_x(1), self.local_y(1), fg, bg, "Inven.");
        context.print_color(self.local_x(8), self.local_y(1), bg, fg, "Equip.");

        if self.window_mode == InventoryWindowMode::Inventory {
            self.draw_inventory_pane(context);
        } else {
            self.draw_equipment_pane(context);
        }
    }
}

impl InventoryWindow {
    fn draw_inventory_pane(&self, context: &mut BTerm) {
        for (line, item) in self.inventory_items.iter().enumerate() {
            self.draw_text(&item, 1, (line + 3) as i32, context)
        }
    }

    fn draw_equipment_pane(&self, context: &mut BTerm) {
        for (line, item) in self.equipment.iter().enumerate() {
            self.draw_text(&item, 1, (line + 3) as i32, context)
        }
    }

    pub fn get_selected_item_id(&self, inventory: &Vec<Item>) -> Option<u64> {
        if self.window_mode == InventoryWindowMode::Inventory {
            if self.selected_item == None {
                return None;
            }
            let item = &inventory[(self.selected_item.unwrap() - 1) as usize];
            Some(item.id)
        } else {
            if self.selected_equipment == None {
                return None;
            }
            let item = &inventory[(self.selected_equipment.unwrap() - 1) as usize];
            Some(item.id)
        }
    }

    pub fn set_max_item_selection(&mut self, max: u8) {
        self.max_selection_items = max;

        if let Some(selection) = self.selected_item {
            if selection > max {
                let new_selection = if selection == 1 { None } else { Some(max) };
                self.selected_item = new_selection;
            }
        }
    }

    pub fn set_max_equipment_selection(&mut self, max: u8) {
        self.max_selection_equipment = max;

        if let Some(selection) = self.selected_equipment {
            if selection > max {
                let new_selection = if selection == 1 { None } else { Some(max) };
                self.selected_equipment = new_selection;
            }
        }
    }
}

impl MouseReceiver for InventoryWindow {
    fn mouse_point(&self, context: &mut BTerm) -> Point {
        let result = context.mouse_point();

        result - Point::constant(self.x(), self.y())
    }
    fn handle_left_click(&mut self, x: i32, y: i32, _context: &mut BTerm) {
        if y == 1 {
            self.window_mode = if x > 6 {
                InventoryWindowMode::Equipment
            } else {
                InventoryWindowMode::Inventory
            };

            // turn off selection so that we don't have to worry about
            //  dropping or manipulating an item selected on the other pane.
            self.selected_equipment = None;
            self.selected_item = None;

            return;
        }

        let y_offset = Some((y - 2) as u8);

        let mut selection;
        let max_selection;
        if self.window_mode == InventoryWindowMode::Inventory {
            selection = self.selected_item;
            max_selection = self.max_selection_items;
        } else {
            selection = self.selected_equipment;
            max_selection = self.max_selection_equipment;
        }

        if (y - 2) as u8 <= max_selection {
            if selection != y_offset {
                selection = y_offset;
            } else {
                selection = None;
            }
        } else {
            selection = None;
        }

        if self.window_mode == InventoryWindowMode::Inventory {
            self.selected_item = selection;
        } else {
            self.selected_equipment = selection;
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillWindow {
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    scroll_x: i32,
    scroll_y: i32,

    max_scroll_x: i32,
    max_scroll_y: i32,

    pub skills: HashMap<String, (u8, u64)>,
}

impl SkillWindow {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            skills: HashMap::new(),

            x,
            y,
            width,
            height,

            scroll_x: 0,
            scroll_y: 0,

            max_scroll_x: 0,
            max_scroll_y: 0,
        }
    }
}

impl ScreenObject for SkillWindow {
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
}
impl BasicWindow for SkillWindow {
    fn scroll_x(&self) -> i32 {
        self.scroll_x
    }

    fn scroll_y(&self) -> i32 {
        self.scroll_y
    }

    fn set_scroll(&mut self, x: i32, y: i32) {
        self.scroll_x = x;
        self.scroll_y = y;
    }

    fn set_max_scroll(&mut self, width: i32, height: i32) {
        use std::cmp::max;
        self.max_scroll_x = max(width - self.width + 1, 0);
        self.max_scroll_y = max(height - self.height + 1, 0);
    }

    fn max_scroll(&self) -> (i32, i32) {
        (self.max_scroll_x, self.max_scroll_y)
    }

    fn draw_text(&self, string: &str, x: i32, y: i32, context: &mut BTerm) {
        let local_x = x + self.x();
        let local_y = y + self.y();

        let fg = RGB::named(rltk::WHITE);
        let bg = RGB::named(rltk::BLACK);

        context.print_color(local_x, local_y, fg, bg, string);
    }

    fn draw_frame(&self, context: &mut BTerm, message: &str) {
        use num_format::{Locale, ToFormattedString};
        self.internal_draw_frame(context, message);
        let mut y = 1;
        let mut skills: Vec<&String> = self.skills.keys().collect();
        skills.sort();

        for skill in &skills {
            let (level, xp) = self.skills[*skill];
            self.draw_static_text(&format!("{:0>2} {}", level, skill), 1, y, context);

            let xp_string = &*xp.to_formatted_string(&Locale::en);
            self.draw_static_text(xp_string, (21 - xp_string.len()) as i32, y + 1, context);
            y += 2;
        }
    }
}
