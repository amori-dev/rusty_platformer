use raylib::prelude::*;

struct Player {
    speed: f32,
    vel: Vector2,
    pos: Vector2,
    is_grounded: bool,
    gravity: f32,
    jumps_left: i32,
    jump_force: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("rusty platformer")
        .build();

    let bg_color: Color = Color::new(73, 170, 137, 255);

    let player_texture = rl
        .load_texture(&thread, "Assets/player.png")
        .expect("Failde to load player");

    let mut player = Player {
        speed: 500.0,
        vel: Vector2{x: 0.0, y: 0.0},
        pos: Vector2{x: 640.0, y: 360.0},
        is_grounded: false,
        gravity: 0.2,
        jumps_left: 2,
        jump_force: 1040.0,
    };

    while !rl.window_should_close() {

        player.is_grounded = player.pos.y >= 655.0;

        // move player
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.vel.x = -player.speed;
        } else if rl.is_key_down(KeyboardKey::KEY_D) {
            player.vel.x = player.speed;
        } else {
            player.vel.x = 0.0;
        }

        // calculate jumps left
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && player.jumps_left != 0 {
            player.jumps_left -=1;
        }

        // reset amount of jumps
        if player.is_grounded {
            player.jumps_left = 2;
            player.vel.y = 0.0;
        } else {
            player.vel.y += player.gravity;
        }

        // handle jump
        if player.jumps_left > 0 && player.jumps_left <= 2 && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            player.vel.y = -player.jump_force;
        }

        // move player
        player.pos.x += player.vel.x * rl.get_frame_time(); 
        player.pos.y += player.vel.y * rl.get_frame_time(); 


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg_color);
        d.draw_texture(&player_texture, player.pos.x as i32, player.pos.y as i32, Color::WHITE);
    }
}
