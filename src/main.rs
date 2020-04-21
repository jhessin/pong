use tetra::{
  graphics::{self, Color, Rectangle, Texture},
  input::{self, Key},
  math::Vec2,
  window, Context, ContextBuilder, Result, State,
};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;

const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

struct Entity {
  texture: Texture,
  position: Vec2<f32>,
  velocity: Vec2<f32>,
}

impl Entity {
  fn new(texture: Texture, position: Vec2<f32>) -> Entity {
    Entity::with_velocity(texture, position, Vec2::zero())
  }

  fn with_velocity(
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
  ) -> Entity {
    Entity { texture, position, velocity }
  }

  fn width(&self) -> f32 {
    self.texture.width() as f32
  }

  fn height(&self) -> f32 {
    self.texture.height() as f32
  }

  fn bounds(&self) -> Rectangle {
    Rectangle::new(
      self.position.x,
      self.position.y,
      self.width(),
      self.height(),
    )
  }

  fn center(&self) -> Vec2<f32> {
    Vec2::new(
      self.position.x + (self.width() / 2.0),
      self.position.y + (self.width() / 2.0),
    )
  }
}

struct GameState {
  player1: Entity,
  player2: Entity,
  ball: Entity,
}

impl GameState {
  fn new(ctx: &mut Context) -> Result<GameState> {
    let p1_texture = Texture::new(ctx, "./resources/player1.png")?;
    let p1_position =
      Vec2::new(16.0, (WINDOW_HEIGHT - p1_texture.height() as f32) / 2.0);

    let p2_texture = Texture::new(ctx, "./resources/player2.png")?;
    let p2_position = Vec2::new(
      WINDOW_WIDTH - p1_texture.width() as f32 - 16.0,
      (WINDOW_HEIGHT - p2_texture.height() as f32) / 2.0,
    );

    let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
    let ball_position = Vec2::new(
      WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
      WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
    );
    let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

    Ok(GameState {
      player1: Entity::new(p1_texture, p1_position),
      player2: Entity::new(p2_texture, p2_position),
      ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
    })
  }
}

impl State for GameState {
  fn update(&mut self, ctx: &mut Context) -> Result<()> {
    if input::is_key_down(ctx, Key::Comma) {
      self.player1.position.y -= PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::O) {
      self.player1.position.y += PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::Up) {
      self.player2.position.y -= PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::Down) {
      self.player2.position.y += PADDLE_SPEED;
    }

    if self.player1.position.y < 0.0 {
      self.player1.position.y = 0.0;
    } else if self.player1.position.y > WINDOW_HEIGHT - self.player1.height() {
      self.player1.position.y = WINDOW_HEIGHT - self.player1.height();
    }

    if self.player2.position.y < 0.0 {
      self.player2.position.y = 0.0;
    } else if self.player2.position.y > WINDOW_HEIGHT - self.player2.height() {
      self.player2.position.y = WINDOW_HEIGHT - self.player2.height();
    }

    self.ball.position += self.ball.velocity;

    let p1_bounds = self.player1.bounds();
    let p2_bounds = self.player2.bounds();
    let ball_bounds = self.ball.bounds();

    let paddle_hit = if ball_bounds.intersects(&p1_bounds) {
      Some(&self.player1)
    } else if ball_bounds.intersects(&p2_bounds) {
      Some(&self.player2)
    } else {
      None
    };

    if let Some(paddle) = paddle_hit {
      // Increase the balls velocity and flip it.
      self.ball.velocity.x =
        -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

      // Calculate the offset between the paddle and the ball
      let offset = (paddle.center().y - self.ball.center().y) / paddle.height();

      // Apply the spin to the ball
      self.ball.velocity.y += PADDLE_SPIN * -offset;
    }

    if self.ball.position.y <= 0.0
      || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
    {
      self.ball.velocity.y = -self.ball.velocity.y;
    }

    if self.ball.position.x < 0.0 {
      window::quit(ctx);
      println!("Player 2 wins!");
    }

    if self.ball.position.x > WINDOW_WIDTH {
      window::quit(ctx);
      println!("Player 1 wins!");
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> Result {
    graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

    graphics::draw(ctx, &self.player1.texture, self.player1.position);
    graphics::draw(ctx, &self.player2.texture, self.player2.position);
    graphics::draw(ctx, &self.ball.texture, self.ball.position);

    Ok(())
  }
}

fn main() -> tetra::Result {
  ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}
