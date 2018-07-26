use {Byte};

const MEMORY_SIZE: usize = 4096;

/// Memory
/// 
/// x000 to x1FF is mostly reserved for the interperter
/// x200 is where most programs start
/// x600 is where programs intended for eti 600 computers starts
struct Memory {
    data: [Byte; MEMORY_SIZE as usize]
}

