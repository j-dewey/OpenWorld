use crate::{physics::PhysicsObject, player::Player, world::World};

pub trait Entity: PhysicsObject{
    // amount is vec so that more information can be given
    // ex: if an entity can only move forward. a vec of [0]
    // is all that's needed
    fn r#move(&mut self, amount: Vec<f32>);
}

pub struct EntityList{
    player: Player
}

impl EntityList{
    pub fn new(scrn_width: u32, scrn_height: u32) -> Self{
        let player = Player::new(scrn_width, scrn_height);
        Self { 
            player 
        }
    }

    pub fn get_player_mut(&mut self) -> &mut Player{
        &mut self.player
    }

    pub fn update(&mut self, world: &World, dt: f32){
        if self.player.physics_on{
            world.check_col(&mut self.player)
        }
    }
}