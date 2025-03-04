use serde::{Deserialize, Serialize};
use serde_json;

// Define the player Structure 
struct Player {
    name: String,
    position: u32, // id of the room
}
#[derive(Serialize, Deserialize)]
struct Room {
    id: u32,
    description: String,
    north: Option<u32>,
    south: Option<u32>,
    east: Option<u32>,
    west: Option<u32>,
}

fn load_rooms() -> Vec<Room> {
    let rooms: &str  = include_str!("../../data/rooms.json");
    let rooms: Vec<Room> = serde_json::from_str(rooms).unwrap();
    rooms
}

fn main() {
    // Load the game date from JSON File 
    let rooms = load_rooms();
    // Prompt for player's name
    println!("What is your name?");
    let mut player_name = String::new();
    std::io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim();
    // Create a player
    let mut player = Player {
        name: player_name.to_string(),
        position: 0,
    };
    loop {
        let room = &rooms[player.position as usize];
        println!("{}: {}", room.id, room.description);
        println!("{}, you can go : ", player.name);
        // Print the available directions
        if room.north.is_some() {
            println!("- north");
        }
        if room.south.is_some() {
            println!("- south");
        }
        if room.east.is_some() {
            println!("- east");
        }
        if room.west.is_some() {
            println!("- west");
        }
        // Prompt for player's command
        println!("Where do you want to go? (type 'quit' to quit)");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        match command {
            "north" => {
                if let Some(room_id) = room.north {
                    player.position = room_id;
                } else {
                    println!("You can't go that way");
                }
            }
            "south" => {
                if let Some(room_id) = room.south {
                    player.position = room_id;
                } else {
                    println!("You can't go that way");
                }
            }
            "east" => {
                if let Some(room_id) = room.east {
                    player.position = room_id;
                } else {
                    println!("You can't go that way");
                }
            }
            "west" => {
                if let Some(room_id) = room.west {
                    player.position = room_id;
                } else {
                    println!("You can't go that way");
                }
            }
            "quit" => {
                println!("Goodbye, {}!", player.name);
                break;
            }
            _ => {
                println!("I don't understand that command");
            }
        }
    }
}
