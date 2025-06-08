// #[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

// #[cfg(not(target_arch = "wasm32"))]
// #[macro_use]
// mod multi_thread;

// #[cfg(target_arch = "wasm32")]
pub use single_thread::*;

// #[cfg(not(target_arch = "wasm32"))]
// pub use multi_thread::*;

use specs::prelude::World;
use super::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs : *mut World);
}

construct_dispatcher!(
    (MapIndexingSystem, "map_index", &[]),
    (VisibilitySystem, "visibility", &[]),
    // (EncumbranceSystem, "encumbrance", &[]),
    (InitiativeSystem, "initiative", &[]),
    (TurnStatusSystem, "turnstatus", &[]),
    (QuipSystem, "quips", &[]),
    (AdjacentAI, "adjacent", &[]), // small drop
    (VisibleAI, "visible", &[]),
    (ApproachAI, "approach", &[]),
    (FleeAI, "flee", &[]),
    (ChaseAI, "chase", &[]),
    (DefaultMoveAI, "default_move", &[]), //huge slow down
    (MovementSystem, "movement", &[]),
    (TriggerSystem, "triggers", &[]),
    (CombatSystem, "combat", &[]),
    (ItemCollectionSystem, "pickup", &[]),
    (ItemEquipOnUse, "equip", &[]),
    (ItemUseSystem, "use", &[]),
    (SpellUseSystem, "spells", &[]),
    (ItemIdentificationSystem, "itemid", &[]),
    (ItemDropSystem, "drop", &[]),
    (ItemRemoveSystem, "remove", &[]),
    (HungerSystem, "hunger", &[]),
    (ParticleSpawnSystem, "particle_spawn", &[]),
    (LightingSystem, "lighting", &[]),
    (TurnEndSystem, "turn_end", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}