Normalize some of the components and deserialization structs, such as those used for natural attacks and weapons
Perhaps effects should be a parametrized enum
Better visual of status effects for entities on side bar
- Don't double stack paralasis
- Implement paralazis 
Add constants for color, make status effect on side bar & the particle effect constant
TODO - add particle effect colors for status effects - make same color as UI
Turn status system needs to subtract energy cost
Merge turn status system with end of turn system & apply right after initiative system?
Migrate enemy abilities to become fully spells
- Spells have an optional min range
- Vision system queries the spells, not the spell metadata on the entity
- Spells should consume mana (enemies)??
- How to prevent monster from spamming spells & running out of mana
- How does a bigger AI refactor play in here?




use_spell_hotkey
spell_trigger
spawn_all_spells exists?
spawner.rs import errors

death - drop all inventory, drop all equipped, roll loot table
ranged_target function shows "Select Target" in top left corner
Magic Items not showing color on map
Item actions not taking up energy
Fireball staff explosion shows up a turn late
Using an item doesn't display on game log















TODO
* Add in stats to player and mobs
* Add in system to add stat bonus
* Update consumable to add stat bonus as permanent or temporary (using duration?)
  * Requires 2 new effects to apply stats?
* Update item template to break effect out of consumable, that way if thrown, an item can trigger it's effects 
  * Also, remove the 'attribute' section and use an "apply_effects" effect
* fill out stats markdown
add in AttributeEffect
add in EquipmentChanged
review attribute modifier trigger
add encumbrance system back in
  * Add in encumbered and immobile status
    * https://github.com/rudehn/rust-roguelike/blob/e539e483833454e8ec9489810455786467d82a3e/src/systems/ai/encumbrance_system.rs
    * https://github.com/rudehn/rust-roguelike/commit/e539e483833454e8ec9489810455786467d82a3e#diff-6df12e7948106bcc4f43cc2b6d4f2bd1cb0d662a152a3d25b20b149a94b343f7R1-R15

Add stats back in to hud + tooltip
* https://github.com/rudehn/rust-roguelike/commit/e539e483833454e8ec9489810455786467d82a3e#diff-e1a0acfc064fa0cbe59092b9ac5e00a8b597846bd5a62fc27a00e76f37c2ab74L8-L10

* Handle bug in cycling targets when only 1 target is available