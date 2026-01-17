#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Terrain(TerrainTile),
}

// ---------------- Terrain ----------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainTile {
    Grass(GrassTile),
    Ocean(OceanTile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrassTile {
    GrassDark = 66,
    GrassLight = 67,
    GrassSpottedLight1 = 130,
    GrassSpottedLight2 = 131,
    GrassSpottedLight3 = 132,
    GrassSpottedLight4 = 196,
    GrassSpottedLight5 = 197,
    GrassSpottedLight6 = 258,
    GrassSpottedLight7 = 259,
    GrassDiagonal = 193,
    GrassGrid = 194,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IslandTile {
    Sand = 70,
    SandSpotted1 = 72,
    SandSpotted2 = 73,
    SandSpotted3 = 74,
    SandEdgeTop = (28 * 64) + 7,
    SandEdgeBottom = (32 * 64) + 7,
    SandEdgeLeft = (30 * 64) + 5,
    SandEdgeRight = (30 * 64) + 9,
    SandCornerTopLeft = (29 * 64) + 6,
    SandCornerTopRight = (29 * 64) + 8,
    SandCornerBottomLeft = (31 * 64) + 6,
    SandCornerBottomRight = (31 * 64) + 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OceanTile {
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
}
