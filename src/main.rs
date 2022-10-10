use macroquad::prelude::*;

const AVATAR_HEIGHT: f32 = 25.;

// const MOVE_SPEED: f32 = 0.1;


// ToDo, make CI and CD
// Make Github Pages auto compile all that shit and make it pretty and playable

#[derive(Debug)]
pub struct Avatar {
    // team: Color, might be usefull later?
    name: String,
    pos: Vec3,
    rot: f32,
    vel: Vec2,
    // shape:
}
impl Avatar {
    pub fn new(name: String, pos: Vec3, rot: f32) -> Self {
        Self {
            name,
            pos,
            rot,
            vel: Vec2::new(0., 0.),
        }
    }

    fn draw(&self) {
        draw_cube_wires(self.pos, vec3(2., 2., 2.), DARKBLUE);
        draw_text(self.name.as_str(), self.pos.x, self.pos.y, 1., BLUE);
    }

    pub fn push(&mut self, direction: Vec2) {
        /*
        let v1 = Vec2::new(
            self.pos.x + rotation.sin() * AVATAR_HEIGHT / 2.,
            self.pos.y - rotation.cos() * AVATAR_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.pos.x - rotation.cos() * AVATAR_BASE / 2. - rotation.sin() * AVATAR_HEIGHT / 2.,
            self.pos.y - rotation.sin() * AVATAR_BASE / 2. + rotation.cos() * AVATAR_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.pos.x + rotation.cos() * AVATAR_BASE / 2. - rotation.sin() * AVATAR_HEIGHT / 2.,
            self.pos.y + rotation.sin() * AVATAR_BASE / 2. + rotation.cos() * AVATAR_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
        */
        self.pos += vec3(direction.x, direction.y, 0.);
        self.draw();
    }

    pub fn shoot(&self) -> Bullet {
        let rot_vec = Vec2::new(self.rot.sin(), -self.rot.cos());
        // last_shot = frame_t;
        return Bullet {
            pos: self.pos + vec3(rot_vec.x, rot_vec.y, 0.) * AVATAR_HEIGHT / 2.,
            vel: rot_vec * 7.,
            // shot_at: frame_t,
            shot_at: 0.,
            collided: false,
        }
    }
}

pub struct Bullet {
    pos: Vec3,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

#[macroquad::main("spf")]
async fn main() {
    let mut blue = Avatar {
        name: String::from("blue"),
        pos: Vec3::new(-5., 0., 0.),
        rot: 0., // only rotate z-axis
        vel: Vec2::new(0., 0.),
    };
    println!("blue: {:#?}", blue);

    // let mut bullets = Vec::new();
    let mut last_shot = get_time();
    let mut gameover = false;
    let mut bullets = Vec::new();

    loop {
        /*
        if gameover {
            clear_background(LIGHTGRAY);
            let mut text = "You Win!. Press [enter] to play again.";
            let font_size = 30.;

            if asteroids.len() > 0 {
                text = "Game Over. Press [enter] to play again.";
            }
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
            if is_key_down(KeyCode::Enter) {
                blue = Avatar {
                    pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
                    rot: 0.,
                    vel: Vec2::new(0., 0.),
                };
                bullets = Vec::new();
                asteroids = Vec::new();
                gameover = false;
                screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
                for _ in 0..10 {
                    asteroids.push(Asteroid {
                        pos: screen_center
                            + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                                .normalize()
                                * screen_width().min(screen_height())
                                / 2.,
                        vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
                        rot: 0.,
                        rot_speed: rand::gen_range(-2., 2.),
                        size: screen_width().min(screen_height()) / 10.,
                        sides: rand::gen_range(3, 8),
                        collided: false,
                    })
                }
            }
            next_frame().await;
            continue;
        }
        */

        // 3D Space
        set_camera(&Camera3D {
            position: vec3(0., 5., 15.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        let frame_t = get_time();

        let mut acc = -blue.vel / 100.; // Friction

        // Forward
        let force: f32 = 0.25;
        if is_key_down(KeyCode::Up) {
            // acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
            blue.push(Vec2::new(0., force));
        }
        if is_key_down(KeyCode::Left) {
            blue.push(Vec2::new(-force, 0.));
        }
        if is_key_down(KeyCode::Right) {
            blue.push(Vec2::new(force, 0.));
        }
        if is_key_down(KeyCode::Down) {
            blue.push(Vec2::new(0., -force));
        }

        // Shot
        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.5 {
            bullets.push(blue.shoot());
        }

        // Euler integration
        blue.vel += acc;
        if blue.vel.length() > 5. {
            blue.vel = blue.vel.normalize() * 5.;
        }
        // blue.pos += blue.vel;
        // blue.pos = wrap_around(&blue.pos);

        // Move each bullet
        /*
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }
        */

        // Bullet lifetime
        // bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        // Remove the collided objects
        // bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);

        // You win?
        if gameover {
            continue;
        }

        clear_background(LIGHTGRAY);

        // for drawable -> type? linke an interface, so not a real "class". Maybe with #[derive(drawable)]
        blue.draw();
        /*
        for bullet in bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., BLACK);
        }
        */

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        // Back to screen space, render some text 2D Space
        set_default_camera(); // 2d drawings are not rendered
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
