//! This module implements the garden, including a highly performant germination implementation


// Re-export types from this module
pub use garden::Garden;
pub use seeds::SeedPacket;

/// Sow the given seed packets.
pub fn sow(seeds: Vec<SeedPacket>) {
    todo!()
}

/// Harvest the produce in the garden that is ready
pub fn harvest(garden: &mut Garden) {
    todo!()
}

/// Garden mod here's a description: garden
mod garden {
    pub struct Garden(pub u32);
}

/// Seeds mod here's a description: seeds
mod seeds {

    /// Seeds struct here's a description: seeds
    pub struct SeedPacket(pub u32, pub u32);
}
