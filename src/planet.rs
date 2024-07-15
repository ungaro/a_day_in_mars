//use minifb::{Key, Window, WindowOptions};
use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;
use std::thread;
use std::time::Duration;
use turbo::{path,circ,text,log};
use turbo::canvas::{Font,clear};
use log::{Level,debug,info};

// width and height, duh
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

// astronomical unit in meters (average distance from Earth to the Sun)
const AU: f64 = 149.6e6 * 1000.0;

// gravitational constant in the SI unit of m^3 kg^-1 s^-2
const G: f64 = 6.67428e-11;

// used to avoid numerical instability in gravitational calculations at close distances
const SOFTENING_FACTOR: f64 = 1.0e9;

// scaling factor to convert astronomical units to screen coordinates
const SCALE: f64 = 250.0 / AU;

// time step for the simulation in seconds (1 day in this case)
const TIMESTEP: f64 = 3600.0 * 24.0;




#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Planet {
    pub x: f64,                 // x-coordinate of the planet's position
    pub y: f64,                 // y-coordinate of the planet's position
    radius: f64,            // radius of the planet
    color: u32,             // color code for visualization
    sprite: String,
    pub mass: f64,              // mass of the planet
    pub orbit: Vec<(f64, f64)>, // list of orbital coordinates for visualization
    pub sun: bool,              // indicates whether the planet represents the sun
    pub distance_to_sun: f64,   // distance from the planet to the sun
    pub x_vel: f64,             // velocity of the planet along the x-axis
    pub y_vel: f64,  
    pub gravity: f64
    
               // velocity of the planet along the y-axis
}

impl Planet {
    // create a new planet with the given properties

    pub fn is_within_gravity_range(&self, rocket_position: (f64, f64)) -> bool {
        let distance = ((rocket_position.0 - self.x).powi(2) + (rocket_position.1 - self.y).powi(2)).sqrt();
        distance < 100.0
    }

    pub  fn new(x: f64, y: f64, radius: f64, color: u32, mass: f64) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            mass,
            orbit: Vec::new(),
            sun: false,
            distance_to_sun: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
            sprite: "PLANET".to_string(),
            gravity: 9.8,

        }
    }

    // draw the planet on the window and update its orbit path
    pub fn draw(&self) {

        // update the orbit path to visualize the planet's movement
        self.update_orbit_points();

        // calculate the planet's position on the window and draw it
        let x = self.x.mul_add(SCALE, WIDTH as f64 / 2.0) as usize;
        let y = self.y.mul_add(SCALE, HEIGHT as f64 / 2.0) as usize;

        circ!(
            //d= self.radius,
            d= self.radius,
            x= x,
            y= y,
            border_width=1,
            border_color= self.color,

        );


        //circ!(x = pancake.x, y = pancake.y + 1.0, d = pancake.radius + 2., color = 0x000000aa); // Render the pancakes


        // display the distance to the sun (unless it's the sun itself)
        if !self.sun {
            let distance_text = format!("{:.1}km", self.distance_to_sun / 1000.0);
            //draw_text(buffer, &distance_text, x, y, self.color);
            debug!("DISTANCE_TEXT {}",&distance_text);

            text!( &distance_text, x= x, y = y, font = Font::S,);



        }
    }

    // keep the orbit path up to date with the planet's current position
    fn update_orbit_points(&self) {
        // calculate and update the visual orbit path
        for (i, (x, y)) in self.orbit.iter().enumerate() {
            //log!("ORBIT point {}: {} {}", i, x, y);
        }
                let updated_points: Vec<(usize, usize)> = self
            .orbit
            .iter()
            .map(|&(x, y)| {
                (
                    x.mul_add(SCALE, WIDTH as f64 / 2.0) as usize,
                    y.mul_add(SCALE, HEIGHT as f64 / 2.0) as usize,
                )
            })
            .collect();

        // draw lines connecting the updated orbit points
        for window in updated_points.windows(2) {
//log!("POINTS {} {}",window[0].0, window[0].1);
            path!(
                start= (window[0].0, window[0].1),
                end= (window[1].0, window[1].1),
                color= self.color,
                width= 1,
                border_radius= 0,
            )
            /*
            draw_line(
                buffer,
                window[0].0,
                window[0].1,
                window[1].0,
                window[1].1,
                self.color,
            );
            */
        }
    }


    

    // update the planet's position and velocity due to gravitational forces
    pub fn update_position(&mut self, planets: &[Self]) {
        //log!("Before Update: x = {}, y = {}, x_vel = {}, y_vel = {}", self.x, self.y, self.x_vel, self.y_vel);
        // initialize variables to store total force components
        let mut total_force_x = 0.0;
        let mut total_force_y = 0.0;

        // calculate gravitational forces between this planet and others
        for other in planets {
            if self as *const _ != other as *const _ {
                let (fx, fy, distance) = self.calculate_force(other);
                if !fx.is_finite() || !fy.is_finite() {
                    log!("Non-finite force calculated for planet: {self:?}");
                    log!("Position of other planet involved: {other:?}");
                    continue;
                }

                // update total force components and track the distance to the sun (if it's the sun)
                total_force_x += fx;
                total_force_y += fy;
                //log!("Total Force: fx = {}, fy = {}", total_force_x, total_force_y);

                if other.sun {
                    self.distance_to_sun = distance;
                }
            }
        }

        // update velocities based on the total force and time step
        self.x_vel += total_force_x / self.mass * TIMESTEP;
        self.y_vel += total_force_y / self.mass * TIMESTEP;

        // check for non-finite velocities and skip position update in case of errors
        if !self.x_vel.is_finite() || !self.y_vel.is_finite() {
            log!(
                "Non-finite velocity calculated: x_vel = {}, y_vel = {}",
                self.x_vel, self.y_vel
            );
            return; // skip updating the position to avoid further issues
        }

        // calculate and update new positions based on updated velocities
        let new_x = self.x_vel.mul_add(TIMESTEP, self.x);
        let new_y = self.y_vel.mul_add(TIMESTEP, self.y);

        // check if the updated positions are finite
        if new_x.is_finite() && new_y.is_finite() {
            self.x = new_x;
            self.y = new_y;

        } else {
            log!("Non-finite position calculated: x = {new_x}, y = {new_y}");
        }


        // add the current position to the orbit path for the visual effect
        self.orbit.push((self.x, self.y));
        //log!("After Update: x = {}, y = {}, x_vel = {}, y_vel = {}", self.x, self.y, self.x_vel, self.y_vel);
    }

    // Calculate gravitational force components and distance to another planet
    fn calculate_force(&self, other: &Self) -> (f64, f64, f64) {
        // calculate the components and magnitude of the distance between two planets
        let distance_x = other.x - self.x;
        let distance_y = other.y - self.y;
        let distance_squared = distance_x.powi(2) + distance_y.powi(2) + SOFTENING_FACTOR.powi(2);
        let distance = distance_squared.sqrt();

        // calculate the gravitational force components and direction
        let force = G * self.mass * other.mass / distance.powi(2);
        let theta = distance_y.atan2(distance_x);
        let force_x = theta.cos() * force;
        let force_y = theta.sin() * force;

        // return force components and distance
        (force_x, force_y, distance)
    }




}



