use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use miniquad::window::set_window_size;
use macroquad::ui::root_ui;


#[macroquad::main("fakemon")]
async fn main() {

    set_window_size(720, 500);

    let mut william = AnimatedSprite::new(
        64,
        64,
        &[ Animation {
            name: "idle".to_string(),
            row: 0,
            frames: 5,
            fps: 12,
        },
        Animation {
            name: "walk_south".to_string(),
            row: 1,
            frames: 6,
            fps: 12,
        },
        Animation {
            name: "walk_north".to_string(),
            row: 2,
            frames: 6,
            fps: 12,
        },
        Animation {
            name: "walk_east".to_string(),
            row: 3,
            frames: 6,
            fps: 12,
        },
        Animation {
            name: "walk_west".to_string(),
            row: 4,
            frames: 6,
            fps: 12,
        }
        ], true);

    let william_texture = load_texture("assets/william.png").await.unwrap();
    let mut william_pos: Vec2 = Vec2::new(50.0, 50.0);

    loop {
        clear_background(DARKGRAY);

        if is_key_down(KeyCode::S) {
            william_pos.y += 2.0;
            william.set_animation(1);
        } else if is_key_down(KeyCode::W) {
            william_pos.y -= 2.0;
            william.set_animation(2);
        } else if is_key_down(KeyCode::D) {
            william_pos.x += 2.0;
            william.set_animation(3);
        } else if is_key_down(KeyCode::A) {
            william_pos.x -= 2.0;
            william.set_animation(4);
        }
        else {
            william.set_animation(0);
        }

        draw_texture_ex(
            &william_texture,
            william_pos.x,
            william_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(william.frame().source_rect),
                dest_size: Some(william.frame().dest_size),
                ..Default::default()
            },
        );

        william.update();

        root_ui().label(None, "");
        if root_ui().button(None, "Start Battle Simulator") {
            println!("Starting Battle Simulator");
            start_battle_sim();
        }

        next_frame().await;
    }
}

fn start_battle_sim() {

    
}
