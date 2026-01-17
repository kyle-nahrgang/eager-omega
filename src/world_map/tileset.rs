#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileIndex {
    _GrassDark = 66,
    _GrassLight = 67,
    _GrassSpottedLight1 = 130,
    _GrassSpottedLight2 = 131,
    _GrassSpottedLight3 = 132,
    _GrassSpottedLight4 = 196,
    _GrassSpottedLight5 = 197,
    _GrassSpottedLight6 = 258,
    _GrassSpottedLight7 = 259,
    _GrassDiagonal = 193,
    _GrassGrid = 194,

    Ocean1 = 1164,
    Ocean2 = 1165,
    Ocean3 = 1166,
    Ocean4 = 1167,

    Ocean5 = 1228,
    Ocean6 = 1229,
    Ocean7 = 1230,
    Ocean8 = 1231,

    Ocean9 = 1292,
    Ocean10 = 1293,
    Ocean11 = 1294,
    Ocean12 = 1295,

    Ocean13 = 1356,
    Ocean14 = 1357,
    Ocean15 = 1358,
    Ocean16 = 1359,

    Sand = 70,
    SandSpotted1 = 72,
    SandSpotted2 = 73,
    SandSpotted3 = 74,
    SandEdgeTop = (28 * 64) + 7,
    SandEdgeBottom = (32 * 64) + 7,
    SandEdgeLeft = (30 * 64) + 5,
    SandEdgeRight = (30 * 64) + 9,
    SandEdgeTopLeft = (29 * 64) + 6,
    SandEdgeTopRight = (29 * 64) + 8,
    SandEdgeBottomLeft = (31 * 64) + 6,
    SandEdgeBottomRight = (31 * 64) + 8,
    SandCornerTopLeft = (28 * 64) + 6,
    SandCornerTopRight = (28 * 64) + 8,
    SandCornerBottomLeft = (32 * 64) + 6,
    SandCornerBottomRight = (32 * 64) + 8,
}

impl TileIndex {
    pub fn is_bottom_edge(&self) -> bool {
        matches!(self, TileIndex::SandEdgeBottom)
    }

    pub fn is_top_edge(&self) -> bool {
        matches!(self, TileIndex::SandEdgeTop)
    }

    pub fn is_right_edge(&self) -> bool {
        matches!(self, TileIndex::SandEdgeRight)
    }

    pub fn is_left_edge(&self) -> bool {
        matches!(self, TileIndex::SandEdgeLeft)
    }
}
