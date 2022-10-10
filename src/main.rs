use macroquad::prelude::*;

#[macroquad::main("spf")]

async fn main() {
    let mut player_x = 100.0;
    let mut player_y = 100.0;

    let mut img_x = 40.0;

    let fren_img =
        Image::from_file_with_format(include_bytes!("../resource/playersprite.png"), None);

    let mut time = get_time();

    let mut right = true;
    let mut left = false;
    loop {
        clear_background(BLACK);
        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) {
            player_x += 100.0 * delta;
            right = false;
            left = true;
        }
        if is_key_down(KeyCode::Left) {
            player_x -= 100.0 * delta;
            right = true;
            left = false;
        }
        if is_key_down(KeyCode::Down) {
            player_y += 100.0 * delta;
        }
        if is_key_down(KeyCode::Up) {
            player_y -= 100.0 * delta;
        }

        if is_key_down(KeyCode::Up) == false
            && is_key_down(KeyCode::Down) == false
            && is_key_down(KeyCode::Left) == false
            && is_key_down(KeyCode::Right) == false
        {
            if right {
                img_x = 40.0;
            }
            if left {
                img_x = 520.0;
            }
            
        } else {
            if time <= 0.1 {
                if right {
                    img_x = 40.0;
                }
                if left {
                    img_x = 520.0;
                }
            }
            if time > 0.2 && time < 0.3 {
                if right {
                    img_x = 200.0;
                }

                if left {
                    img_x = 680.0;
                }
            }

            if time > 0.3 {
                if right {
                    img_x = 360.0;
                }
                if left {
                    img_x = 840.0;
                }
            }
        }
        time = get_time();
        time = time % 0.4;
        let rect = Rect::new(img_x, 0.0, 80.0, 160.0);
        let subimage = fren_img.sub_image(rect);

        let fren_texture = Texture2D::from_image(&subimage);
        fren_texture.update(&subimage);
        draw_texture(fren_texture, player_x, player_y, WHITE);

        next_frame().await
    }
}
