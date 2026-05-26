// All block types we will have.
#[derive(Clone, Copy, PartialEq)]
pub enum BlockData {
    AIR,
    GRASS,
    DIRT,
    STONE,
    WOOD,
    LEAVES,
}

// Each type of block has six faces which can be rendered. Each face (until we
// add non-cubic blocks) corresponds to a single square in the texture atlas, so
// this is a store for which faces exist to be rendered.
pub struct BlockTextureCoordinates {
    pub right: (f32, f32),
    pub left: (f32, f32),
    pub top: (f32, f32),
    pub bottom: (f32, f32),
    pub front: (f32, f32),
    pub back: (f32, f32),
}

// TODO: stash these in a file somewhere?

// If a block is invalid, we render with the Adam pickaxe texture
const ERROR_COORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.8, 0.0),
    left: (0.8, 0.0),
    top: (0.8, 0.0),
    bottom: (0.8, 0.0),
    front: (0.8, 0.0),
    back: (0.8, 0.0),
};

// For completeness, air will give us the Jake texture in case it gets rendered
// by accident, so we know what is happening
const AIR_COORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.9, 0.0),
    left: (0.9, 0.0),
    top: (0.9, 0.0),
    bottom: (0.9, 0.0),
    front: (0.9, 0.0),
    back: (0.9, 0.0),
};

const GRASS_COORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.1, 0.0),
    left: (0.1, 0.0),
    top: (0.0, 0.0),
    bottom: (0.2, 0.0),
    front: (0.1, 0.0),
    back: (0.1, 0.0),
};

const DIRT_COORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.2, 0.0),
    left: (0.2, 0.0),
    top: (0.2, 0.0),
    bottom: (0.2, 0.0),
    front: (0.2, 0.0),
    back: (0.2, 0.0),
};

const STONE_COORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.3, 0.0),
    left: (0.3, 0.0),
    top: (0.3, 0.0),
    bottom: (0.3, 0.0),
    front: (0.3, 0.0),
    back: (0.3, 0.0),
};

const WOOD_CORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.3, 0.1),
    left: (0.3, 0.1),
    top: (0.4, 0.1),
    bottom: (0.4, 0.1),
    front: (0.3, 0.1),
    back: (0.3, 0.1),
};

const LEAF_COORDS: BlockTextureCoordinates = BlockTextureCoordinates {
    right: (0.5, 0.1),
    left: (0.5, 0.1),
    top: (0.5, 0.1),
    bottom: (0.5, 0.1),
    front: (0.5, 0.1),
    back: (0.5, 0.1),
};

pub fn get_block_texture_coordinates(block_type: BlockData) -> BlockTextureCoordinates {
    match block_type {
        BlockData::AIR => AIR_COORDS,
        BlockData::GRASS => GRASS_COORDS,
        BlockData::DIRT => DIRT_COORDS,
        BlockData::STONE => STONE_COORDS,
        BlockData::WOOD => WOOD_CORDS,
        BlockData::LEAVES => LEAF_COORDS,
        // Not implemented yet!
        _ => ERROR_COORDS,
    }
}
