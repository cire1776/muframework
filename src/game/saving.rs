use super::*;

pub struct GameSaver {}

impl GameSaver {
    pub fn save_game_to_string(
        player: &Player,
        characters: &CharacterList,
        items: &ItemList,
        inventories: &InventoryList,
        facilities: &FacilityList,
        game_state: &GameState,
    ) -> String {
        use ron::ser::*;

        let mut save_text = format!(
            "tick => {}\nNEXT_ITEM_ID => {:?}\nNEXT_ID => {:?}\n===END OF SERVERDATA===\n",
            game_state.ticks, GLOBAL_NEXT_ITEM_ID, GLOBAL_NEXT_ID,
        );

        save_text += &match ron::ser::to_string_pretty(&player, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };
        save_text += "\n===END OF PLAYERS===\n";

        save_text += &match ron::ser::to_string_pretty(&characters, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };

        save_text += "\n===END OF CHARACTERS===\n===END OF ITEM TYPES===\n";

        save_text += &match ron::ser::to_string_pretty(&inventories, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };

        save_text += "\n===END OF INVENTORIES===\n";

        let bundled_items = items.bundled_items();

        save_text += &match ron::ser::to_string_pretty(&bundled_items, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };

        save_text += "\n===END OF ITEMS===\n";

        save_text += &match ron::ser::to_string_pretty(&facilities, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };

        save_text += "\n===END OF FACILITIES===\n";

        let stored_items = items.stored_items();

        save_text += &match ron::ser::to_string_pretty(&stored_items, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };

        save_text += "\n===END OF STORED ITEMS===\n";

        let equipped_items = items.equipped_items();

        save_text += &match ron::ser::to_string_pretty(&equipped_items, PrettyConfig::new()) {
            Err(error) => {
                println!("unable to save: {:?}", error);
                return "".into();
            }
            Ok(text) => text,
        };

        save_text += "\n===END OF EQUIPPED ITEMS===\n";

        save_text
    }

    pub fn save_file_name<S: ToString>(level: S) -> String {
        use chrono::{Datelike, Timelike};

        let now = Local::now();

        format!(
            "{}-{}-{:0>2}-{:0>2}-{:0>2}{:0>2}.sav",
            level.to_string(),
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
        )
    }

    pub fn save_to_file(data: String) {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new("saves").join(&Self::save_file_name("level1"));
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

pub struct GameLoader {}

impl GameLoader {
    pub fn find_latest_save_file() -> Option<String> {
        use glob::glob;

        let mut files: Vec<String> = glob("saves/*-20??-*.sav")
            .expect("failed to read glob pattern")
            .map(|ref f| {
                if let Ok(path) = f {
                    format!(
                        "{}.{}",
                        path.file_stem().unwrap().to_string_lossy(),
                        path.extension().unwrap().to_string_lossy(),
                    )
                } else {
                    "".to_string()
                }
            })
            .collect();
        files.sort();
        files.reverse();

        if files.len() > 0 {
            Some(files[0].clone())
        } else {
            None
        }
    }

    pub fn load_game<S: ToString>(
        filename: S,
        map: &mut TileMap,
        game_state: &mut GameState,
        timer: &mut Timer,
        update_tx: Option<&GameUpdateSender>,
    ) -> (
        Player,
        BlockingMap,
        CharacterList,
        ItemList,
        FacilityList,
        InventoryList,
    ) {
        let save_data = GameLoader::load_from_file(filename);
        let (new_player, new_characters, new_items, mut new_facilities, new_inventories) =
            GameLoader::load_game_from_save_data(game_state, save_data);

        let mut obstacles = BlockingMap::new();
        obstacles.refresh(&map);

        for (_, facility) in new_facilities.iter() {
            facility.reawaken(timer)
        }

        Level::introduce(
            &new_player,
            map,
            &mut obstacles,
            &new_characters,
            &mut new_facilities,
            &new_items,
            &new_inventories,
            update_tx,
        );

        (
            new_player,
            obstacles,
            new_characters,
            new_items,
            new_facilities,
            new_inventories,
        )
    }

    pub fn load_game_from_save_data(
        game_state: &mut GameState,
        save_data: String,
    ) -> (Player, CharacterList, ItemList, FacilityList, InventoryList) {
        let re = regex::Regex::new("(?sm)(.+)===END OF SERVERDATA===\r?\n(.+)===END OF PLAYERS===\r?\n(.+)===END OF CHARACTERS===\r?\n(.*)===END OF ITEM TYPES===\r?\n(.+)===END OF INVENTORIES===\r?\n(.+)===END OF ITEMS===\r?\n(.+)===END OF FACILITIES===\r?\n(.+)===END OF STORED ITEMS===\r?\n(.+)===END OF EQUIPPED ITEMS===\r?\n").expect("unable to parse regex.");

        let captures = re.captures(&save_data).expect("unable to form captures.");

        let server_data = capture_string(&captures, 1);

        let re = regex::Regex::new(
            r#"(?sm)\s*tick\s*=>\s*(\d+)\s*$\s*NEXT_ITEM_ID\s*=>\s*(\d+)\s*$\s*NEXT_ID\s*=>\s*(\d+)"#,
        ).expect("unable to parge regex");

        let sd_captures = re.captures(&server_data).expect("unable to form captures.");

        let ticks = capture_integer(&sd_captures, 1);
        let next_item_id: u64 = capture_integer(&sd_captures, 2);
        let next_id: u64 = capture_integer(&sd_captures, 3);

        game_state.ticks = ticks;
        GLOBAL_NEXT_ITEM_ID.store(next_item_id, Ordering::SeqCst);
        GLOBAL_NEXT_ID.store(next_id, Ordering::SeqCst);

        let player_data = capture_string(&captures, 2);
        let player: Player = ron::from_str(&player_data)
            .ok()
            .expect("unable to deserialize player");

        let character_data = capture_string(&captures, 3);
        let characters: CharacterList = ron::from_str(&character_data)
            .ok()
            .expect("unable to deserialize characters");

        let inventories_data = capture_string(&captures, 5);
        let mut inventories: InventoryList = ron::from_str(&inventories_data)
            .ok()
            .expect("unable to deserialize inventories");

        let items_data = capture_string(&captures, 6);
        let mut items: ItemList = ron::from_str(&items_data)
            .ok()
            .expect("unable to deserialize items");

        let item_types = &mut items.item_types.clone();
        {
            for (_, inventory) in &mut inventories {
                for (_, mut item) in &mut inventory.items {
                    let updated_item_type =
                        item_types.find(item.item_type.class, &item.item_type.description);
                    item.item_type = updated_item_type.clone();
                }
            }
        }

        let facilities_data = capture_string(&captures, 7);
        let facilities: FacilityList = ron::from_str(&facilities_data)
            .ok()
            .expect("unable to deserialize facilities.");

        let stored_items_data = capture_string(&captures, 8);
        let stored_items: ItemList = ron::from_str(&stored_items_data)
            .ok()
            .expect("unable to deserialize stored items.");

        items.merge(&stored_items);

        let equipped_items_data = capture_string(&captures, 9);
        let equipped_items: ItemList = ron::from_str(&equipped_items_data)
            .ok()
            .expect("unable to deserialize equipped items.");

        items.merge(&equipped_items);

        {
            for (_, item_state) in items.iter_mut() {
                let item_type = ItemState::extract_item(item_state).item_type;
                let updated_item_type = item_types.find(item_type.class, item_type.description);
                item_state.update_item_type(updated_item_type.clone());
            }
        }
        (player, characters, items, facilities, inventories)
    }

    pub fn load_from_file<S: ToString>(filename: S) -> String {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        // Create a path to the desired file
        let path = Path::new("saves").join(&(filename.to_string()));
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {}
        }
        string
    }
}
