
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::Vector2;



use crate::assets::{Assets};
use crate::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind{
   Normalburger,
   Killingburger,
   Poisonousburger,
   Normalduner,
   Killingduner,
   Poisonousduner,
   Antidote,
   Jackdaniels,
   Steak,
   Cactus,
   Mushroom,
   Pepper,
   Protein,

}



pub struct Food{
    pub pos:Position,
    pub kind:Kind,
}


impl Food{
    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        let draw_params = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.075, y: 0.075 });
        let draw_params1 = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.0475, y: 0.0475 });
        let draw_params2 = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.0425, y: 0.03 });
        let draw_params3 = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.038, y: 0.0475 });
        let draw_params4 = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.012, y: 0.012 });
        let draw_params5 = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.12, y: 0.15 });
        match self.kind{
            Kind::Normalburger =>  graphics::draw(ctx, &assets.burger,draw_params)?,
            Kind::Killingburger =>  graphics::draw(ctx, &assets.killing_burger, draw_params)?,
            Kind::Poisonousburger =>  graphics::draw(ctx, &assets.poisonous_burger, draw_params)?,
            Kind::Normalduner =>  graphics::draw(ctx, &assets.duner,draw_params1)?,
            Kind::Killingduner =>  graphics::draw(ctx, &assets.killing_duner,draw_params1)?,
            Kind::Poisonousduner =>  graphics::draw(ctx, &assets.poisonous_duner,draw_params1)?,
            Kind::Antidote =>  graphics::draw(ctx, &assets.antidote,draw_params1)?,
            Kind::Jackdaniels  =>  graphics::draw(ctx, &assets.jackdaniels,draw_params2)?,
            Kind::Steak  =>  graphics::draw(ctx, &assets.steak,draw_params3)?,
            Kind::Cactus =>  graphics::draw(ctx, &assets.cactus,draw_params4)?,
            Kind::Mushroom =>  graphics::draw(ctx, &assets.mushroom,draw_params1)?,
            Kind::Pepper =>  graphics::draw(ctx, &assets.pepper,draw_params5)?,
            Kind::Protein =>  graphics::draw(ctx, &assets.protein,draw_params1)?,
        }
        
        
        Ok(())
    }

    pub fn new(p:Position,k:Kind)->Self{
      Food{pos:p,kind:k}
    }
}
