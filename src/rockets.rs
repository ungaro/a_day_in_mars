use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;
use std::f64::consts::PI;

use crate::planet::Planet;
use crate::{G, SOFTENING_FACTOR, AU, SCALE, TIMESTEP};

// Constants
const GRAVITY: f64 = 9.8;
const LAUNCH_POWER_INCREASE: f64 = 1_000_000.0;
const MAX_LAUNCH_POWER: f64 = 100_000_000.0;
const PLANET_RADIUS: f64 = 50.0;
// width and height, duh
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Rocket {
    name: String,
    manufacturer: String,
    height: f64,
    diameter: f64,
    mass: f64,
    thrust: f64,
    fuel_capacity: f64,
    max_speed: f64,
    max_altitude: f64,
    stages: u8,
    payload_capacity: f64,
    reliability: f64,
    cost: u64,
    cooldown_time: u32,
    price: u64,
    image: String,
    construction_speed: u32,
    pub is_launching: bool,
    pub x: f64,
    pub y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub rotation: f64,
    launch_power: f64,
}

impl Rocket {
    fn thrust_to_weight_ratio(&self) -> f64 {
        const GRAVITY: f64 = 9.81;
        self.thrust / (self.mass * GRAVITY)
    }

    pub fn new() -> Self {
        Self {
            name: "Falcon 9".to_string(),
            manufacturer: "SpaceX".to_string(),
            height: 70.0,
            diameter: 3.7,
            mass: 5490.54,
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
            x: -1.0 * AU / 1000.0, // Closer to origin
            y: 6371000.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            rotation: 0.0,
            launch_power: 0.0,
            is_launching: false,
        }
    }





    pub fn draw(&self, scale: f64) {
/*
        let [canvas_width, canvas_height] = canvas_size!();
    
        let adjusted_scale = scale * 1e-3; // Increased from 1e-5
        
        let screen_x = ((self.x * adjusted_scale) + (canvas_width as f64 / 2.0)) as i32;
        let screen_y = ((self.y * adjusted_scale) + (canvas_height as f64 / 2.0)) as i32;
    

      //  let screen_x = screen_x.clamp(0, canvas_width as i32 - 1);
      //  let screen_y = screen_y.clamp(0, canvas_height as i32 - 1);

        
*/
let rotation_degrees = (self.rotation * 180.0 / PI) as i32;
        //log!("Screen position: ({}, {})", self.x, self.y);

        sprite!(
            "falcon9",
            x = self.x,
            y = self.y,
            w = 64,
            h = 128,
            color = 0xFFFFFFFF,
            opacity = 1.0,
            rotate = rotation_degrees,
            scale_x = 0.5,
            scale_y = 0.5,
            flip_x = false,
            flip_y = false,
            fps = 0,
        );
    }




    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_velocity(&mut self, x: f64, y: f64) {
        self.velocity_x = x;
        self.velocity_y = y;
        }

    pub fn update_position(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }


pub fn apply_gravity(&mut self, planet:Planet) {
    let dx = planet.x - self.x;
    let dy = planet.y - self.y;
    let distance = (dx * dx + dy * dy).sqrt();

    // Avoid division by zero
    if distance == 0.0 {
        return;
    }

    // Normalize the direction vector
    let direction = (dx / distance, dy / distance);

    // Apply gravitational force with a stronger effect
    let gravity_effect = planet.gravity / (distance * distance); // Inverse-square law for gravity
    self.velocity_x += direction.0 * gravity_effect;
    self.velocity_y += direction.1 * gravity_effect;
}



    pub fn update(&mut self, planets: &[Planet], delta_time: f64) {
        if self.is_launching {
            self.launch_power += LAUNCH_POWER_INCREASE;
            self.launch_power = self.launch_power.min(MAX_LAUNCH_POWER);
            
            let launch_angle = std::f64::consts::PI / 4.0; // 45 degrees
            let acceleration = self.launch_power / self.mass * 1000.0; // Multiply by 1000 for more noticeable effect
            self.velocity_x += acceleration * launch_angle.cos() * delta_time;
            self.velocity_y += acceleration * launch_angle.sin() * delta_time;
            
  
        }

        let sun = planets.iter().find(|p| p.sun).unwrap();
        let dx = sun.x - self.x;
        let dy = sun.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let force = G * sun.mass / (distance * distance);
        let angle = dy.atan2(dx);
        
        self.velocity_x += force * angle.cos() * delta_time / 10_000_000.0;;
        self.velocity_y += force * angle.sin() * delta_time / 10_000_000.0;;




        self.x += self.velocity_x * delta_time;
        self.y += self.velocity_y * delta_time;


        self.x = self.x.mul_add(SCALE, WIDTH as f64 / 2.0) / 10_000_000.0;
        self.y = self.y.mul_add(SCALE, HEIGHT as f64 / 2.0) / 10_000_000.0;



        self.rotation = self.velocity_y.atan2(self.velocity_x);

        log!("Position: ({}, {})", self.x, self.y);
        log!("Velocity: ({}, {})", self.velocity_x, self.velocity_y);
    }
 
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
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
    Ok(())
}