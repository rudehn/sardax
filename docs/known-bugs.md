# Known bugs
* The multi-processing system dispatcher is currently disabled. When it's enabled, system execution order isn't deterministic, so some systems will have bugs, like traps not firing when stepped on
* Some dungeon layouts will not spawn an outer wall. Floor tiles are adjacent to the map out of bounds
* Setting fire to fire that already exists will give the fire a "burning" status
* https://github.com/amethyst/rustrogueliketutorial/issues/165
* https://github.com/amethyst/rustrogueliketutorial/issues/188
* https://github.com/amethyst/rustrogueliketutorial/issues/208
* https://github.com/amethyst/rustrogueliketutorial/issues/205
* A* pathfinding is incredibly slow
* Casting damage through a staff doesn't log damage
* Map covers up the console log
* Respawning and then taking damage in new game kills you immediately