use specs::prelude::*;
use std::collections::HashSet;
use crate::{Initiative, Slow, Haste, MyTurn, Position, RunState, StatusEffect};

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
            if initiative.current >= 0 {
                // If its the player, we want to go to an AwaitingInput state
                if entity == *player {
                    // Give control to the player
                    *runstate = RunState::AwaitingInput;
                }
                turns.insert(entity, MyTurn{}).expect("Unable to insert turn");
            }
        }
    }
}