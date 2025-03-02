use specs::prelude::*;
use std::collections::HashSet;
use crate::{Initiative, Slow, Haste, MyTurn, Position, RunState, StatusEffect, DEFAULT_ACTION_COST, MOVE_ACTION_COST, ATTACK_ACTION_COST};

pub struct InitiativeSystem {}

impl<'a> System<'a> for InitiativeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteStorage<'a, Initiative>,
                        ReadStorage<'a, Position>,
                        WriteStorage<'a, MyTurn>,
                        Entities<'a>,
                        WriteExpect<'a, RunState>,
                        ReadExpect<'a, Entity>,
                        ReadStorage<'a, StatusEffect>,
                        ReadStorage<'a, Slow>,
                        ReadStorage<'a, Haste>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut initiatives, positions, mut turns, entities,
            mut runstate, player,
            statuses, slowed, hasted) = data;

        if *runstate != RunState::Ticking { return; }

        // Clear any remaining MyTurn we left by mistkae
        turns.clear();

        let mut hasted_entities: HashSet<Entity> = HashSet::new();
        let mut slowed_entities: HashSet<Entity> = HashSet::new();
        for (status, haste, slow) in (&statuses, (&hasted).maybe(), (&slowed).maybe()).join() {
            if haste.is_some() {
                hasted_entities.insert(status.target);
            }
            if slow.is_some() {
                slowed_entities.insert(status.target);
            }
        }

        // Roll initiative
        for (entity, initiative, _pos) in (&entities, &mut initiatives, &positions).join() {
            let mut energy_gain = initiative.energy_gain;
            if slowed_entities.contains(&entity) {
                energy_gain = energy_gain / 2;
            }
            if hasted_entities.contains(&entity){
                energy_gain = energy_gain * 2;
            }
            initiative.current += energy_gain;
            // Several options here
            // 1. It's the players turn
            // 2. It's the enemies turn, but they are too far away, so don't process them
            // 3. It's the enemies turn and they are nearby
            if initiative.current >= 0 {
                // let mut myturn = true;

                // If its the player, we want to go to an AwaitingInput state
                if entity == *player {
                    // Give control to the player
                    *runstate = RunState::AwaitingInput;
                }
                //  else {
                //     let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, rltk::Point::new(pos.x, pos.y));
                //     if distance > 20.0 {
                //         myturn = false;
                //     }
                // }

                // It's my turn!
                // if myturn {
                turns.insert(entity, MyTurn{}).expect("Unable to insert turn");
                // }

            }
        }
    }
}

pub fn apply_move_action_cost(initiative: &mut Initiative) {
    // Convert the multiplier so...
    // - 50 takes 2x the action cost of the default 100. Ex 100 / .5 = 200
    // - 75 takes 1.33 the action cost of the default 100. Ex 100 / .75 = 133
    initiative.current -= (MOVE_ACTION_COST as f32 / initiative.move_action_mult).round() as i32;
}


pub fn apply_attack_action_cost(initiative: &mut Initiative) {
    // Convert the multiplier so...
    // - 50 takes 2x the action cost of the default 100. Ex 100 / .5 = 200
    // - 75 takes 1.33 the action cost of the default 100. Ex 100 / .75 = 133
    initiative.current -= (ATTACK_ACTION_COST as f32 / initiative.attack_action_mult).round() as i32;
}

pub fn apply_generic_action_cost(initiative: &mut Initiative) {
    initiative.current -= DEFAULT_ACTION_COST;
}