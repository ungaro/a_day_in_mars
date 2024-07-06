use log::{debug, info, Level};
mod planet;
mod rockets;
mod scenemanager;

//use rockets::list_rockets;
use std::thread;
use std::time::Duration;

use planet::Planet;
use rockets::Rocket;
use rockets::list_rockets;
use scenemanager::Screen;
use std::io::BufReader;

// width and height, duh
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


//pub const TIMESTEP: f64 = 1.0; // Decreased from 60.0 to make movement more gradual
pub const TIMESTEP: f64 = 3600.0 * 24.0;





turbo::cfg! {r#"
    name = "A day in Mars"
    version = "1.0.0"
    author = "Alp Guneysel"
    description = "what is a day in Mars called?"
    [settings]
    resolution = [1920, 1080]
"#}

fn main() {
    env_logger::init();

    info!("starting up");
    list_rockets();
    // ...
}

// Define the game state initialization
turbo::init! {

    /*
let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
let sink = rodio::Sink::try_new(&handle).unwrap();

let file = std::fs::File::open("assets/outer_space.mp3").unwrap();
sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

sink.sleep_until_end();

    */

    struct GameState {
        is_ready: bool,
        screen: Screen,
        rocket: Rocket,
        tick: u32,
        is_launching: bool,
        planets: Vec<Planet>
    
    } = {
        Self::new()
    }
}

impl GameState {



    pub fn new() -> Self {
        let [screen_w, screen_h] = resolution();
        //Rocket::new();

//    pub  fn new(x: f64, y: f64, radius: f64, color: u32, mass: f64) -> Self {


        let mut sun = Planet::new(0.0, 0.0, 100.0, 0x00FF_FF00, 1.98892_f64 * 10.0_f64.powi(30));
        sun.sun = true;
        let mut earth = Planet::new(
            -1.0 * AU,
            0.0,
            160.0,
            0x0064_95ED,
//            5.9742_f64 * 10.0_f64.powi(24),
            5.9742_f64 * 10.0_f64.powi(24),
        );
        earth.y_vel = 29.783 * 1000.0;
        let mut mars = Planet::new(
            -1.524 * AU,
            0.0,
            120.0,
            0x00BC_2732,
            6.39_f64 * 10.0_f64.powi(23),
        );
        mars.y_vel = 24.077 * 1000.0;
        let mut mercury = Planet::new(
            0.387 * AU,
            0.0,
            80.0,
            0x0050_4E51,
            3.30_f64 * 10.0_f64.powi(23),
        );
        mercury.y_vel = -47.4 * 1000.0;
        let mut venus = Planet::new(
            0.723 * AU,
            0.0,
            140.0,
            0x00FF_FFFF,
            4.8685_f64 * 10.0_f64.powi(24),
        );
        venus.y_vel = -35.02 * 1000.0;

        let mut planets = vec![sun, earth, mars, mercury, venus];
//        let mut planets = vec![sun, earth];
        let mut rocket = Rocket::new();
        Self {
            // Initialize all fields with default values
            tick: 0,
            is_ready: false,
            is_launching:false,
            planets: planets,
            screen: Screen::Title,
            rocket:rocket,

         
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! {
    //text!("Hello, world!!!");
    let mut state = GameState::load();

    let [mut camera_x, mut camera_y] = cam!();




   match state.screen.clone() {
    Screen::Game => {
        draw_game_screen(&state);
        update_game_screen(&mut state);
    }
    Screen::Title => {
        draw_title_screen(&state);
        update_title_screen(&mut state);
    }
    Screen::RocketSelect => {
        draw_rocket_screen(&state);
        update_rocket_screen(&mut state);
    }
}







   state.tick += 1;
   state.save();

}




fn draw_game_screen(state: &GameState){
           // Make a clone of the current state of planets for reading
       //DRAWING



}


fn update_game_screen(state: &mut GameState){
    // Make a clone of the current state of planets for reading
    let planets_clone = state.planets.clone();
    let rocket_clone = state.rocket.clone();

  // Check if rocket has gone too far and reset if necessary
  let max_distance = 20.0 * AU; // 20 times the Earth-Sun distance
  let distance_from_origin = (state.rocket.x.powi(2) + state.rocket.y.powi(2)).sqrt();
  if distance_from_origin > max_distance {
      log!("Rocket too far, resetting position");
      state.rocket = Rocket::new(); // Reset the rocket to its initial state
  }


  state.rocket.update_position();
       // if state.planets.earth.is_within_gravity_range(self.rocket.position()) {
            state.rocket.apply_gravity(state.planets[1].clone());
        //}
        //log!("Rocket position: {:?}", state.rocket.position());



    // Iterate over planets with indices
    for (i, planet) in state.planets.iter_mut().enumerate() {
        // Create a slice of all planets except the current one
        let others = [&planets_clone[..i], &planets_clone[i + 1..]].concat();

        // Update the position of the current planet
        planet.update_position(&others);
    }

    //rocket_clone.update_position();
    //rocket_clone.draw();

let gp = gamepad(0);




// Checks if the A button is JustPressed
if gp.start.pressed() {
    // Handle JustPressed state
    log!("SPACE_PRESSED");
    state.rocket.is_launching = true;
}


    // Update rocket position and apply launch if necessary
    if state.rocket.is_launching {
        log!("ROCKET_IS_LAUNCHING");

        // Calculate initial launch velocity (adjust these values as needed)
        let launch_angle = std::f64::consts::PI / 4.0; // 45 degrees
        let launch_speed = 10000.0; // Adjust this value for desired orbit height
        
        state.rocket.velocity_x = launch_speed * launch_angle.cos();
        state.rocket.velocity_y = launch_speed * launch_angle.sin();
        
        log!("VelO_X {}",state.rocket.velocity_x.to_string());
        log!("VelO_Y {}",state.rocket.velocity_y.to_string());
        // Reset the launching flag
        state.rocket.is_launching = false;
    }

    for planet in &state.planets {
        let dx = planet.x - state.rocket.x;
        let dy = planet.y - state.rocket.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        let force = G * planet.mass / (distance * distance + SOFTENING_FACTOR);
        let angle = dy.atan2(dx);
        
        state.rocket.velocity_x += force * angle.cos() * TIMESTEP;
        state.rocket.velocity_y += force * angle.sin() * TIMESTEP;
    }
    log!("VelO_X {}",state.rocket.velocity_x.to_string());
    log!("VelO_Y {}",state.rocket.velocity_y.to_string());

 // Update rocket position
 state.rocket.x += state.rocket.velocity_x;
 state.rocket.y += state.rocket.velocity_y;


 state.rocket.rotation = state.rocket.velocity_y.atan2(state.rocket.velocity_x);
// state.rocket.update(&state.planets, TIMESTEP);
 state.rocket.update(&state.planets[..], TIMESTEP);


 // Draw the rocket
 state.rocket.draw(SCALE);




    for planet in &state.planets {
        planet.draw();
    }

}




fn draw_title_screen(state: &GameState) {
    let [screen_w, screen_h] = canvas_size!();
    //log!("DRAW_TITLE_SCREEN");
    clear!(0x95bea1ff);
    //sprite!("enemy_meteor");
    sprite!("title_screen");


    let screen_w = screen_w as i32;
    let screen_h = screen_h as i32;
    let center = screen_w / 2;
    //log!("screen width {}",screen_w);

    // Logo
    let x = center - 48;
    let progress = (state.tick * 2).min(screen_h as u32);
    let y = screen_h - (progress as i32).min(screen_h);
    let t = progress as f32 / 10.;
    let scale = 2.0 + (t.sin() / 10.);
    let sw = 96.0 * scale;
    let xoff = sw as i32 / 4;
    let yoff = 32;



        // sprite!("logo", x = x - xoff, y = y + yoff, scale_x = scale, scale_y = scale);
        let x = (screen_w / 2) - ((11 * 8) / 2);
        let y = screen_h / 2;
        rect!(w = screen_w, h = 50, x = 0, y = y - 12, color = 0xff0000ff);
        if state.tick % 60 < 30 {
            text!("PRESS START", font = Font::L, x = x, y = y);
        }
        // Show players who joined
        /*
        let num_players = state.players.len();
        for i in 0..num_players {
            let player = &state.players[i];
            draw_player(&player, num_players > 1);
        }
        */
    
  
}

fn update_title_screen(state: &mut GameState) {

    if gamepad(0).start.just_pressed() || gamepad(0).a.just_pressed() {
        state.screen = Screen::Game;
        state.tick = 0;
    }
    /*
    for i in 1..MAX_PLAYERS {
        let i = i as u32;
        if gamepad(i).a.just_pressed() || gamepad(i).b.just_pressed() {
            if state.players.iter().position(|p| p.id == i).is_none() {
                let mut player = state.players[0].clone();
                player.id = i;
                player.color = PLAYER_COLORS[i as usize];
                state.players.push(player)
            }
        }
    }
    */
}



fn draw_rocket_screen(state: &GameState){
    // Make a clone of the current state of planets for reading
   

}




fn update_rocket_screen(state: &GameState){
    // Make a clone of the current state of planets for reading
   

}
