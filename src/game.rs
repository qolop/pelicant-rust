use piston_window::*;
use find_folder;
use bird::Bird;
use graphics::character::CharacterCache;

// TODO:
// *Add image resizer
// *Add obstacles
// *Add objects to background (e.g. clouds)
// *Add collision detection via traditional coordinate checking
// *Add trigger for changing bird type
// *Add night and day

type Color = [f32; 4];
// Colors
const SKY_BLUE: Color = [0.0, 0.75, 1.0, 1.0];
const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];

// Default game values
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    pub width: u32,
    pub height: u32,
    time: f64,
    state: GameState,
    bird: Bird,
}

impl Game {
    pub fn new() -> Game {
        Game {
            width: WIDTH,
            height: HEIGHT,
            time: 0.0,
            state: GameState::Playing,
            bird: Bird::new(),
        }
    }


    pub fn run(&mut self, window: &mut PistonWindow) {
        // we put the image generation here rather than on_render to avoid generating the image on
        // each user action.
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let pelican = assets.join("pelican.png");
        let pelican = Texture::from_path(&mut window.factory,
                                         &pelican,
                                         Flip::None,
                                         &TextureSettings::new())
            .unwrap();

        while let Some(e) = window.next() {
            match e {
                Event::Render(_) => {
                    window.draw_2d(&e, |c, g| {
                        let bird_pos = self.bird.get_position();
                        self.on_render(c, g);
                        println!("out of self.on_render");
                        image(&pelican,
                              c.transform
                                  .trans(bird_pos.0, bird_pos.1),
                              g);
                        println!("Draw pelican");
                    });
                }
                Event::Input(Input::Press(a)) => self.on_input(&a),
                Event::Update(a) => self.on_update(&a),
                // we -150 to the y value in order to make the cursor control the bird from the
                // middle rather than its head
                Event::Input(Input::Move(Motion::MouseCursor(_, y))) => self.bird.y = y - 150f64,
                _ => {}
            }
        }
    }

    fn on_update(&mut self, args: &UpdateArgs) {
        self.update_state();

        self.bird.render();
    }

    fn update_state(&mut self) {
        match self.state {
            GameState::Paused => {
                self.time = 0f64;
                return;
            }
            GameState::GameOver => {
                self.state = GameState::Playing;
                println!("Ready to play again.");
                return;
            }
            _ => {}
        }
    }

    fn on_render(&mut self, c: Context, g: &mut G2d) {
        use graphics::rectangle;
        // let square = rectangle::square(0.0, 0.0, (1 * self.tile_size as i32) as f64);
        clear(SKY_BLUE, g);
        use bird::BirdState::*;
        let bullet_y = self.bird.y;
        let rect = rectangle::Rectangle::new_round(YELLOW, 1.0);
        let mut bullet_x = 300.0;
        if self.bird.state != Flying {
            println!("executing while loop");
            Bullet
            bullet_x += 1.0;
            if bullet_x > WIDTH as f64 {
                self.bird.state = Flying;
            }
            println!("Drawing rect");
            rect.draw([bullet_x, bullet_y - 30.0, 90.0, 90.0], &c.draw_state, c.transform, g);
        }
    }

    fn on_input(&mut self, args: &Button) {
        use piston_window::Button::Keyboard;
        use piston_window::Key;

        match *args {
            Keyboard(Key::Right) |
            Keyboard(Key::D) => {
                println!("pewpew");
                self.bird.attack()
            }
            Keyboard(Key::T) => self.bird.customize(),
            Keyboard(Key::Space) => {
                self.state = match self.state {
                    GameState::Playing => GameState::Paused,
                    _ => GameState::Playing,
                }
            }
            _ => {}
        }
    }
}
