use crate::prelude::*;

pub fn spawn_player(esc: &mut World, pos: Point) {
    esc.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
