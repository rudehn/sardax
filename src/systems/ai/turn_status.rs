use specs::prelude::*;
use crate::{MyTurn, Paralysis, Initiative, RunState, StatusEffect, effects::add_effect, effects::EffectType, effects::Targets};
use std::collections::HashSet;

pub struct TurnStatusSystem {}

impl<'a> System<'a> for TurnStatusSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteStorage<'a, MyTurn>,
                        ReadStorage<'a, Paralysis>,
                        Entities<'a>,
                        ReadExpect<'a, RunState>,
                        ReadStorage<'a, StatusEffect>,
                        WriteStorage<'a, Initiative>
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut turns, paralysis, entities, runstate, statuses, initiatives) = data;

        if *runstate != RunState::Ticking { return; }

        // Collect a set of all entities whose turn it is
        let mut entity_turns = HashSet::new();
        for (entity, _turn) in (&entities, &turns).join() {
            entity_turns.insert(entity);
        }

        // Find status effects affecting entities whose turn it is
        let mut not_my_turn : Vec<Entity> = Vec::new();
        for (effect_entity, status_effect) in (&entities, &statuses).join() {
            if entity_turns.contains(&status_effect.target) {
                // Skip turn for paralysis
                if paralysis.get(effect_entity).is_some() {
                    add_effect(
                        None, 
                        EffectType::Particle{
                            glyph : rltk::to_cp437('?'),
                            fg : rltk::RGB::named(rltk::YELLOW),
                            bg : rltk::RGB::named(rltk::BLACK),
                            lifespan: 200.0
                        },
                        Targets::Single{ target:status_effect.target }
                    );
                    not_my_turn.push(status_effect.target);
                }
            }
        }

        for e in not_my_turn {
            turns.remove(e);
            let initiative = initiatives.get(e);
            // TODO - need to consume energy if the entity didn't do anything this turn
        }
    }
}
