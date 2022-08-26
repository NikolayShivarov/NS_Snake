
use std::collections::VecDeque;
use std::collections::LinkedList;
use crate::position::Position;
use crate::food::{Food,Kind};
use crate::assets::Sounds;
use ggez::{Context, GameResult};
use ggez::graphics;
use rand::Rng;
use ggez::audio::SoundSource;
const CELL_SIZE: i32 = 30;
const MAX_POS: i32 = 15;
pub fn remove(l:&mut LinkedList<Food>,ind:usize){
let mut split_list = l.split_off(ind);
split_list.pop_front();
l.append(&mut split_list);
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl Direction{
    pub fn is_opposite(&self,  other:Direction) ->bool{
        match (self,other){
            (Direction::Down,Direction::Up) =>return true,
            (Direction::Up,Direction::Down) =>return true,
            (Direction::Left,Direction::Right) => return true,
            (Direction::Right,Direction::Left) => return true,
            (_,_) => return false,

        };
    }
}
pub struct Snake{
    pub body:VecDeque<Position>,
    pub is_alive: bool,
    pub points:i32,
    pub direction: Direction,
    pub is_poisoned: bool,
    length:u32,
    pub killed_by: String,
    pub is_drunk: bool,
    pub is_mushroomed:bool,
    pub bonus:u32,

}

impl Snake{
    pub fn new(pos: Position) -> Self{
        let mut v: VecDeque<Position> = VecDeque::new();
        v.push_back(Position::new(pos.x,pos.y));
        v.push_back(Position::new(pos.x-1,pos.y));
        v.push_back(Position::new(pos.x-2,pos.y));
        Snake{
            body:v,
            is_alive: true,
            points: 0,
            direction: Direction::Right,
            is_poisoned: false,
            length:3,
            killed_by: "".to_string(),
            is_drunk: false,
            is_mushroomed: false,
            bonus:0,
           
        }
    }
    
    pub fn head(&self) -> Position{
        return self.body[0];
    }
    pub fn kill(&mut self){
        self.is_alive=false;
    }

    pub fn change_dir(&mut self,dir:Direction){
         if dir==self.direction{return;};
         if self.direction.is_opposite(dir) {return;};
         self.direction=dir; 

    }

    pub fn add_point(&mut self){
        self.points+=1;
    }

    pub fn eats_food(&self,f:&Food) -> bool{
        if self.head()==f.pos {return true;};
        return false;
    }

    pub fn eats_itself(&self) -> bool{
        for i in 1..self.length as usize{
            if self.body[i]==self.head() {return true;};
        }
        return false;
    }

    pub fn poison(&mut self){
        self.is_poisoned=true;
    }

    pub fn heal(&mut self){
        self.is_poisoned=false;
        self.is_mushroomed=false;
    }

    pub fn drunk(&mut self){
        self.is_drunk=true;
    }
    
    pub fn sober(&mut self){
        self.is_drunk=false;
    }
    pub fn mushroom(&mut self){
        self.is_mushroomed=true;
    }

    
    pub fn movement(&mut self) {
		let Position{x, y} = self.head();
		self.body.push_front( match self.direction {
			Direction::Up => Position::new(x, y - 1),
			Direction::Down => Position::new(x, y + 1),
			Direction::Left => Position::new(x - 1, y),
			Direction::Right => Position::new(x + 1, y),
		});
		self.body.pop_back();
		assert_eq!(self.length as usize, self.body.len());
	}

	pub fn grow(&mut self) {
		let Position{x, y} = self.head();
		self.body.push_front( match self.direction {
			Direction::Up => Position::new(x, y - 1),
			Direction::Down => Position::new(x, y + 1),
			Direction::Left => Position::new(x - 1, y),
			Direction::Right => Position::new(x + 1, y),
		});
		self.length += 1;
		assert_eq!(self.length as usize, self.body.len());
	}

    pub fn shrink(&mut self) {
		let Position{x, y} = self.head();
		self.body.push_front( match self.direction {
			Direction::Up => Position::new(x, y - 1),
			Direction::Down => Position::new(x, y + 1),
			Direction::Left => Position::new(x - 1, y),
			Direction::Right => Position::new(x + 1, y),
		});
		self.body.pop_back();
        self.body.pop_back();
        self.length-=1;
		assert_eq!(self.length as usize, self.body.len());
	}
    pub fn check_free(&self,l: &LinkedList<Food>,p:Position) -> bool{
      let mut flag= true;
      let x=self.head().x;
      let y=self.head().y;
      let xp=p.x;
      let yp=p.y;
      if (x==xp && y==yp-1) || (x==xp && y==yp+1) || (x==xp-1 && y==yp) || (x==xp+1 && y==yp){
          flag=false;
      }
      for part in &self.body{
          if part.clone()==p {flag=false;};
      }
      for f in l{
          if f.pos==p {flag=false;};
      }
      return flag;

    }
    pub fn generate_position(&self,l: &mut LinkedList<Food>) ->Position{
        let mut flag=true;
        let mut p=Position::new(0,0);
        while flag{
         p=Position::random(MAX_POS,MAX_POS);
         if self.check_free(l, p) {flag=false;};
        }
        p
    }

    pub fn add_normal(&self,l: &mut LinkedList<Food>){
        let mut rng = rand::thread_rng();
        let n=rng.gen_range(0.0..1.0);
        let p=self.generate_position(l);
        if n<=0.5 {
            l.push_back(Food::new(p,Kind::Normalburger))
        }
        else {
            l.push_back(Food::new(p,Kind::Normalduner))
        }

    }
    pub fn add_poisonous(&self,l: &mut LinkedList<Food>){
        let mut rng = rand::thread_rng();
        let n=rng.gen_range(0.0..1.0);
        let p=self.generate_position(l);
        if n<=0.5 {
            l.push_back(Food::new(p,Kind::Poisonousburger))
        }
        else {
            l.push_back(Food::new(p,Kind::Poisonousduner))
        }

    }
    
    pub fn add_killing(&self,l: &mut LinkedList<Food>){
        let mut rng = rand::thread_rng();
        let n=rng.gen_range(0.0..1.0);
        let p=self.generate_position(l);
        if n<=0.5 {
            l.push_back(Food::new(p,Kind::Killingburger))
        }
        else {
            l.push_back(Food::new(p,Kind::Killingduner))
        }

    }

    pub fn add_antidote(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Antidote));  
    }
    pub fn add_pepper(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Pepper));  
    }
    pub fn add_alchohol(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Jackdaniels));  
    }
    pub fn add_steak(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Steak));  
    }
    pub fn add_cactus(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Cactus));  
    }
    pub fn add_mushroom(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Mushroom));  
    }
    pub fn add_protein(&self,l: &mut LinkedList<Food>){
        let p=self.generate_position(l);
        l.push_back(Food::new(p,Kind::Protein));  
    }


    pub fn remove_point(&mut self){
      self.points-=1;
    }

    pub fn bonus(&mut self){
        let mut rng = rand::thread_rng();
        let n=rng.gen_range(30..70);
        self.bonus+=n;
    }


    pub fn draw(&self, ctx: &mut Context) -> GameResult<()>{
        
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0 as f32 , 
                0 as f32 , 
                (CELL_SIZE-1) as f32  , 
                (CELL_SIZE-1) as f32  ),
                [0.0,128.0,0.0,1.0].into())?;
        if !self.is_mushroomed {
            for part in self.body.iter(){
            
            let draw_params = graphics::DrawParam::default().dest(part.clone());
                
                graphics::draw(ctx, &rect, draw_params)?;
            
        }
      
    }

    else{
        for (i, part) in self.body.iter().enumerate(){
            if (i as u32) <= self.length/2  {
            let draw_params = graphics::DrawParam::default().dest(part.clone());
                
                graphics::draw(ctx, &rect, draw_params)?;
            }
        }

    }
        Ok(())
    }

    pub fn update(&mut self, foods: &mut LinkedList<Food> ,ctx: &mut Context, sounds: &mut Sounds) {
        let mut eats=false;
        let mut ind=0;
        let mut case=0;
        for (i, f) in foods.iter().enumerate(){
            if self.eats_food(f) {eats=true;
            ind=i;
            self.add_point();
            if self.bonus>0{
                self.add_point();
            }
        match f.kind{
            Kind::Normalburger =>  {case=1;},
            Kind::Killingburger =>  {let _ = sounds.dying_sound.play(ctx);self.kill();self.killed_by="Killed by KillingBurger".to_string();},
            Kind::Poisonousburger =>  {self.poison();case=2;},
            Kind::Normalduner =>  {case=1;},
            Kind::Killingduner =>  {let _ =sounds.dying_sound.play(ctx);self.kill();self.killed_by="Killed by KillingDuner".to_string();},
            Kind::Poisonousduner =>  {self.poison();case=2;},
            Kind::Antidote =>  {self.heal();case=3;},
            Kind::Jackdaniels  =>  {self.drunk();case=4},
            Kind::Steak  =>  {self.add_point();self.add_point();case=7;},
            Kind::Cactus =>  {self.remove_point();self.remove_point();self.remove_point();case=5;},
            Kind::Mushroom =>  {self.mushroom();case=6},
            Kind::Pepper => {self.sober();case=7},
            Kind::Protein => {case=8;} ,
        };
            
            }
        }
       if eats {
        remove(foods,ind);
        let _ = sounds.eating_sound.play(ctx);
        if case==1 {self.grow();self.add_normal(foods);};
        if case==2 {self.shrink();self.add_antidote(foods);};
        if case==3 {self.grow();};
        if case==4 {self.movement();self.add_pepper(foods);};
        if case==5 {self.movement();};
        if case==6 {self.movement();self.add_antidote(foods);};
        if case==7 {self.movement();};
        if case==8 {self.movement();self.bonus();};
        

      }

      if self.length<=2{
        let _ = sounds.dying_sound.play(ctx);
        self.kill();
        self.killed_by="Killed by Poison".to_string();
       }
       if !eats {
           if !self.is_poisoned {self.movement();}
           else {
               self.shrink();
               let _ = sounds.poisoned_sound.play(ctx);
             }
        }
        

       if self.bonus>0{
           self.bonus-=1;
       } 
       if  self.eats_itself(){
        let _ = sounds.collision_sound.play(ctx);
           self.kill();
           self.killed_by="Killed by Yourself".to_string();
       }
       if self.head().out_of_bounds() {
        let _ = sounds.collision_sound.play(ctx);
        self.kill();
        self.killed_by="Killed by Wall".to_string();
    }
       let mut rng = rand::thread_rng();
        let n=rng.gen_range(0..310);
        if n>145 && n<148 {self.add_poisonous(foods);};
        if n>=148 && n<=149 {self.add_killing(foods);};
        if n==150 {self.add_protein(foods);};
        if n==151 || n==152 {self.add_mushroom(foods);};
        if n==153 || n==154 {self.add_steak(foods);};
        if n==155  {self.add_protein(foods);};
        if n==156 || n==157 {self.add_alchohol(foods);};
        if n==158 || n==159 {self.add_cactus(foods);};
    }


}