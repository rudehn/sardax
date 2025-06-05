# Feature Requests
* Increase the size of the screen, especially for the web
* Add a view that can be accessed that shows the entire message history log
* It would be nice to have an optional? recharge for chargeable items, such as staves. The recharge time would be like 500 steps and could be higher the more potent the item is
* It would be an interesting concept for fire on the map to destroy any items in the same cell
* Revisit fire mechanics - currently not scalable damage. Determine tradeoffs of boosting duration, intensity (damage/turn), fire stacks, etc
    * Adding a stat to boost fire stacks (damage done from fire attacks) would be an interesting stat on items or as a skill
* Add support for spells doing damage in a line, such as the tunneling items
* Update tunneling staff to tunnel more than 1 cell
* Update tunneling staff targeting to only select walls (not floors)
* Determine tradeoff of enemies dropping all of their equipment vs chance to drop 1 or more items, vs random loot drop, or some combination of approaches
* Add concept for sleeping/guarding/idle AI. So any prefabs we place won't have their mobs just start wandering off
* Determine how to add in damage types, IE expand pass simple physical damage into fire, poison, etc. Does the attack itself inflict that damage type? Does the attack inflict a burning or poison status effect? etc
* How does the vision range feel? Does it need updated?
* Each type of weapon should have a unique stat or effect. Such as a spear hitting 2 tiles in front of character, while an axe does AOE in a circle around character, etc. Giving a unique feel for each type of weapon makes for more interesting gameplay options, rather than just swapping out for a weapon with better stats
    * Possible effects - club=knockback, spear=pierce, sword=more damage, axe=AOE around user
* Add additional types of ground tiles, such as grass & long grass. Long grass would obscure vision
* Update fire system to let fire spread if any burnable tiles, such as grass, are adjacent to a current fire tile
* Rework status effect duration updates to move them out of the initiative system
  * Add a global turn counter that rotates every 10 frames, make a system read for the counter & run any pre/post turn processing?
* When we remove turns, add in a "TurnComplete" component with the turn action cost. Then have a system iterate those turn complete components, update the linked entities initiative, and run any status effects on the entity
* TODO - add creature XP & mana
* Gain skills by beating bosses
- Weapons should have different attack speeds
- Make loot table have a chance to drop nothing
- Separate out spawns.json into separate files
- Add armor attribute
- Add lifesteal
- Add a spawn item function
  - Chance to spawn common/rare/legendary items. Rare/Legendary change increases as dungeon level goes up
  - Add a spawn table for items?
- Mob abilities should not use a chance %, instead use mana
- Add action/move energy multiplier
- Enemies with auras
- Enemies that target the weakest creature
- Make all player spells rechargeable
- Limit spell slots


Create AI for determining which spells to cast and where to cast them
- Create a new AI system that...
  - Loops through all actions an entity can take, including spells, melee & ranged weapon attacks, movement
    - For each entity in view, determine an action weight for using a spell/attack on the entity
    - Select action with the highest weight (or weighed random selection)
    - Action weights are determined by the following criteria
      - How much damage an action will inflict on hostile enemies
      - How much damage healed on allies
      - Different weights for status effects on enemies
      - Different weights for boosts on allies
    - Only calculate for actions that are off cooldown

# TODO Priority
* Update tooltip to show entity inventory
* ogre enemy, attack causes stun
* REfactor armor class
- It would be nice if the dungeon exit was always at the bottom center of the first dungeon floor
* Bumping into the dungeon exit should try to leave dungeon, rather than standing on the exit and pressing ','
* Hovering over an item should show stats
* Hovering over an enemy should show stats
* Organize what spawns structure template is for each entity type
* New title screen
* It would be nice to have a list of all the items/enemies in line of sight be listed on the side of the screen, like Brogue does
  * Hovering over the entity on the side would highlight it on the map
- Update kill drops to roll from loot table
  - creature table vs floor table vs creature inventory
- Particle effects should have the option to use background tile color