use specs::prelude::*;
use std::collections::HashSet;
use crate::components::{Initiative, Slow, Haste, MyTurn, Position, StatusEffect, Stun};
use crate::RunState;
use crate::effects::{EffectType, add_effect, Targets};
use crate::constants::DEFAULT_ACTION_COST;

pub struct EnergySystem {}

impl<'a> System<'a> for EnergySystem {
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
                        ReadStorage<'a, Stun>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut initiatives, positions, mut turns, entities,
            mut runstate, player,
            statuses, slowed, hasted, stunned) = data;

        if *runstate != RunState::Ticking { return; }

        // Clear any remaining MyTurn we left by mistkae
        turns.clear();
        
        // Collect a set of all entities whose turn it is
        let mut entity_turns = HashSet::new();

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

        // Give all entities their energy alottment
        for (entity, initiative, _pos) in (&entities, &mut initiatives, &positions).join() {
            let mut energy_gain = initiative.energy_gain;
            if slowed_entities.contains(&entity) {
                energy_gain = energy_gain / 2;
            }
            if hasted_entities.contains(&entity){
                energy_gain = energy_gain * 2;
            }
            initiative.current += energy_gain;
            if initiative.current >= 0 {
                turns.insert(entity, MyTurn{}).expect("Unable to insert turn");
                entity_turns.insert(entity);
            }
        }

        // Find status effects affecting entities whose turn it is
        for (entity, status_effect) in (&entities, &statuses).join() {
            if entity_turns.contains(&status_effect.target) {
                // Skip turn for stun.
                // We are checking the status effect entity to see if it has a stunned component
                if stunned.get(entity).is_some() {
                    add_effect(
                        None, 
                        EffectType::Particle{
                            glyph : rltk::to_cp437('?'),
                            fg : rltk::RGB::named(rltk::YELLOW),
                            bg : rltk::RGB::named(rltk::BLACK),
                            lifespan: 200.0
                        },
                        Targets::Single{ target: status_effect.target }
                    );
                    turns.remove(status_effect.target);

                    if let Some(initiative) = initiatives.get_mut(status_effect.target){
                        // If an entities' turn is skipped, we need to remove an action's worth of energy
                        // so the energy doesn't keep banking up and the entity can perform a double action
                        // in the future
                        initiative.current -= DEFAULT_ACTION_COST;
                    }
                }
            }
        }

        // Only assign state to RunState::AwaitingInput after we confirm the player isn't affected by a status effect
        if turns.get(*player).is_some() {
            *runstate = RunState::AwaitingInput;
        }
    }
}