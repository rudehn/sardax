use std::collections::{HashMap, HashSet};
use specs::prelude::*;
use crate::components::*;
use crate::attr_bonus;
use super::{Raws, faction_structs::Reaction};
use crate::random_table::{MasterTable, RandomTable};
use crate::components::EffectValues;
use crate::DEFAULT_ENERGY_GAIN;
use regex::Regex;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn parse_dice_string(dice : &str) -> (i32, i32, i32) {
    lazy_static! {
        static ref DICE_RE : Regex = Regex::new(r"(\d+)d(\d+)([\+\-]\d+)?").unwrap();
    }
    let mut n_dice = 1;
    let mut die_type = 4;
    let mut die_bonus = 0;
    for cap in DICE_RE.captures_iter(dice) {
        if let Some(group) = cap.get(1) {
            n_dice = group.as_str().parse::<i32>().expect("Not a digit");
        }
        if let Some(group) = cap.get(2) {
            die_type = group.as_str().parse::<i32>().expect("Not a digit");
        }
        if let Some(group) = cap.get(3) {
            die_bonus = group.as_str().parse::<i32>().expect("Not a digit");
        }

    }
    (n_dice, die_type, die_bonus)
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpawnType {
    AtPosition { x: i32, y: i32 },
    Equipped { by: Entity },
    Carried { by: Entity }
}

pub struct RawMaster {
    raws : Raws,
    item_index : HashMap<String, usize>,
    mob_index : HashMap<String, usize>,
    prop_index : HashMap<String, usize>,
    loot_index : HashMap<String, usize>,
    faction_index : HashMap<String, HashMap<String, Reaction>>,
    spell_index : HashMap<String, usize>
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws : Raws{
                items: Vec::new(),
                mobs: Vec::new(),
                props: Vec::new(),
                spawn_table: Vec::new(),
                loot_tables: Vec::new(),
                faction_table : Vec::new(),
                spells : Vec::new(),
            },
            item_index : HashMap::new(),
            mob_index : HashMap::new(),
            prop_index : HashMap::new(),
            loot_index : HashMap::new(),
            faction_index : HashMap::new(),
            spell_index : HashMap::new()
        }
    }

    pub fn load(&mut self, raws : Raws) {
        self.raws = raws;
        self.item_index = HashMap::new();
        let mut used_names : HashSet<String> = HashSet::new();

        for (i,item) in self.raws.items.iter().enumerate() {
            if used_names.contains(&item.name) {
                rltk::console::log(format!("WARNING -  duplicate item name in raws [{}]", item.name));
            }
            self.item_index.insert(item.name.clone(), i);
            used_names.insert(item.name.clone());
        }
        for (i,mob) in self.raws.mobs.iter().enumerate() {
            if used_names.contains(&mob.name) {
                rltk::console::log(format!("WARNING -  duplicate mob name in raws [{}]", mob.name));
            }
            self.mob_index.insert(mob.name.clone(), i);
            used_names.insert(mob.name.clone());
        }
        for (i,prop) in self.raws.props.iter().enumerate() {
            if used_names.contains(&prop.name) {
                rltk::console::log(format!("WARNING -  duplicate prop name in raws [{}]", prop.name));
            }
            self.prop_index.insert(prop.name.clone(), i);
            used_names.insert(prop.name.clone());
        }

        for spawn in self.raws.spawn_table.iter() {
            if !used_names.contains(&spawn.name) {
                rltk::console::log(format!("WARNING - Spawn tables references unspecified entity {}", spawn.name));
            }
        }

        for (i,loot) in self.raws.loot_tables.iter().enumerate() {
            self.loot_index.insert(loot.name.clone(), i);
        }

        for faction in self.raws.faction_table.iter() {
            let mut reactions : HashMap<String, Reaction> = HashMap::new();
            for other in faction.responses.iter() {
                reactions.insert(
                    other.0.clone(),
                    match other.1.as_str() {
                        "ignore" => Reaction::Ignore,
                        "flee" => Reaction::Flee,
                        _ => Reaction::Attack
                    }
                );
            }
            self.faction_index.insert(faction.name.clone(), reactions);
        }

        for (i,spell) in self.raws.spells.iter().enumerate() {
            self.spell_index.insert(spell.name.clone(), i);
        }
    }
}

#[inline(always)]
pub fn faction_reaction(my_faction : &str, their_faction : &str, raws : &RawMaster) -> Reaction {
    //println!("Looking for reaction to [{}] by [{}]", my_faction, their_faction);
    if raws.faction_index.contains_key(my_faction) {
        let mf = &raws.faction_index[my_faction];
        if mf.contains_key(their_faction) {
            //println!("  :  {:?}", mf[their_faction]);
            return mf[their_faction];
        } else if mf.contains_key("Default") {
            //println!("  :  {:?}", mf["Default"]);
            return mf["Default"];
        } else {
            //println!("   : IGNORE");
            return Reaction::Ignore;
        }
    }
    //println!("   : IGNORE");
    Reaction::Ignore
}

fn find_slot_for_equippable_item(tag : &str, raws: &RawMaster) -> EquipmentSlot {
    if !raws.item_index.contains_key(tag) {
        panic!("Trying to equip an unknown item: {}", tag);
    }
    let item_index = raws.item_index[tag];
    let item = &raws.raws.items[item_index];
    if let Some(_wpn) = &item.weapon {
        return EquipmentSlot::Melee;
    } else if let Some(wearable) = &item.wearable {
        return string_to_slot(&wearable.slot);
    }
    panic!("Trying to equip {}, but it has no slot tag.", tag);
}

pub fn get_vendor_items(categories: &[String], raws : &RawMaster) -> Vec<(String, f32)> {
    let mut result : Vec<(String, f32)> = Vec::new();

    for item in raws.raws.items.iter() {
        if let Some(cat) = &item.vendor_category {
            if categories.contains(cat) && item.base_value.is_some() {
                result.push((
                    item.name.clone(),
                    item.base_value.unwrap()
                ));
            }
        }
    }

    result
}

pub fn get_scroll_tags() -> Vec<String> {
    let raws = &super::RAWS.lock().unwrap();
    let mut result = Vec::new();

    for item in raws.raws.items.iter() {
        if let Some(magic) = &item.magic {
            if &magic.naming == "scroll" {
                result.push(item.name.clone());
            }
        }
    }

    result
}

pub fn get_potion_tags() -> Vec<String> {
    let raws = &super::RAWS.lock().unwrap();
    let mut result = Vec::new();

    for item in raws.raws.items.iter() {
        if let Some(magic) = &item.magic {
            if &magic.naming == "potion" {
                result.push(item.name.clone());
            }
        }
    }

    result
}

pub fn is_tag_magic(tag : &str) -> bool {
    let raws = &super::RAWS.lock().unwrap();
    if raws.item_index.contains_key(tag) {
        let item_template = &raws.raws.items[raws.item_index[tag]];
        item_template.magic.is_some()
    } else {
        false
    }
}

fn spawn_position<'a>(pos : SpawnType, new_entity : EntityBuilder<'a>, tag : &str, raws: &RawMaster) -> EntityBuilder<'a> {
    let eb = new_entity;

    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition{x,y} => eb.with(Position{ x, y }),
        SpawnType::Carried{by} => eb.with(InBackpack{ owner: by }),
        SpawnType::Equipped{by} => {
            let slot = find_slot_for_equippable_item(tag, raws);
            eb.with(Equipped{ owner: by, slot })
        }
    }
}

fn get_renderable_component(renderable : &super::item_structs::Renderable) -> crate::components::Renderable {
    let mut bg: Option<rltk::RGB> = None;
    if let Some(render_bg) = &renderable.bg{
        bg = Some(rltk::RGB::from_hex(render_bg).expect("Invalid RGB"));
    }
    crate::components::Renderable{
        glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
        fg : rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
        bg : bg,
        render_order : renderable.order
    }
}

pub fn string_to_slot(slot : &str) -> EquipmentSlot {
    match slot {
        "Shield" => EquipmentSlot::Shield,
        "Head" => EquipmentSlot::Head,
        "Torso" => EquipmentSlot::Torso,
        "Legs" => EquipmentSlot::Legs,
        "Feet" => EquipmentSlot::Feet,
        "Hands" => EquipmentSlot::Hands,
        "Melee" => EquipmentSlot::Melee,
        _ => { rltk::console::log(format!("Warning: unknown equipment slot type [{}])", slot)); EquipmentSlot::Melee }
    }
}

fn parse_particle_line(n : &str) -> SpawnParticleLine {
    let tokens : Vec<_> = n.split(';').collect();
    SpawnParticleLine{
        glyph : rltk::to_cp437(tokens[0].chars().next().unwrap()),
        color : rltk::RGB::from_hex(tokens[1]).expect("Bad RGB"),
        lifetime_ms : tokens[2].parse::<f32>().unwrap()
    }
}

fn parse_particle(n : &str) -> SpawnParticleBurst {
    let tokens : Vec<_> = n.split(';').collect();
    SpawnParticleBurst{
        glyph : rltk::to_cp437(tokens[0].chars().next().unwrap()),
        color : rltk::RGB::from_hex(tokens[1]).expect("Bad RGB"),
        lifetime_ms : tokens[2].parse::<f32>().unwrap()
    }
}

macro_rules! apply_effects {
    ( $effects:expr, $eb:expr ) => {
        for effect in $effects.iter() {
        let effect_name = effect.0.as_str();
            match effect_name {
                "provides_healing" =>  $eb = $eb.with(ProvidesHealing{ heal_amount: effect.1.amount.unwrap() }),
                "provides_mana" => $eb = $eb.with(ProvidesMana{ mana_amount: effect.1.amount.unwrap() }),
                "teach_spell" => $eb = $eb.with(TeachesSpell{ spell: effect.1.value.as_ref().unwrap().to_string() }),
                "ranged" => $eb = $eb.with(Ranged{ range: effect.1.amount.unwrap() }),
                "damage" => $eb = $eb.with(InflictsDamage{ damage : effect.1.amount.unwrap() }),
                "area_of_effect" => $eb = $eb.with(AreaOfEffect{ radius: effect.1.amount.unwrap() }),
                "paralysis" => $eb = $eb.with(InflictsParalysis{ turns: effect.1.duration.unwrap() }),
                "burning" => $eb = $eb.with(InflictsBurning{turns: 7}),
                "tunneling" => $eb = $eb.with(CreatesTunnel{}),
                "duration" => $eb = $eb.with(Duration{turns: effect.1.amount.unwrap(), total_turns: effect.1.amount.unwrap() }),
                "magic_mapping" => $eb = $eb.with(MagicMapper{}),
                "town_portal" => $eb = $eb.with(TownPortal{}),
                "food" => $eb = $eb.with(ProvidesFood{}),
                "single_activation" => $eb = $eb.with(SingleActivation{}),
                "particle_line" => $eb = $eb.with(parse_particle_line(effect.1.value.as_ref().unwrap())),
                "particle" => $eb = $eb.with(parse_particle(effect.1.value.as_ref().unwrap())),
                "remove_curse" => $eb = $eb.with(ProvidesRemoveCurse{}),
                "identify" => $eb = $eb.with(ProvidesIdentification{}),
                "slow" => $eb = $eb.with(Slow{}),
                "haste" => $eb = $eb.with(Haste{}),
                "target_self" => $eb = $eb.with( AlwaysTargetsSelf{} ),
                _ => rltk::console::log(format!("Warning: consumable effect {} not implemented.", effect_name))
            }
        }
    };
}

pub fn spawn_named_item(raws: &RawMaster, ecs : &mut World, key : &str, pos : SpawnType) -> Option<Entity> {
    if raws.item_index.contains_key(key) {
        let item_template = &raws.raws.items[raws.item_index[key]];

        let dm = ecs.fetch::<crate::map::MasterDungeonMap>();
        let scroll_names = dm.scroll_mappings.clone();
        let potion_names = dm.potion_mappings.clone();
        let identified = dm.identified_items.clone();
        std::mem::drop(dm);
        let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();

        // Spawn in the specified location
        eb = spawn_position(pos, eb, key, raws);

        // Renderable
        if let Some(renderable) = &item_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
            if renderable.x_size.is_some() || renderable.y_size.is_some() {
                eb = eb.with(TileSize{ x : renderable.x_size.unwrap_or(1), y : renderable.y_size.unwrap_or(1) });
            }
        }

        eb = eb.with(Name{ name : item_template.name.clone() });

        eb = eb.with(crate::components::Item{
            weight_lbs : item_template.weight_lbs.unwrap_or(0.0),
            base_value : item_template.base_value.unwrap_or(0.0)
        });

        if let Some(consumable) = &item_template.consumable {
            let max_charges = consumable.charges.unwrap_or(1);
            eb = eb.with(crate::components::Consumable{ max_charges, charges : max_charges });
            apply_effects!(consumable.effects, eb);
        }

        if let Some(weapon) = &item_template.weapon {
            eb = eb.with(Equippable{ slot: EquipmentSlot::Melee });
            let (n_dice, die_type, bonus) = parse_dice_string(&weapon.base_damage);
            let wpn = Weapon{
                range : if weapon.range == "melee" { None } else { Some(weapon.range.parse::<i32>().expect("Not a number")) },
                damage_n_dice : n_dice,
                damage_die_type : die_type,
                damage_bonus : bonus,
                hit_bonus : weapon.hit_bonus,
                proc_chance : weapon.proc_chance,
                proc_target : weapon.proc_target.clone()
            };
            eb = eb.with(wpn);
            if let Some(proc_effects) =& weapon.proc_effects {
                apply_effects!(proc_effects, eb);
            }
        }

        if let Some(wearable) = &item_template.wearable {
            let slot = string_to_slot(&wearable.slot);
            eb = eb.with(Equippable{ slot });
            eb = eb.with(Wearable{ slot, armor_class: wearable.armor_class });
        }

        if let Some(magic) = &item_template.magic {
            let class = match magic.class.as_str() {
                "uncommon" => MagicItemClass::Uncommon,
                "rare" => MagicItemClass::Rare,
                "legendary" => MagicItemClass::Legendary,
                _ => MagicItemClass::Common
            };
            eb = eb.with(MagicItem{ class });

            if !identified.contains(&item_template.name) {
                match magic.naming.as_str() {
                    "scroll" => {
                        eb = eb.with(ObfuscatedName{ name : scroll_names[&item_template.name].clone() });
                    }
                    "potion" => {
                        eb = eb.with(ObfuscatedName{ name: potion_names[&item_template.name].clone() });
                    }
                    _ => {
                        eb = eb.with(ObfuscatedName{ name : magic.naming.clone() });
                    }
                }
            }

            if let Some(cursed) = magic.cursed {
                if cursed { eb = eb.with(CursedItem{}); }
            }
        }

        if let Some(ab) = &item_template.attributes {
            eb = eb.with(AttributeBonus{
                strength : ab.strength,
                // constitution : ab.constitution,
                // dexterity : ab.dexterity,
                // intelligence : ab.intelligence,
            });
        }

        return Some(eb.build());
    }
    None
}

#[allow(clippy::cognitive_complexity)]
pub fn spawn_named_mob(raws: &RawMaster, ecs : &mut World, key : &str, pos : SpawnType) -> Option<Entity> {
    if raws.mob_index.contains_key(key) {
        let mob_template = &raws.raws.mobs[raws.mob_index[key]];

        let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();

        // Spawn in the specified location
        eb = spawn_position(pos, eb, key, raws);

        // Initiative of -2 so they move behind the player on turn 1
        let mut initiative = Initiative{
            energy_gain: DEFAULT_ENERGY_GAIN,
            attack_action_mult: 1.0,
            move_action_mult: 1.0,
            current:-2
        };
        if let Some(energy) = mob_template.attributes.energy {
            initiative.energy_gain = energy;
        }
        // if let Some(attack_cost) = mob_template.attributes.attack_action_mult {
        //     initiative.attack_action_mult = attack_cost;
        // }
        // if let Some(move_cost) = mob_template.attributes.move_action_mult {
        //     initiative.move_action_mult = move_cost;
        // }
        eb = eb.with(initiative);

        // Renderable
        if let Some(renderable) = &mob_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
            if renderable.x_size.is_some() || renderable.y_size.is_some() {
                eb = eb.with(TileSize{ x : renderable.x_size.unwrap_or(1), y : renderable.y_size.unwrap_or(1) });
            }
        }

        eb = eb.with(Name{ name : mob_template.name.clone() });

        match mob_template.movement.as_ref() {
            "random" => eb = eb.with(MoveMode{ mode: Movement::Random }),
            "random_waypoint" => eb = eb.with(MoveMode{ mode: Movement::RandomWaypoint{ path: None } }),
            _ => eb = eb.with(MoveMode{ mode: Movement::Static })
        }

        if let Some(quips) = &mob_template.quips {
            eb = eb.with(Quips{
                available: quips.clone()
            });
        }

        if mob_template.blocks_tile {
            eb = eb.with(BlocksTile{});
        }

        let mut attr = Attributes{
            strength: Attribute { base: 10, modifiers: 0, bonus: attr_bonus(10) }
        };
        if let Some(strength) = mob_template.attributes.strength {
            attr.strength = Attribute{ base: strength, modifiers: 0, bonus: attr_bonus(strength) };;
        }
    
        eb = eb.with(attr);

        let (n_dice, die_type, bonus) = parse_dice_string(&mob_template.health);
        let base_health = crate::rng::roll_dice(n_dice, die_type);
        let mob_hp = base_health + bonus;

        let pools = Pools{
            hit_points : Pool{ current: mob_hp, max: mob_hp },
            mana: Pool{current: 0, max: 0},
            total_weight : 0.0,
            gold : if let Some(gold) = &mob_template.gold {
                    let (n, d, b) = parse_dice_string(&gold);
                    (crate::rng::roll_dice(n, d) + b) as f32
                } else {
                    0.0
                },
            god_mode : false
        };
        eb = eb.with(pools);
        eb = eb.with(EquipmentChanged{});

        eb = eb.with(Viewshed{ visible_tiles : Vec::new(), range: mob_template.vision_range, dirty: true });

        if let Some(na) = &mob_template.natural {
            let mut nature = NaturalAttackDefense{
                armor_class : na.armor_class,
                attacks: Vec::new()
            };
            if let Some(attacks) = &na.attacks {
                for nattack in attacks.iter() {
                    let (n, d, b) = parse_dice_string(&nattack.damage);
                    let attack = NaturalAttack{
                        name : nattack.name.clone(),
                        hit_bonus : nattack.hit_bonus,
                        damage_n_dice : n,
                        damage_die_type : d,
                        damage_bonus: b,
                        effect: nattack.on_hit.clone()
                    };
                    nature.attacks.push(attack);
                }
            }
    
            eb = eb.with(nature);
        }

        if let Some(loot) = &mob_template.loot_table {
            eb = eb.with(LootTable{table: loot.clone()});
        }

        if let Some(light) = &mob_template.light {
            eb = eb.with(LightSource{ range: light.range, color : rltk::RGB::from_hex(&light.color).expect("Bad color") });
        }

        if let Some(faction) = &mob_template.faction {
            eb = eb.with(Faction{ name: faction.clone() });
        } else {
            eb = eb.with(Faction{ name : "Mindless".to_string() })
        }

        if let Some(vendor) = &mob_template.vendor {
            eb = eb.with(Vendor{ categories : vendor.clone() });
        }

        if let Some(ability_list) = &mob_template.abilities {
            let mut a = SpecialAbilities { abilities : Vec::new() };
            for ability in ability_list.iter() {
                a.abilities.push(
                    SpecialAbility{
                        chance : ability.chance,
                        spell : ability.spell.clone(),
                        range : ability.range,
                        min_range : ability.min_range
                    }
                );
            }
            eb = eb.with(a);
        }

        if let Some(ability_list) = &mob_template.on_death {
            let mut a = OnDeath{ abilities : Vec::new() };
            for ability in ability_list.iter() {
                a.abilities.push(
                    SpecialAbility{
                        chance : ability.chance,
                        spell : ability.spell.clone(),
                        range : ability.range,
                        min_range : ability.min_range
                    }
                );
            }
            eb = eb.with(a);
        }

        let new_mob = eb.build();

        // Are they wielding anyting?
        if let Some(wielding) = &mob_template.equipped {
            for tag in wielding.iter() {
                spawn_named_entity(raws, ecs, tag, SpawnType::Equipped{ by: new_mob });
            }
        }

        return Some(new_mob);
    }
    None
}

pub fn spawn_natural_attack(ecs : &mut World, effects : HashMap<String, EffectValues>) -> Entity {

    let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();
    apply_effects!(effects, eb);
    return eb.build();
}

pub fn spawn_named_prop(raws: &RawMaster, ecs : &mut World, key : &str, pos : SpawnType) -> Option<Entity> {
    if raws.prop_index.contains_key(key) {
        let prop_template = &raws.raws.props[raws.prop_index[key]];

        let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();

        // Spawn in the specified location
        eb = spawn_position(pos, eb, key, raws);

        // Renderable
        if let Some(renderable) = &prop_template.renderable {
            eb = eb.with(get_renderable_component(renderable));
            if renderable.x_size.is_some() || renderable.y_size.is_some() {
                eb = eb.with(TileSize{ x : renderable.x_size.unwrap_or(1), y : renderable.y_size.unwrap_or(1) });
            }
        }

        eb = eb.with(Name{ name : prop_template.name.clone() });

        if let Some(hidden) = prop_template.hidden {
            if hidden { eb = eb.with(Hidden{}) };
        }
        if let Some(blocks_tile) = prop_template.blocks_tile {
            if blocks_tile { eb = eb.with(BlocksTile{}) };
        }
        if let Some(blocks_visibility) = prop_template.blocks_visibility {
            if blocks_visibility { eb = eb.with(BlocksVisibility{}) };
        }
        if let Some(door_open) = prop_template.door_open {
            eb = eb.with(Door{ open: door_open });
        }
        if let Some(entry_trigger) = &prop_template.entry_trigger {
            eb = eb.with(EntryTrigger{});
            apply_effects!(entry_trigger.effects, eb);
        }
        if let Some(light) = &prop_template.light {
            eb = eb.with(LightSource{ range: light.range, color : rltk::RGB::from_hex(&light.color).expect("Bad color") });
            eb = eb.with(Viewshed{ range: light.range, dirty: true, visible_tiles: Vec::new() });
        }


        return Some(eb.build());
    }
    None
}

pub fn spawn_named_spell(raws: &RawMaster, ecs : &mut World, key : &str) -> Option<Entity> {
    if raws.spell_index.contains_key(key) {
        let spell_template = &raws.raws.spells[raws.spell_index[key]];

        let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();
        eb = eb.with(SpellTemplate{ mana_cost : spell_template.mana_cost });
        eb = eb.with(Name{ name : spell_template.name.clone() });
        apply_effects!(spell_template.effects, eb);

        return Some(eb.build());
    }
    None
}

pub fn spawn_all_spells(ecs : &mut World) {
    let raws = &super::RAWS.lock().unwrap();
    for spell in raws.raws.spells.iter() {
        spawn_named_spell(raws, ecs, &spell.name);
    }
}

pub fn find_spell_entity(ecs : &World, name : &str) -> Option<Entity> {
    let names = ecs.read_storage::<Name>();
    let spell_templates = ecs.read_storage::<SpellTemplate>();
    let entities = ecs.entities();

    for (entity, sname, _template) in (&entities, &names, &spell_templates).join() {
        if name == sname.name {
            return Some(entity);
        }
    }
    None
}

pub fn find_spell_entity_by_name(
    name : &str,
    names : &ReadStorage::<Name>,
    spell_templates : &ReadStorage::<SpellTemplate>,
    entities : &Entities) -> Option<Entity>
{
    for (entity, sname, _template) in (entities, names, spell_templates).join() {
        if name == sname.name {
            return Some(entity);
        }
    }
    None
}

pub fn spawn_named_entity(raws: &RawMaster, ecs : &mut World, key : &str, pos : SpawnType) -> Option<Entity> {
    if raws.item_index.contains_key(key) {
        return spawn_named_item(raws, ecs, key, pos);
    } else if raws.mob_index.contains_key(key) {
        return spawn_named_mob(raws, ecs, key, pos);
    } else if raws.prop_index.contains_key(key) {
        return spawn_named_prop(raws, ecs, key, pos);
    }

    None
}


struct MagicItemChance{
    pub common: i32,
    pub uncommon: i32,
    pub rare: i32,
    pub legendary: i32,
}

pub fn get_magic_item_loot_table_weight(depth: i32) -> MagicItemChance {
    match depth {
        // common = 432 / 1000 = 23.0%
        // uncommon = 400 / 1000 = 50%
        // rare = 160 / 1000 = 25.0%
        // legendary = 20 / 1000 = 2.0% 
        21 | 22 | 23 | 24 | 25 | 26 => MagicItemChance{common: 230, uncommon: 500, rare: 250, legendary: 20},
        // common = 432 / 1000 = 43.2%
        // uncommon = 400 / 1000 = 40%
        // rare = 160 / 1000 = 16.0%
        // legendary = 8 / 1000 = .8% 
        16 | 17 | 18 | 19 | 20 => MagicItemChance{common: 432, uncommon: 400, rare: 160, legendary: 8},
        // common = 616 / 1000 = 61.6%
        // uncommon = 300 / 1000 = 30%
        // rare = 80 / 1000 = 8.0%
        // legendary = 4 / 1000 = .4% 
        11 | 12 | 13 | 14 | 15 => MagicItemChance{common: 616, uncommon: 300, rare: 80, legendary: 4},
        // common = 879 / 1000 = 75.8%
        // uncommon = 200 / 1000 = 20%
        // rare = 40 / 1000 = 4.0%
        // legendary = 2 / 1000 = .2% 
        6 | 7 | 8 | 9 | 10 => MagicItemChance{common: 758, uncommon: 200, rare: 40, legendary: 2},
        // common = 879 / 1000 = 87.9%
        // uncommon = 100 / 1000 = 10%
        // rare = 20 / 1000 = 2.0%
        // legendary = 1 / 1000 = .1% 
        1 | 2 | 3 | 4 | 5 | _ => MagicItemChance{common: 879, uncommon: 100, rare: 20, legendary: 10000},
        // 1 | 2 | 3 | 4 | 5 | _ => MagicItemChance{common: 879, uncommon: 100, rare: 20, legendary: 1},
        // 26 => MagicItemChance{uncommon: 0.9, rare: 0.2, legendary: 0.01},
        // 25 => MagicItemChance{uncommon: 0.8, rare: 0.125, legendary: 0.005},
        // 24 => MagicItemChance{uncommon: 0.7, rare: 0.1, legendary: 0.0033},
        // 23 => MagicItemChance{uncommon: 0.6, rare: 0.05, legendary: 0.0025},
        // 21 | 22 => MagicItemChance{uncommon: 0.5, rare: 0.033, legendary: 0.002},
        // 17 | 18 | 19 | 20 => MagicItemChance{uncommon: 0.4, rare: 0.025, legendary: 0.001667},
        // 13 | 14 | 15 | 16 => MagicItemChance{uncommon: 0.3, rare: 0.02, legendary: 0.001429},
        // 9 | 10 | 11 | 12 => MagicItemChance{uncommon: 0.2, rare: 0.01667, legendary: 0.00125},
        // 5 | 6 | 7 | 8 => MagicItemChance{uncommon: 0.125, rare: 0.0133, legendary: 0.001111},
        // 1 | 2 | 3 | 4 => MagicItemChance{uncommon: 0.1, rare: 0.01, legendary: 0.001},
        // _ => MagicItemChance{uncommon: 0.1, rare: 0.01, legendary: 0.001 },
    }
}


pub enum SpawnTableType { Item, Mob, Prop }

pub fn spawn_type_by_name(raws: &RawMaster, key : &str) -> SpawnTableType {
    if raws.item_index.contains_key(key) {
        SpawnTableType::Item
    } else if raws.mob_index.contains_key(key) {
        SpawnTableType::Mob
    } else {
        SpawnTableType::Prop
    }
}

pub fn get_spawn_table_for_depth(raws: &RawMaster, depth: i32) -> MasterTable {
    use super::SpawnTableEntry;

    let magic_item_weights = get_magic_item_loot_table_weight(depth);

    let available_options : Vec<&SpawnTableEntry> = raws.raws.spawn_table
        .iter()
        .filter(|a| depth >= a.min_depth && depth <= a.max_depth)
        .collect();

    let mut rt = MasterTable::new();
    for e in available_options.iter() {
        // This iterates the entire spawn table, which includes mobs, items & props
        // We want to treat items differently, and set their weights based off of 
        // item rarity and depth level
        // We treat non-magic items & common magic items as both having common spawn weights
        let mut weight = e.weight;

        if raws.item_index.contains_key(&e.name) {
            let item_template = &raws.raws.items[raws.item_index[&e.name]];
            if let Some(magic) = &item_template.magic {
                let class = match magic.class.as_str() {
                    "uncommon" => weight = magic_item_weights.uncommon,
                    "rare" => weight = magic_item_weights.rare,
                    "legendary" => weight = magic_item_weights.legendary,
                    _ => weight = magic_item_weights.common
                };
            }
            else {
                // Non magic item, assign common weight
                weight = magic_item_weights.common;
            }
        }
        else if e.add_map_depth_to_weight.is_some() {
            // We don't add depth weight to items
            weight += depth;
        }
        rt.add(e.name.clone(), weight, raws);
    }

    rt
}

pub fn get_item_drop(raws: &RawMaster, table: &str) -> Option<String> {
    if raws.loot_index.contains_key(table) {
        let mut rt = RandomTable::new();
        let available_options = &raws.raws.loot_tables[raws.loot_index[table]];
        for item in available_options.drops.iter() {
            rt.add(item.name.clone(), item.weight);
        }
        let result =rt.roll();
        return Some(result);
    }

    None
}
