# Known bugs
* The multi-processing system dispatcher is currently disabled. When it's enabled, system execution order isn't deterministic, so some systems will have bugs, like traps not firing when stepped on
* Some dungeon layouts will not spawn an outer wall. Floor tiles are adjacent to the map out of bounds
* Hovering over a fire entity that spawned this turn will show `Nameless item (bug)`
* Setting fire to fire that already exists will give the fire a "burning" status
* https://github.com/amethyst/rustrogueliketutorial/issues/165
* https://github.com/amethyst/rustrogueliketutorial/issues/188
* https://github.com/amethyst/rustrogueliketutorial/issues/208
* https://github.com/amethyst/rustrogueliketutorial/issues/205
* A* pathfinding is incredibly slow
* Targeting with the bow can sometimes cause an exception
* Casting damage through a staff doesn't log damage
* SimpleMapBuilder sometimes puts starting & ending stairs in the same room
* Map covers up the console log
* turn_status system should consume energy
* Paralasys doesn't prevent movement for player
* Respawning and then taking damage in new game kills you immediately
* Tile cost is not reflected in actual energy consumption