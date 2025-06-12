use rltk::{rex::XpFile};

rltk::embedded_resource!(SMALL_DUNGEON, "../../resources/SmallDungeon_80x50.xp");
rltk::embedded_resource!(WFC_DEMO_IMAGE1, "../../resources/wfc-demo1.xp");
rltk::embedded_resource!(WFC_DEMO_IMAGE2, "../../resources/wfc/wfc-demo2.xp");
rltk::embedded_resource!(WFC_POPULATED, "../../resources/wfc-populated.xp");
rltk::embedded_resource!(WFC_BASIC_CELLS, "../../resources/wfc/basic_cells.xp");
rltk::embedded_resource!(WFC_BASIC_CELLS1, "../../resources/wfc/basic_cells1.xp");
rltk::embedded_resource!(WFC_GOBLIN_FIRE, "../../resources/prefabs/goblin_fire.xp");

pub struct RexAssets {
    pub menu : XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../../resources/SmallDungeon_80x50.xp");
        rltk::link_resource!(WFC_DEMO_IMAGE1, "../../resources/wfc-demo1.xp");
        rltk::link_resource!(WFC_DEMO_IMAGE2, "../../resources/wfc/wfc-demo2.xp");
        rltk::link_resource!(WFC_POPULATED, "../../resources/wfc-populated.xp");
        rltk::link_resource!(WFC_BASIC_CELLS, "../../resources/wfc/basic_cells.xp");
        rltk::link_resource!(WFC_BASIC_CELLS1, "../../resources/wfc/basic_cells1.xp");
        rltk::link_resource!(WFC_BASIC_CELLS1, "../../resources/prefabs/goblin_fire.xp");

        RexAssets{
            menu : XpFile::from_resource("../../resources/SmallDungeon_80x50.xp").unwrap()
        }
    }
}
