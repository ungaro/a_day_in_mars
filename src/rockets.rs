use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;
use std::f32::consts::PI;



// Constants
const GRAVITY: f32 = 9.8;
const LAUNCH_POWER_INCREASE: f32 = 0.1;
const MAX_LAUNCH_POWER: f32 = 10.0;
const PLANET_RADIUS: f32 = 50.0;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize,Serialize,Deserialize)]
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
    pub is_launching:bool,
    launch_power: f32,
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    rotation: f32,

}

impl Rocket {
    fn thrust_to_weight_ratio(&self) -> f32 {
        const GRAVITY: f32 = 9.81;  // m/s^2
        self.thrust / (self.mass * GRAVITY)
    }

    pub fn new() -> Self {

        Self {
            name:"Falcon 9".to_string(),
            manufacturer: "SpaceX".to_string(),
            height: 70.0,
            diameter: 3.7,
            mass: 549054.0,
            thrust: 7607000.0,
            fuel_capacity: 287400.0,
            max_speed: 7500.0,
            max_altitude: 250000.0,
            stages: 2,
            payload_capacity: 22800.0,
            reliability: 0.98,
            cost: 62000000,
            cooldown_time: 1209600,
            price: 67000000,
            image: "falcon9.png".to_string(),
            construction_speed: 180,
            x:0.0,
            y:0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            rotation: 0.0,
            launch_power: 0.0,
            is_launching: false,

        }


    }


    pub fn update(&mut self, planet: &Planet, delta_time: f32) {
        if self.is_launching {
            self.launch_power += LAUNCH_POWER_INCREASE;
            self.launch_power = self.launch_power.min(MAX_LAUNCH_POWER);
        } else if self.launch_power > 0.0 {
            self.velocity_x = self.launch_power * self.rotation.cos();
            self.velocity_y = self.launch_power * self.rotation.sin();
            self.launch_power = 0.0;
        }

        let dx = planet.x - self.x;
        let dy = planet.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        let gravity_force = GRAVITY / (distance * distance);
        self.velocity_x += gravity_force * dx / distance * delta_time;
        self.velocity_y += gravity_force * dy / distance * delta_time;

        self.x += self.velocity_x * delta_time;
        self.y += self.velocity_y * delta_time;

        self.rotation = self.velocity_y.atan2(self.velocity_x);
    }


    pub fn draw(&self) {
        let [canvas_width, canvas_height] = canvas_size!();


        //log!("ROCKET_DRAW_CALLED");
        sprite!("falcon9", x = canvas_width/2, y = canvas_height/2, fps = fps::FAST);


    }


}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize,Serialize,Deserialize)]
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
    /*
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
    */

    Ok(())
}