use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;

#[derive(Deserialize, Serialize)]
struct Snake {
    body: Vec<(u16, u16)>,
    direction: Direction,
    invalid: bool,
}

#[derive(Deserialize, Serialize)]
struct GameState {
    snake: Snake,
    food: (u16, u16),
}

#[derive(Serialize)]
struct ApiResponse {
    body: Vec<(u16, u16)>,
    food: (u16, u16),
    game_over: bool,
}

impl GameState {
    fn update(&mut self) {
        self.snake.move_forward();
        let (head_x, head_y) = self.snake.body.last().unwrap();
        if head_x == &self.food.0 && head_y == &self.food.1 {
            self.snake.grow();
            self.food = (
                rand::random::<u16>() % WIDTH,
                rand::random::<u16>() % HEIGHT,
            );
        }
    }
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![(5, 5), (5, 6), (5, 7)],
            direction: Direction::Right,
            invalid: false,
        }
    }

    fn move_forward(&mut self) {
        let (head_x, head_y) = self.head();
        let new_head = match self.direction {
            Direction::Up if head_y > 0 => (head_x, head_y - 1),
            Direction::Down if head_y < HEIGHT - 1 => (head_x, head_y + 1),
            Direction::Left if head_x > 0 => (head_x - 1, head_y),
            Direction::Right if head_x < WIDTH - 1 => (head_x + 1, head_y),
            _ => {
                self.invalid = true;
                return; // Don't move if out of bounds
            }
        };
        self.body.remove(0);

        self.body.push(new_head);
    }

    fn head(&self) -> (u16, u16) {
        self.body[self.body.len() - 1]
    }

    fn change_direction(&mut self, direction: Direction) {
        if self.direction == Direction::Up && direction == Direction::Down
            || self.direction == Direction::Down && direction == Direction::Up
            || self.direction == Direction::Left && direction == Direction::Right
            || self.direction == Direction::Right && direction == Direction::Left
            || self.direction == Direction::Up && direction == Direction::Up
            || self.direction == Direction::Down && direction == Direction::Down
            || self.direction == Direction::Left && direction == Direction::Left
            || self.direction == Direction::Right && direction == Direction::Right
        {
            return;
        }
        self.direction = direction;
    }

    fn grow(&mut self) {
        let (tail_x, tail_y) = self.body[0];
        let new_tail = match self.direction {
            Direction::Up => (tail_x, tail_y + 1),
            Direction::Down => (tail_x, tail_y - 1),
            Direction::Left => (tail_x + 1, tail_y),
            Direction::Right => (tail_x - 1, tail_y),
        };
        self.body.insert(0, new_tail);
    }
}

async fn new(game: web::Data<Mutex<GameState>>) -> impl Responder {
    println!("Creating new game");
    let mut game = game.lock().unwrap();
    game.snake = Snake::new();
    game.food = generate_food(&game.snake.body);
    HttpResponse::Ok().json(game.snake.body.clone())
}

fn generate_food(snake_body: &Vec<(u16, u16)>) -> (u16, u16) {
    loop {
        let food = (
            rand::random::<u16>() % WIDTH,
            rand::random::<u16>() % HEIGHT,
        );
        if !snake_body.contains(&food) {
            return food;
        }
    }
}

async fn move_snake(
    game: web::Data<Mutex<GameState>>,
    direction: web::Path<String>,
) -> impl Responder {
    println!("Moving snake: {:?}", direction);
    let mut game = game.lock().unwrap();
    let direction = match direction.as_str() {
        "up" => Direction::Up,
        "down" => Direction::Down,
        "left" => Direction::Left,
        "right" => Direction::Right,
        _ => return HttpResponse::BadRequest().body("Invalid direction"),
    };
    game.snake.change_direction(direction);

    if game.snake.head() == game.food {
        println!("Snake ate food");
        game.snake.grow();
        game.food = (
            rand::random::<u16>() % WIDTH,
            rand::random::<u16>() % HEIGHT,
        );
    }
    game.update();
    //game.food = generate_food(&game.snake.body);
    HttpResponse::Ok().json(ApiResponse {
        body: game.snake.body.clone(),
        food: game.food,
        game_over: game.snake.invalid,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game = web::Data::new(Mutex::new(GameState {
        snake: Snake::new(),
        food: (
            rand::random::<u16>() % WIDTH,
            rand::random::<u16>() % HEIGHT,
        ),
    }));
    use http;
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(game.clone())
            .service(web::resource("/new").route(web::post().to(new)))
            .service(web::resource("/move/{direction}").route(web::post().to(move_snake)))
    })
    .bind(("127.0.0.1:8080"))?
    .run()
    .await
}
