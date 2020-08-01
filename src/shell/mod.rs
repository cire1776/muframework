use muframework::{game, Command};
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::*;

pub fn shell_loop(command_tx: Sender<Command>) {
    loop {
        #[cfg(target_os = "windows")]
        let addrs = [
            SocketAddr::from(([127, 0, 0, 1], 7878)),
            SocketAddr::from(([127, 0, 9, 1], 7879)),
        ];
        #[cfg(target_os = "linux")]
        let addrs = [
            SocketAddr::from(([127, 0, 0, 1], 7878)),
            SocketAddr::from(([127, 0, 9, 1], 7879)),
        ];
        #[cfg(target_os = "macos")]
        let addrs = [
            SocketAddr::from(([192, 168, 1, 134], 7878)),
            SocketAddr::from(([192, 168, 1, 134], 7879)),
        ];

        let listener = TcpListener::bind(&addrs[..]).unwrap();

        println!(
            "listening for telnet on: {:?}",
            listener.local_addr().unwrap()
        );
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            stream.write(&[0xff, 0xfd, 3]).unwrap();
            let mut input = read_to_vector(&mut stream);

            input.reverse();
            while !input.is_empty() {
                let char = input.pop().unwrap();
                print!("{} ", char);
                if char == 0 {
                    break;
                }
            }
            println!("");

            stream
                .write("Welcome to the Mufra Shell\r\nAuthorized Users Only!\r\n".as_bytes())
                .unwrap();

            loop {
                if !handle_connection(&mut stream, command_tx.clone()) {
                    break;
                }
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream, command_tx: Sender<Command>) -> bool {
    let result = stream.write("\r\n> ".as_bytes());
    if result.is_err() {
        return false;
    }
    stream.flush().unwrap();

    let mut input: String = "".to_string();
    loop {
        let char = read_from_stream(stream);
        input += &char;
        if char.len() != 1 || char == "\n" || char == "\r\n" {
            break;
        }
        println!("{}", char);
    }

    let mut args: Vec<&str> = input.split_ascii_whitespace().rev().collect();
    let command = args.pop();
    if command.is_none() {
        return true;
    }

    let mut response = "".to_string();

    match command.unwrap() {
        "exit" => return false,
        "quit_game" => {
            stream.write("Are you sure Y/n: ".as_bytes()).unwrap();

            let reply = read_from_stream(stream);
            if reply == "Y" {
                Command::send(Some(command_tx.clone()), Command::QuitGame);
            }
        }
        "save" => Command::send(Some(command_tx), Command::SaveGame),
        "spawn_item" => {
            response = "help: spawn_item inventory_id item_class \"description\"".to_string();
            let inventory_id = args.pop();
            if let Some(inventory_id) = inventory_id {
                if let Some(inventory_id) = inventory_id.parse::<u64>().ok() {
                    if let Some(class_name) = args.pop() {
                        if let Ok(item_class) = game::items::ItemClass::from_name(class_name) {
                            args.reverse();
                            let description = args.join(" ").trim_matches('"').to_string();

                            Command::send(
                                Some(command_tx.clone()),
                                Command::SpawnItem(inventory_id, item_class, description),
                            );
                            Command::send(Some(command_tx), Command::RefreshInventory);
                            response = "Ok.".into();
                        }
                    }
                }
            };
        }
        "spawn_items" => {
            response =
                "help: spawn_items inventory_id quantity item_class \"description\"".to_string();
            let inventory_id = args.pop();
            if let Some(inventory_id) = inventory_id {
                if let Some(inventory_id) = inventory_id.parse::<u64>().ok() {
                    if let Some(quantity_str) = args.pop() {
                        if let Some(quantity) = quantity_str.parse::<u8>().ok() {
                            if let Some(class_name) = args.pop() {
                                if let Ok(item_class) =
                                    game::items::ItemClass::from_name(class_name)
                                {
                                    args.reverse();
                                    let description = args.join(" ").trim_matches('"').to_string();

                                    Command::send(
                                        Some(command_tx.clone()),
                                        Command::SpawnItems(
                                            inventory_id,
                                            quantity,
                                            item_class,
                                            description,
                                        ),
                                    );
                                    Command::send(
                                        Some(command_tx.clone()),
                                        Command::RefreshInventory,
                                    );
                                    response = "Ok.".into();
                                }
                            }
                        }
                    }
                }
            };
        }
        "spawn_facility" => {
            response = "spawn_facility x y class \"description\" \"properties\"".into();

            let re = regex::Regex::new(r#"spawn_facility (\d+) (\d+) (\w+) "([^"]+)" "([^"]*)""#)
                .expect("unable to form Regex");

            if let Some(captures) = re.captures(&input) {
                if let Some(x) = captures.get(1).unwrap().as_str().parse().ok() {
                    if let Some(y) = captures.get(1).unwrap().as_str().parse().ok() {
                        if let Some(class) = muframework::FacilityClass::from_string(
                            captures.get(3).unwrap().as_str(),
                        ) {
                            let description = captures.get(4).unwrap().as_str().to_string();
                            let properties = captures.get(5).unwrap().as_str().to_string();
                            Command::send(
                                Some(command_tx),
                                Command::SpawnFacility(x, y, class, description, properties),
                            );
                            response = "Ok.".into();
                        }
                    }
                }
            }
        }
        //
        "give_player_level" => {
            response = "give_player_level player_id skill level".into();

            let player_id = args.pop();
            if let Some(player_id) = player_id {
                if let Some(player_id) = player_id.parse::<u64>().ok() {
                    if let Some(skill_str) = args.pop() {
                        let skill = game::Skill::from_string(skill_str.to_lowercase());
                        let skill_level_str = args.pop();
                        if let Some(skill_level_str) = skill_level_str {
                            if let Some(skill_level) = skill_level_str.parse::<u8>().ok() {
                                Command::send(
                                    Some(command_tx),
                                    Command::SetSkillLevel(player_id, skill, skill_level),
                                );
                                response = "Ok.".into();
                            }
                        }
                    }
                }
            }
        }
        "set_facility_property" => {
            response = r#"set_facility_property facility_id "property_name" new_value"#.into();
            let re = regex::Regex::new(r#"set_facility_property (\d+) "([^"]+)" (\d+)"#)
                .expect("unable to form regex");

            let captures = re.captures(&input).expect("unable to capture input");

            if let Some(id_str) = captures.get(1) {
                if let Some(id) = id_str.as_str().parse().ok() {
                    if let Some(property_name) = captures.get(2) {
                        let property_name = property_name.as_str().to_string();
                        if let Some(new_value_str) = captures.get(3) {
                            if let Some(new_value) = new_value_str.as_str().parse::<i128>().ok() {
                                Command::send(
                                    Some(command_tx),
                                    Command::SetFacilityProperty(id, property_name, new_value),
                                )
                            }
                        }
                    }
                }
            }
        }

        _ => {
            args.reverse();
            response = format!("{} {:?}", command.unwrap(), args);
        }
    }

    stream.write(response.as_bytes()).unwrap();

    true
}

fn read_from_stream<'a>(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let input = String::from_utf8_lossy(&buffer[..]);
    input.trim_matches('\0').trim().to_string()
}

fn read_to_vector(stream: &mut TcpStream) -> Vec<u8> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    buffer.to_vec()
}
