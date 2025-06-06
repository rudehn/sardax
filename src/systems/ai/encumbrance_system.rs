// use specs::prelude::*;Add commentMore actions
// use crate::{EquipmentChanged, Item, InBackpack, Equipped, Pools, StatusEffect, Slow};
// use std::collections::HashMap;

// pub struct EncumbranceSystem {}

// impl<'a> System<'a> for EncumbranceSystem {
//     #[allow(clippy::type_complexity)]
//     type SystemData = (
//         WriteStorage<'a, EquipmentChanged>,
//         Entities<'a>,
//         ReadStorage<'a, Item>,
//         ReadStorage<'a, InBackpack>,
//         ReadStorage<'a, Equipped>,
//         WriteStorage<'a, Pools>,
//         ReadStorage<'a, StatusEffect>,
//         ReadStorage<'a, Slow>
//     );

//     fn run(&mut self, data : Self::SystemData) {
//         let (mut equip_dirty, entities, items, backpacks, wielded,
//             mut pools, statuses, slowed) = data;

//         if equip_dirty.is_empty() { return; }

//         struct ItemUpdate {
//             weight : f32,
//             initiative : f32,
//             strength : i32,
//             constitution : i32,
//             dexterity : i32,
//             intelligence : i32
//         }

//         // Build the map of who needs updating
//         let mut to_update : HashMap<Entity, ItemUpdate> = HashMap::new(); // (weight, intiative)
//         for (entity, _dirty) in (&entities, &equip_dirty).join() {
//             to_update.insert(entity, ItemUpdate{ weight: 0.0, initiative: 0.0, strength: 0, constitution: 0, dexterity: 0, intelligence: 0 });
//         }

//         // Remove all dirty statements
//         equip_dirty.clear();

//         // Total up equipped items

//         for (item, equipped, entity) in (&items, &wielded, &entities).join() {
//             if to_update.contains_key(&equipped.owner) {
//                 let totals = to_update.get_mut(&equipped.owner).unwrap();
//                 totals.weight += item.weight_lbs;
//                 totals.initiative += item.initiative_penalty;
//             }
//         }

//         // Total up carried items
//         for (item, carried) in (&items, &backpacks).join() {
//             if to_update.contains_key(&carried.owner) {
//                 let totals = to_update.get_mut(&carried.owner).unwrap();
//                 totals.weight += item.weight_lbs;
//                 totals.initiative += item.initiative_penalty;
//             }
//         }

//         // Total up haste/slow
//         for (status, slow) in (&statuses, &slowed).join() {
//             if to_update.contains_key(&status.target) {
//                 let totals = to_update.get_mut(&status.target).unwrap();
//                 totals.initiative += slow.initiative_penalty;
//             }
//         }

//         // Apply the data to Pools
//         for (entity, item) in to_update.iter() {
//             if let Some(pool) = pools.get_mut(*entity) {
//                 pool.total_weight = item.weight;
//                 pool.total_initiative_penalty = item.initiative;
//             }
//         }
//     }
// }