# Design Ideas

TileSet - https://en.wikipedia.org/wiki/Code_page_437

## Basic Game Play Loop
* Win condition
  - Go down through the dungeon to retrieve the amulet on level 10 and return up the floors and out the dungeon entrance
  - If you die, game over
* Progression
  - Enemies will get stronger over time, so the character will also get stronger through equipment, spells and stat increases

* Map generation
* Mobs
  - AI
    - Sleeping, wandering, chasing, guarding
* Items
  * Generation
* Spells
* Damage System
  * Hit Chance
    - hit_chance = attacker_accuracy * .987 ^ defender_evasion
    - accuracy = 100; later multiply by 1.065^(enchant level)
    - evasion = 10 * dex bonus
  * Damage
    - damage = roll(dmg_min, dmg_max) - (roll(armor/2, armor) - attacker_pierce)
      - Min/max damage is from the attackers weapon, natural attack, or 1d4 if unarmed
      - Armor is from the defenders gear or natural armor
      - Pierce is an attribute on weapons
  * Damage types
    - Physical
    - Fire
    - Poison
  * Resistances/Weakness/Immunities
  * Status Effects
    - Burning: 1d3 for 5 turns. Inflicted by any fire attack. Additional fire damage will reset the counter. Stepping into water will extinguish. TODO - implement fire spreading on flammible terrain
    - Poison: Deal n damage for m turns. Poison stacks, new poison damage is added to the previous amount and the number of turns is increased by the new duration
* Turn system
* UI
  - Items & creatures in view should be displayed on the side
    - Health & status effects should be displayed
    - Hovering over these entities should show a detailed description
  - Hovering over an item or creature on the ground should show a description

## Level Generation
* https://www.rockpapershotgun.com/how-do-roguelikes-generate-levels
* https://brogue.fandom.com/wiki/Level_Generation





Mobs
- https://www.reddit.com/r/roguelikedev/comments/viicvz/share_some_enemyability_gimmicks/
- Each mob should have a unique theme, possible themes include
  - basic melee
  - basic ranged, single target
  - debuff
  - inflict status effect,
  - AOE 
  - poison
  - slow
  - invisible
  - glass cannon
  - cast silence
  - tough/regenerates, low damage
  - stun only, but loses 1 hp every time it attacks
  - parasite, giving positive + negative benefits
  - spawn amalgamation if too many of same entity type die in same place
  - an enemy with knockback
  - enemy with high armor and attack, but very slow
  - stationary props that have some kind of environmental impact
- Monsters gain strength through levels by adding prefixes. Prefixes add stats  

- Monsters using items
- Monsters picking up items
- Monsters coordinating/boosting other monsters
- Mimics (appear as another symbol)
- Demons (appear as &)
- Types
  - goblins
  - orcs
  - dark elves
  - dwarves
  - dragon
  - bandits
  - slimes
  - trolls
- Bosses
- Minibosses

Spawning:
- 

Item
- Generation
  - items should have an identity
- Types
  - Consumables
    - Potion
      - Healing
      - Mana
      - Attr increase (temp vs permanent)
    - Scroll
  - Book of magic
  - Food
  - Weapons
    - bow - ranged
    - dagger - can pierce armor, initiative boost, low damage
    - sword - normal speed
    - axe - chance to bleed, initiative penalty, heavy damage
    - hammer - chance to stun, chance to knockback initiative penalty, heavy damage
  - Armor
    - Slots
      - helmet
      - chest
      - gloves
      - legs
      - boots
      - shield
    - light armor
    - heavy armor - high def; magic penalty; slow initiative
    - Robes
  - Jewelry
    - 2 slots
    - types
      - amulet (rare)
      - ring
    - modify character's attributes, grant various powers, special ability or resistance. Also they can allow magical power or spell to be activated. 

Ideas:
  - Lanterns
  - Wand
    - invokes certain power which is commonly unknown at first place, the wands have a limited number of charges or uses, and can be recharged using other actions.
  - Rod
    - Rods use energy of wielder to create desired effect or absorb it slowly from environment effectively being usable only once per certain period of time.
- Treasure rooms guarded/locked w/ spawn table for higher level loot
- more prefabs
- rare/unique items in vendors
- item rarity should affect drop chances: common, uncommon, rare, legendary
- Add memory tiles
- Update display coloring
- TODO - customizable map size per level
- status immunities
- status effects:
  - berserk
  - blind - vision reduced/removed
  - bleed - lose 2.5% max hp per tick
  - confuse - attack both ally and enemy
  - fear - causes enemies to run away from source of fear
  - stun - prevent all actions for 1 turn
  - curse - drop in stats
  - slow - reduce targets speed by 50%, less for bosses
  - poison - lose a small amount of hp per turn
  - burn
  - charmed - you control the charmed target
  - silence - target is unable to use magic spells
  - sleep - skips turn
  - trapped - can't move
  - invisible - can't be seen
  - Webbed - slowed/stopped for x turns


Converting DND armor stats into this game

Leather armor AC = 1
Weight: 10

head: 15%
torso: 40%
legs: 25%
feet: 10%
hands: 10%


https://github.com/jice-nospam/doryen-rs/tree/master/docs/demo
https://github.com/Alvarz/Roguelike-Rust/tree/master/src