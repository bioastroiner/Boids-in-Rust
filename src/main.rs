use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::*;
use macroquad::input::{is_key_down, mouse_position};
use macroquad::math::*;
use macroquad::miniquad::window::{self, request_quit, screen_size};
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line};
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_frame_time};
use macroquad::ui::{root_ui, widgets};
use macroquad::window::{
    clear_background, next_frame, screen_height, screen_width, Conf as WindowCFG,
};
use rand::{random, Rng};
fn window_conf() -> WindowCFG {
    WindowCFG {
        window_title: "My Game".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct Boid {
    pos: Vec2,
    vel: Vec2,
}

#[macroquad::main(window_conf)]
async fn main() {
    /*
       Separation: boids move away from other boids that are too close
       Alignment: boids attempt to match the velocities of their neighbors
       Cohesion: boids move toward the center of mass of their neighbors
    */
    let mut min_speed: f32 = 20.; // Flocking birds (like starlings) are never stationary in flight. So, we'll prevent the speed of any boid from dropping below
    let mut max_speed: f32 = 80.;
    let mut protected_range: f32 = 10.0; //to avoid running into other boids
    let mut visible_range: f32 = 20.0;
    let mut avoid_factor: f32 = 2.;
    let mut turnfactor = 2.;
    let mut matching_factor: f32 = 0.05;
    let mut centring_factor: f32 = 0.0005;
    let mut margin = 50.;
    let mut debug_mode = false;
    let mut boids_vec = vec![Boid::default(); 600];
    let (game_width, game_height) = screen_size();
    for ele in &mut boids_vec {
        ele.pos = vec2(
            rand::thread_rng().gen_range(10.0..(game_width - 10.0)),
            rand::thread_rng().gen_range(10.0..(game_height - 10.0)),
        );
        ele.vel = vec2(
            rand::thread_rng().gen_range(-min_speed..max_speed),
            rand::thread_rng().gen_range(-min_speed..max_speed),
        );
    }
    loop {
        if (is_key_down(macroquad::input::KeyCode::Escape)) {
            request_quit();
        }
        set_default_camera();
        clear_background(BLACK);
        // SEPERATION
        // Each boid attempts to avoid running into other boids.
        // If two or more boids get too close to one another
        // i.e. within one another's protected range, they will steer away from one another.
        let cpy_boids = boids_vec.clone();
        for boid in &mut boids_vec {
            let mut close_d = Vec2::ZERO;
            for other_boid in &cpy_boids {
                if !boid.pos.eq(&other_boid.pos)
                    && other_boid.pos.distance(boid.pos) < protected_range
                {
                    close_d += (boid.pos - other_boid.pos);
                }
            }
            boid.vel += close_d * avoid_factor;
            if debug_mode {
                draw_text(
                    format!("close_d: {}", close_d).as_str(),
                    boid.pos.x,
                    boid.pos.y - 20.0,
                    15.0,
                    WHITE,
                );
                draw_line(
                    boid.pos.x,
                    boid.pos.y,
                    (boid.pos + close_d).x,
                    (boid.pos + close_d).y,
                    8.0,
                    RED,
                );
            }
        }
        // Allignment
        // match velocity with their neighbors
        let cpy_boids = boids_vec.clone();
        for boid in &mut boids_vec {
            let mut vel_avg = Vec2::ZERO;
            let mut neighboring_boids = 0 as u32;
            for other_boid in &cpy_boids {
                if !boid.pos.eq(&other_boid.pos)
                    && other_boid.pos.distance(boid.pos) < visible_range
                {
                    vel_avg += other_boid.vel;
                    neighboring_boids += 1;
                }
            }
            if neighboring_boids > 0 {
                vel_avg = vel_avg / neighboring_boids as f32;
            }
            boid.vel += (vel_avg - boid.vel) * matching_factor;
        }
        // Cohision (moving to the COM)
        let cpy_boids = boids_vec.clone();
        for boid in &mut boids_vec {
            let mut pos_avg = Vec2::ZERO;
            let mut neighboring_boids = 0 as u32;
            for other_boid in &cpy_boids {
                if !boid.pos.eq(&other_boid.pos)
                    && other_boid.pos.distance(boid.pos) < visible_range
                {
                    pos_avg += other_boid.pos;
                    neighboring_boids += 1;
                }
            }
            if neighboring_boids > 0 {
                pos_avg = pos_avg / neighboring_boids as f32;
            }
            boid.vel += (pos_avg - boid.pos) * centring_factor;
        }
        // Screen Edges
        for boid in &mut boids_vec {
            if boid.pos.x < margin {
                boid.vel.x = boid.vel.x + turnfactor;
            }
            if boid.pos.x > screen_width() - margin {
                boid.vel.x = boid.vel.x - turnfactor;
            }
            if boid.pos.y < margin {
                boid.vel.y = boid.vel.y + turnfactor;
            }
            if boid.pos.y > screen_height() - margin {
                boid.vel.y = boid.vel.y - turnfactor;
            }
        }
        for boid in &mut boids_vec {
            let speed = boid.vel.length();
            if speed > max_speed {
                boid.vel = vec2(
                    (boid.vel.x / speed) * max_speed,
                    (boid.vel.y / speed) * min_speed,
                );
            }
            if speed < min_speed {
                boid.vel = vec2(
                    (boid.vel.x / speed) * max_speed,
                    (boid.vel.y / speed) * min_speed,
                );
            }
        }

        for boid in &mut boids_vec {
            boid.pos += boid.vel * get_frame_time();
        }
        for boid in &boids_vec {
            if debug_mode {
                draw_circle_lines(boid.pos.x, boid.pos.y, protected_range, 2.0, GREEN);
                draw_circle_lines(boid.pos.x, boid.pos.y, visible_range, 2.0, YELLOW);
            }
            draw_circle(boid.pos.x, boid.pos.y, 6.0, GREEN);
            draw_line(
                boid.pos.x,
                boid.pos.y,
                (boid.pos + boid.vel.normalize() * 15.0).x,
                (boid.pos + boid.vel.normalize() * 15.0).y,
                3.0,
                YELLOW,
            );
        }
        widgets::Window::new(1, vec2(0., 0.), vec2(340., 120.))
            .movable(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, format!("FPS:{}", get_fps()).as_str());
                ui.label(None, "Tweak Boids");
                ui.slider(2, "min speed", 0.01..200.0, &mut min_speed);
                ui.slider(3, "max speed", 0.01..200.0, &mut max_speed);
                ui.slider(4, "protected_range", 1.0..100.0, &mut protected_range);
                ui.slider(5, "visible_range", 0.01..100.0, &mut visible_range);
                ui.slider(6, "avoid_factor", 0.01..20., &mut avoid_factor);
                ui.slider(7, "matching_factor", 0.01..0.1, &mut matching_factor);
                ui.slider(8, "centring_factor", 0.00001..0.0001, &mut centring_factor);
                ui.slider(11, "margin", 10.0..50.0, &mut margin);
                ui.slider(12, "turn factor", 0.2..10.0, &mut turnfactor);
                ui.checkbox(9, "DEBUG", &mut debug_mode);
                if ui.button(None, "Spawn") {
                    boids_vec.push(Boid {
                        pos: vec2(
                            random::<f32>() * screen_width(),
                            random::<f32>() * screen_height(),
                        ),
                        vel: vec2(
                            random::<f32>() * max_speed * 20.,
                            random::<f32>() * max_speed * 20.,
                        ),
                    });
                    println!("Spawned {:?}", boids_vec.last());
                }
                ui.label(None, format!("Boids: {}", boids_vec.len()).as_str());
                ui.label(None, format!("mouse: {:?}", mouse_position()).as_str());
            });
        next_frame().await;
    }
}
