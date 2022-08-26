use ggez::audio;
use ggez::graphics;

use ggez::{Context, GameResult};


pub struct Assets {
    pub burger:   graphics::Image,
    pub killing_burger:   graphics::Image,
    pub poisonous_burger:   graphics::Image,
    pub duner:   graphics::Image,
    pub killing_duner:   graphics::Image,
    pub poisonous_duner:   graphics::Image,
    pub antidote: graphics::Image,
    pub jackdaniels:   graphics::Image,
    pub steak:   graphics::Image,
    pub cactus:   graphics::Image,
    pub mushroom:   graphics::Image,
    pub pepper:   graphics::Image,
    pub protein:   graphics::Image,

    
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let burger   = graphics::Image::new(ctx, "/burger.png")?;
        let killing_burger   = graphics::Image::new(ctx, "/killingburger.png")?;
        let poisonous_burger   = graphics::Image::new(ctx, "/poisonousburger.png")?;
        let duner =  graphics::Image::new(ctx,"/duner.png")?;
        let killing_duner   = graphics::Image::new(ctx, "/killingduner.png")?;
        let poisonous_duner   = graphics::Image::new(ctx, "/poisonousduner.png")?;
        let antidote   = graphics::Image::new(ctx, "/antidote.png")?;
        let jackdaniels   = graphics::Image::new(ctx, "/jackdaniels.png")?;
        let steak   = graphics::Image::new(ctx, "/steak.png")?;
        let cactus =  graphics::Image::new(ctx,"/cactus.png")?;
        let mushroom   = graphics::Image::new(ctx, "/mushroom.png")?;
        let pepper   = graphics::Image::new(ctx, "/pepper.png")?;
        let protein   = graphics::Image::new(ctx, "/protein.png")?;
        Ok(Assets {
            burger,killing_burger,poisonous_burger,duner,killing_duner,poisonous_duner,
            antidote,jackdaniels,steak,cactus,mushroom,pepper,protein
            
        })
    }
}
pub struct Sounds {
    pub eating_sound: audio::Source,
    pub poisoned_sound: audio::Source,
    pub collision_sound: audio::Source,
    pub dying_sound: audio::Source,
}

impl Sounds {
    pub fn new(ctx: &mut Context) -> GameResult<Sounds> {

        let eating_sound = audio::Source::new(ctx, "/eating.mp3")?;
        let poisoned_sound=audio::Source::new(ctx, "/poisoned_sound.wav")?;
        let collision_sound = audio::Source::new(ctx, "/collision.wav")?;
        let dying_sound=audio::Source::new(ctx, "/dying.wav")?;

        Ok(Sounds {
            eating_sound,poisoned_sound,collision_sound,dying_sound,
        })
    }
}

