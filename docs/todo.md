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
each item should have a rarity & floor range
loot table should first roll for loot rarity, 1: legendary, 10: rare, 100: uncommon, 1000: common,
 - then roll rarity from available floor range  
loot table updates