use specs::prelude::*;
use crate::constants::TICKS_PER_TURN;
use crate::{RunState, GameStats, StatusEffect, Burning, Duration};

// Handles all processing/cleanup at the end of a turn

pub struct TurnEndSystem {}

impl<'a> System<'a> for TurnEndSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        ReadExpect<'a, RunState>,
                        WriteExpect<'a, GameStats>,
                        Entities<'a>,
                        WriteStorage<'a, Duration>,
                        ReadStorage<'a, StatusEffect>,
                        ReadStorage<'a, Burning>
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (runstate,mut game_stats, entities,
        mut durations, statuses, burning) = data;

        if *runstate != RunState::Ticking { return; }

        if game_stats.game_ticks % TICKS_PER_TURN == 0 {
            // Handle durations
            for (effect_entity, duration, status) in (&entities, &mut durations, (&statuses).maybe()).join() {
                // Status exists
                if let Some(status) = status{
                    if entities.is_alive(status.target) {
                        use crate::effects::*;
                        duration.turns -= 1;
                        if let Some(_burn) = burning.get(effect_entity) {
                            // Roll for burn damage
                            let burn_damage = crate::rng::roll_dice(1, 3);
                            add_effect(
                                None, 
                                EffectType::Damage{ amount : burn_damage }, 
                                Targets::Single{ target : status.target 
                                }
                            );
                        }
                        if duration.turns < 1 {
                            entities.delete(effect_entity).expect("Unable to delete");
                        }
                    }
                } else {
                    // Currently the only other flow to get here is fire effects on the map that have a duration
                    duration.turns -= 1;
                    if duration.turns < 1 {
                        entities.delete(effect_entity).expect("Unable to delete");
                    }
                }
            }

        }


        // Update # of turns that have been processed
        game_stats.game_ticks += 1;
    }
}