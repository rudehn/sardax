#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub struct PrefabRoom {
    pub template : &'static str,
    pub width : usize,
    pub height: usize,
    pub first_depth: i32,
    pub last_depth: i32
}

// Hold a container of all room prefabs
pub struct RoomPrefabs{
    pub prefabs: Vec<PrefabRoom>
}

impl RoomPrefabs{
    pub fn new() -> RoomPrefabs{
        RoomPrefabs{
            prefabs: vec![TOTALLY_NOT_A_TRAP, CHECKERBOARD, SILLY_SMILE, GOBLIN_WATCH_FIRE, ORC_WATCH_FIRE]
        }
    }
}

#[allow(dead_code)]
pub const TOTALLY_NOT_A_TRAP : PrefabRoom = PrefabRoom{
    template : TOTALLY_NOT_A_TRAP_MAP,
    width: 5,
    height: 5,
    first_depth: 0,
    last_depth: 100
};

#[allow(dead_code)]
const TOTALLY_NOT_A_TRAP_MAP : &str = "
     
 ^^^ 
 ^!^ 
 ^^^ 
     
";

#[allow(dead_code)]
pub const SILLY_SMILE : PrefabRoom = PrefabRoom{
    template : SILLY_SMILE_MAP,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100
};

#[allow(dead_code)]
const SILLY_SMILE_MAP : &str = "
      
 ^  ^ 
  ##  
      
 #### 
      
";

#[allow(dead_code)]
pub const CHECKERBOARD : PrefabRoom = PrefabRoom{
    template : CHECKERBOARD_MAP,
    width: 6,
    height: 6,
    first_depth: 0,
    last_depth: 100
};

#[allow(dead_code)]
const CHECKERBOARD_MAP : &str = "
      
 #^#  
 g#%# 
 #!#  
 ^# # 
      
";

#[allow(dead_code)]
pub const GOBLIN_WATCH_FIRE : PrefabRoom = PrefabRoom{
    template : GOBLIN_WATCH_FIRE_MAP,
    width: 3,
    height: 3,
    first_depth: 0,
    last_depth: 100
};

const GOBLIN_WATCH_FIRE_MAP : &str = "
   
g☼g
   
";

pub const ORC_WATCH_FIRE : PrefabRoom = PrefabRoom{
    template : ORC_WATCH_FIRE_MAP,
    width: 3,
    height: 3,
    first_depth: 0,
    last_depth: 100
};

const ORC_WATCH_FIRE_MAP : &str = "
 o 
 ☼o
   
";