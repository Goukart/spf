use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut player_x = 100.0;
    let mut player_y = 100.0;

    let mut img_x = 40.0;

    let fren_img = 
        Image::from_file_with_format(include_bytes!("../resource/playersprite.png"), None);

    let mut time = get_time();
    let init_time = time;
    loop {
        clear_background(BLACK);
        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) {
            player_x += 100.0 * delta;
        }
        if is_key_down(KeyCode::Left) {
            player_x -= 100.0 * delta;
        }
        if is_key_down(KeyCode::Down) {
            player_y += 100.0 * delta;
        }
        if is_key_down(KeyCode::Up) {
            player_y -= 100.0 * delta;
        }
        if time  <= 0.1 {
            img_x = 40.0;
        }
        if time > 0.2 && time < 0.3 {
            img_x = 200.0;
        }        

        if time > 0.3 { 
            img_x = 360.0;
        }

        time = get_time();
        time = time % 0.4;
        
        println!("{}, {}", time, init_time);
        let rect = Rect::new(img_x, 0.0, 80.0, 160.0);
        let subfren = fren_img.sub_image(rect);

        let fren_texture = Texture2D::from_image(&subfren);
        fren_texture.update(&subfren);
        draw_texture(fren_texture, player_x, player_y, WHITE);

        next_frame().await
    }
}
