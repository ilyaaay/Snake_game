use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, video::Window,
};
use std::{ops::Add, time::Duration};

const WINDOW_WEIGHT: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const DOT_SIZE: u32 = 20;

pub enum State {
    Playing,
    Paused,
}

pub enum MoveType {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

pub struct GameContext {
    pub position: Vec<Point>,
    pub player_direction: MoveType,
    pub food: Point,
    pub state: State,
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: MoveType::Right,
            food: Point(3, 3),
            state: State::Paused,
        }
    }

    pub fn next_tick(&mut self) {
        if let State::Paused = self.state {
            return;
        }

        let head_position = self.position.first().unwrap();
        let next_head_position = match self.player_direction {
            MoveType::Up => *head_position + Point(0, -1),
            MoveType::Down => *head_position + Point(0, 1),
            MoveType::Right => *head_position + Point(1, 0),
            MoveType::Left => *head_position + Point(-1, 0),
        };

        self.position.pop();
        self.position.reverse();
        self.position.push(next_head_position);
        self.position.reverse();
    }

    pub fn move_up(&mut self) {
        self.player_direction = MoveType::Up;
    }

    pub fn move_down(&mut self) {
        self.player_direction = MoveType::Down;
    }

    pub fn move_right(&mut self) {
        self.player_direction = MoveType::Right;
    }

    pub fn move_left(&mut self) {
        self.player_direction = MoveType::Left;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            State::Playing => State::Paused,
            State::Paused => State::Playing,
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|err| err.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE as i32,
            y * DOT_SIZE as i32,
            DOT_SIZE,
            DOT_SIZE,
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            State::Playing => Color::RGB(0, 0, 0),
            State::Paused => Color::RGB(30, 30, 30),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.position {
            self.draw_dot(point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;

        Ok(())
    }
}

fn game_loop() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snake_game", WINDOW_WEIGHT, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|err| err.to_string())?;

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_counter = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => context.move_up(),
                    Keycode::A => context.move_left(),
                    Keycode::S => context.move_down(),
                    Keycode::D => context.move_right(),
                    Keycode::Escape => context.toggle_pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context)?;
    }

    Ok(())
}

fn main() {
    game_loop().ok();
}
