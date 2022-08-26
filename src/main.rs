mod position;
mod food;
mod assets;
mod snake;
//use ggez::audio::SoundSource;
use ggez::input::mouse::MouseButton;
use ggez::graphics::PxScale;
use ggez::ContextBuilder;
use ggez::conf::{Conf, WindowMode};
use ggez::event::KeyCode;
use std::time::Duration;
use crate::time::Instant;
use ggez::event;
use ggez::filesystem;
use std::time;
use std::collections::LinkedList;
use crate::position::Position;
use crate::food::Food;
use crate::food::Kind;
use crate::snake::{Direction,Snake};
use crate::assets::Assets;
use crate::assets::Sounds;
use ggez::{Context, GameResult};
use ggez::graphics;



use std::env;
use std::path;

const SCREEN_SIZE_X: f32 = 480.0;
const SCREEN_SIZE_Y: f32 = 480.0;
const CELL_SIZE: i32 = 30;
const START_POS_X: i32 = 4;
const START_POS_Y: i32 = 4;
const EXTRA_SCREEN: f32 =120.0;
const EASY_UPDATES_PER_SECOND: f32 = 4.0;
const MEDIUM_UPDATES_PER_SECOND: f32 = 5.5;
const HARD_UPDATES_PER_SECOND: f32 = 7.0;
const EASY_MILLIS_PER_UPDATE: u64 = (1.0 / EASY_UPDATES_PER_SECOND * 1000.0) as u64;
const MEDIUM_MILLIS_PER_UPDATE: u64 = (1.0 / MEDIUM_UPDATES_PER_SECOND * 1000.0) as u64;
const HARD_MILLIS_PER_UPDATE: u64 = (1.0 / HARD_UPDATES_PER_SECOND * 1000.0) as u64;

const MAX_POS: i32=15;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State{
    Menu,
    Help,
    GameOver,
    Playing,
    Difficulty,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty{
    Easy,
    Medium,
    Hard,
}

pub struct MainState{
    last_update: time::Instant,
    snake: Snake,
    foods: LinkedList<Food>,
    assets: Assets,
    state:State,
    sounds:Sounds,
    can_move:bool,
    difficulty:Difficulty         
}

impl MainState{
    pub fn new(ctx: &mut Context)->GameResult<MainState>{
    let assets = Assets::new(ctx)?;
    let sn=Snake::new(Position::new(START_POS_X,START_POS_Y));
    let  foods=LinkedList::new();
    
    let m=MainState{
        last_update: time::Instant::now(),
        snake:sn,
        foods:foods,
        assets:assets,
        state:State::Menu,
        sounds:Sounds::new(ctx).unwrap(),
        can_move:true,
        difficulty:Difficulty::Easy,

    };
    Ok(m)

    }

    fn draw_help(&mut self, ctx: &mut Context) -> GameResult<()>{
        let white = graphics::Color::from_rgb(255,255,255);
        let yellow=graphics::Color::from_rgb(255,255,0);
        let font = graphics::Font::new(ctx,"/DejaVuSerif.ttf")?;
            let f=Food::new(Position::new(1,1),Kind::Normalburger);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Gives you a point and the snake grows")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,1));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,2),Kind::Normalduner);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Gives you a point and the snake grows")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,2));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,3),Kind::Killingburger);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Gives you a point,but you die :(")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,3));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,4),Kind::Killingduner);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Gives you a point,but you die :(")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,4));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,5),Kind::Poisonousduner);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("You will lose a square on every movement,until you take an antidote,that will be spawned")).color(white).font(font).scale(PxScale{x:10.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,5));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,6),Kind::Poisonousburger);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("You will lose a square on every movement,until you take an antidote,that will be spawned")).color(white).font(font).scale(PxScale{x:10.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,6));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,7),Kind::Mushroom);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Half of the snake becomes invisible,until you take an antidote,that will be spawned")).color(white).font(font).scale(PxScale{x:10.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,7));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,8),Kind::Jackdaniels);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Your direction buttons are switched,until you eat a pepper,that will be spawned")).color(white).font(font).scale(PxScale{x:10.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,8));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,9),Kind::Antidote);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("Saves you from poison and mushroom")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,9));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,10),Kind::Pepper);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("You become sober")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,10));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,11),Kind::Steak);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("You receive 3points")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,11));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,12),Kind::Protein);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("You receive a bonus point for every food eaten for a random period")).color(white).font(font).scale(PxScale{x:12.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,12));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let f=Food::new(Position::new(1,13),Kind::Cactus);
            f.draw(ctx, &self.assets)?;
            let description=graphics::TextFragment::new(format!("You lose 2points")).color(white).font(font).scale(PxScale{x:20.0,y:30.0});  
            let draw_params=graphics::DrawParam::default().dest(Position::new(2,13));
            graphics::draw(ctx,&graphics::Text::new(description),draw_params)?;
            let back_button=graphics::TextFragment::new(format!("BACK")).color(yellow).font(font).scale(PxScale{x:150.0,y:150.0});
            let draw_params1=graphics::DrawParam::default().dest(Position::new(2,15));
            graphics::draw(ctx,&graphics::Text::new(back_button),draw_params1)?;

            graphics::present(ctx)?;
            return Ok(())  
    }

    fn draw_game_over(&mut self, ctx: &mut Context) -> GameResult<()>{
         
          let white = graphics::Color::from_rgb(255,255,255);
          let yellow=graphics::Color::from_rgb(255,255,0);
          let font = graphics::Font::new(ctx,"/DejaVuSerif.ttf")?;
           let game_over_score= graphics::TextFragment::new(format!("GAME OVER \n Score:{} \n{}",self.snake.points,self.snake.killed_by)).color(white).font(font).scale(PxScale{x:30.0,y:30.0});
           let restart_button= graphics::TextFragment::new(format!("Restart")).color(yellow).font(font).scale(PxScale{x:100.0,y:100.0});
           let menu_button= graphics::TextFragment::new(format!("Menu")).color(yellow).font(font).scale(PxScale{x:100.0,y:100.0});
           let draw_params1 = graphics::DrawParam::default().dest(Position::new(MAX_POS/4,MAX_POS/7));
           let draw_params2 = graphics::DrawParam::default().dest(Position::new(MAX_POS/6,MAX_POS/2));
           let draw_params3 = graphics::DrawParam::default().dest(Position::new(MAX_POS/4,(MAX_POS/5)*4));
           graphics::draw(ctx,&graphics::Text::new(game_over_score),draw_params1)?;
           graphics::draw(ctx,&graphics::Text::new(restart_button),draw_params2)?;
           graphics::draw(ctx,&graphics::Text::new(menu_button),draw_params3)?;
            graphics::present(ctx)?;
            return Ok(())
    }

    fn draw_menu(&mut self, ctx: &mut Context) -> GameResult<()>{
        let yellow=graphics::Color::from_rgb(255,255,0);
        let font = graphics::Font::new(ctx,"/DejaVuSerif.ttf")?;
        let play_button=graphics::TextFragment::new(format!("PLAY")).color(yellow).font(font).scale(PxScale{x:150.0,y:150.0});  
        let draw_params1=graphics::DrawParam::default().dest(Position::new(MAX_POS/4,MAX_POS/4));
        graphics::draw(ctx,&graphics::Text::new(play_button),draw_params1)?;
        let help_button=graphics::TextFragment::new(format!("HELP")).color(yellow).font(font).scale(PxScale{x:150.0,y:150.0});  
        let draw_params2=graphics::DrawParam::default().dest(Position::new(MAX_POS/4,(MAX_POS/5)*4));
        graphics::draw(ctx,&graphics::Text::new(help_button),draw_params2)?;
        graphics::present(ctx)?;
          return Ok(()) 

    }
    fn draw_difficulty_menu(&mut self, ctx: &mut Context) -> GameResult<()>{
        let yellow=graphics::Color::from_rgb(255,255,0);
        let font = graphics::Font::new(ctx,"/DejaVuSerif.ttf")?;
        let easy_button=graphics::TextFragment::new(format!("EASY")).color(yellow).font(font).scale(PxScale{x:110.0,y:110.0});  
        let draw_params1=graphics::DrawParam::default().dest(Position::new(3,2));
        graphics::draw(ctx,&graphics::Text::new(easy_button),draw_params1)?;
        let medium_button=graphics::TextFragment::new(format!("MEDIUM")).color(yellow).font(font).scale(PxScale{x:110.0,y:110.0});  
        let draw_params2=graphics::DrawParam::default().dest(Position::new(1,7));
        graphics::draw(ctx,&graphics::Text::new(medium_button),draw_params2)?;
        let hard_button=graphics::TextFragment::new(format!("HARD")).color(yellow).font(font).scale(PxScale{x:110.0,y:110.0});  
        let draw_params3=graphics::DrawParam::default().dest(Position::new(3,12));
        graphics::draw(ctx,&graphics::Text::new(hard_button),draw_params3)?;
        graphics::present(ctx)?;
          return Ok(()) 

    }
}

impl event::EventHandler for MainState{
     fn update(&mut self,ctx: &mut Context)->  GameResult<()> {
        if self.state != State::Playing {
            return Ok(());
        }
        let n = Instant::now();
		if self.difficulty==Difficulty::Easy{
        if n - self.last_update <= Duration::from_millis(EASY_MILLIS_PER_UPDATE) {
			return Ok(());
		}
       } 
       if self.difficulty==Difficulty::Medium{
        if n - self.last_update <= Duration::from_millis(MEDIUM_MILLIS_PER_UPDATE) {
			return Ok(());
		}
       }
       if self.difficulty==Difficulty::Hard{
        if n - self.last_update <= Duration::from_millis(HARD_MILLIS_PER_UPDATE) {
			return Ok(());
		}
       }        

		self.last_update = n;
		self.snake.update(&mut self.foods,ctx,&mut self.sounds);
        self.can_move=true;
        if !self.snake.is_alive{self.state=State::GameOver};
		Ok(())
     }

     fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {  if self.state==State::Playing && self.can_move{
        self.can_move=false;
        if !self.snake.is_drunk{
        match keycode{
            KeyCode::Right =>self.snake.change_dir(Direction::Right),
            KeyCode::Left =>self.snake.change_dir(Direction::Left),
            KeyCode::Up  =>self.snake.change_dir(Direction::Up),
            KeyCode::Down =>self.snake.change_dir(Direction::Down),
            _ => (),
        }
       }
       else{
        match keycode{
            KeyCode::Right =>self.snake.change_dir(Direction::Left),
            KeyCode::Left =>self.snake.change_dir(Direction::Right),
            KeyCode::Up  =>self.snake.change_dir(Direction::Down),
            KeyCode::Down =>self.snake.change_dir(Direction::Up),
            _ => (),
        }
       }
     }
      
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        y: f32
    ){

       if button==MouseButton::Left{ 
       if self.state==State::GameOver{
           if y>=((MAX_POS/2)*CELL_SIZE) as f32 && y<(((MAX_POS/5)*4)*CELL_SIZE) as f32{
              self.snake=Snake::new(Position::new(START_POS_X,START_POS_Y));
              self.foods=LinkedList::new();
              self.snake.add_normal(&mut self.foods);
              self.snake.add_normal(&mut self.foods);
              self.snake.add_killing(&mut self.foods);
              self.snake.add_poisonous(&mut self.foods);
              self.state=State::Playing;
           };
           if y>=(((MAX_POS/5)*4)*CELL_SIZE) as f32{
              self.state=State::Menu;
           };
       }

       else if self.state==State::Menu{
         if y< (SCREEN_SIZE_Y+EXTRA_SCREEN)/2.0{
            self.snake=Snake::new(Position::new(START_POS_X,START_POS_Y));
            self.foods=LinkedList::new();
            self.snake.add_normal(&mut self.foods);
            self.snake.add_normal(&mut self.foods);
            self.snake.add_killing(&mut self.foods);
            self.snake.add_poisonous(&mut self.foods);
            self.state=State::Difficulty;
         }
         else {
             self.state=State::Help
         }

       }

       else if self.state==State::Help{
        if y>SCREEN_SIZE_Y-(CELL_SIZE as f32){
            self.state=State::Menu;
         }
       }

       else if self.state==State::Difficulty{
           if y>(2*CELL_SIZE) as f32 && y<(5*CELL_SIZE) as f32{
                self.difficulty=Difficulty::Easy;
                self.state=State::Playing;
           }
           else if y>(7*CELL_SIZE) as f32 && y<(10*CELL_SIZE) as f32{
            self.difficulty=Difficulty::Medium;
            self.state=State::Playing;
           }
          else if y>(12*CELL_SIZE) as f32 && y<(15*CELL_SIZE) as f32{
           self.difficulty=Difficulty::Hard;
           self.state=State::Playing;
           }
           

       }
    }

        
    }

    


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        graphics::clear(ctx, dark_blue);
        if self.state==State::Help{
            self.draw_help(ctx)?;

        }
        if self.state==State::Menu{
         self.draw_menu(ctx)?; 
        }
        if self.state==State::GameOver {
            self.draw_game_over(ctx)?;
        }
        if self.state==State::Difficulty {
            self.draw_difficulty_menu(ctx)?;
        }
        if self.state==State::Playing{
        self.snake.draw(ctx)?;
        for f in &self.foods{
            f.draw(ctx,&self.assets)?;
        }

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0 as f32 , 
                0 as f32 , 
                SCREEN_SIZE_X, 
                EXTRA_SCREEN  ),
                [0.0,0.0,0.0,1.0].into())?;
        let draw_params = graphics::DrawParam::default().dest(Position::new(0,MAX_POS+1));
        graphics::draw(ctx, &rect, draw_params)?;
        let score= graphics::Text::new(format!("Score:{}",self.snake.points));
        graphics::draw(ctx,&score,draw_params)?;        
        graphics::present(ctx)?;
        return Ok(());}
        return Ok(());
    }




}




fn main() {
    let conf = Conf::new().
    window_mode(WindowMode {
        width: SCREEN_SIZE_X,
        height: SCREEN_SIZE_Y+EXTRA_SCREEN,
        ..Default::default()
    });
let (mut ctx, event_loop) = ContextBuilder::new("snake", "NS").
    default_conf(conf.clone()).
    build().
    unwrap();


if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    filesystem::mount(&mut ctx, &path, true);
}

let state = MainState::new(&mut ctx).unwrap();

event::run(ctx, event_loop, state);
}
