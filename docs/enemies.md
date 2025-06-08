# Proposed types of enemies
* Goblins - low attack, low hp
* Orcs - high attack, medium hp
* Ogre - knockback or stun
* Giant - stun or knockback
* Lizardmen - very high attack, low hp
* Troll - high attack, very high hp, regenerates, slow
* Dragon
* Slime
* Giant spider


{
        "name" : "Entity Name",
        "renderable": {
            "glyph" : "s",
            "fg" : "#FF0000",
            "bg" : "#000000",
            "order" : 1
        },
        "blocks_tile" : true,
        "vision_range" : 18,
        "movement" : "static",
        // "unique": True,
        // can pickup/equip items - AI setting?
        "health": "4d10+4",
        "stats": {
            "speed": 100,
            "evasion": 10,
            "armor": 10,
            <!-- "str": 10, -->
            <!-- "dex": 10, -->
        },
        "traits": {
            "poisonous", "slow_moving", "stunning_attacks", "lesser regeneration"
        },
        "attacks": [
            { "name" : "bite", "hit_bonus" : 1, "damage" : "1d8+3", "on_hit": {
                        "proc_chance": 1,
                        "proc_target": "enemy",
                        "proc_effects": {
                            "slow" :  {}
                        }
                    }. }
        ],
        "skills": [
            "web", "zap"
        ],
        "equipped": {

        },
        "inventory": {

        },
        "faction": "Enemies"
    },