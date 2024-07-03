use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rocket {
    name: String,
    manufacturer: String,
    height: f32,
    diameter: f32,
    mass: f32,
    thrust: f32,
    fuel_capacity: f32,
    max_speed: f32,
    max_altitude: f32,
    stages: u8,
    payload_capacity: f32,
    reliability: f32,
    cost: u64,
    cooldown_time: u32,
    price: u64,
    image: String,
    construction_speed: u32,
}

impl Rocket {
    fn thrust_to_weight_ratio(&self) -> f32 {
        const GRAVITY: f32 = 9.81;  // m/s^2
        self.thrust / (self.mass * GRAVITY)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RocketData {
    rockets: Vec<Rocket>,
}

fn load_rockets_from_json(file_path: &str) -> Result<HashMap<String, Rocket>> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let rocket_data: RocketData = serde_json::from_str(&contents)?;
    
    let mut rockets = HashMap::new();
    for rocket in rocket_data.rockets {
        rockets.insert(rocket.name.clone(), rocket);
    }

    Ok(rockets)
}

pub fn list_rockets() -> Result<()> {
    let rockets = load_rockets_from_json("./data/rockets.json")?;
    
    for (name, rocket) in rockets.iter() {
        println!("Rocket: {}", name);
        println!("Manufacturer: {}", rocket.manufacturer);
        println!("Height: {} m", rocket.height);
        println!("Payload capacity: {} kg", rocket.payload_capacity);
        println!("Price: ${}", rocket.price);
        println!("Construction time: {} days", rocket.construction_speed);
        println!("Thrust-to-Weight Ratio: {:.2}", rocket.thrust_to_weight_ratio());
        println!("---");
    }

    Ok(())
}