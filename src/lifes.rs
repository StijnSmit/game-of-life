use bevy::{
    prelude::*,
};

/// MARK: - Oscillator Life Forms
pub const BLINKER_SHAPE: [IVec2; 3] = [
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
];

pub const TOAD_SHAPE: [IVec2; 6] = [
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(2, 1),
];

pub const BEACON_SHAPE: [IVec2; 8] = [
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, -1),
    IVec2::new(2, -1),
    IVec2::new(1, -2),
    IVec2::new(2, -2),
];

pub const PULSAR_SHAPE: [IVec2; 48] = [
    // Inner 4 lines horizontal
    IVec2::new(-4, 1),
    IVec2::new(-3, 1),
    IVec2::new(-2, 1),
    IVec2::new(2, 1),
    IVec2::new(3, 1),
    IVec2::new(4, 1),
    IVec2::new(-4, -1),
    IVec2::new(-3, -1),
    IVec2::new(-2, -1),
    IVec2::new(2, -1),
    IVec2::new(3, -1),
    IVec2::new(4, -1),
    
    // Inner 4 lines vertical
    IVec2::new(1, 2),
    IVec2::new(1, 3),
    IVec2::new(1, 4),
    IVec2::new(1, -4),
    IVec2::new(1, -3),
    IVec2::new(1, -2),
    IVec2::new(-1, 2),
    IVec2::new(-1, 3),
    IVec2::new(-1, 4),
    IVec2::new(-1, -4),
    IVec2::new(-1, -3),
    IVec2::new(-1, -2),

    // Outer 4 vertical lines
    IVec2::new(-6, 2),
    IVec2::new(-6, 3),
    IVec2::new(-6, 4),
    IVec2::new(6, 2),
    IVec2::new(6, 3),
    IVec2::new(6, 4),
    IVec2::new(-6, -2),
    IVec2::new(-6, -3),
    IVec2::new(-6, -4),
    IVec2::new(6, -2),
    IVec2::new(6, -3),
    IVec2::new(6, -4),

    // Outer 4 horizontal lines
    IVec2::new(-4, 6),
    IVec2::new(-3, 6),
    IVec2::new(-2, 6),
    IVec2::new(4, 6),
    IVec2::new(3, 6),
    IVec2::new(2, 6),
    IVec2::new(-4, -6),
    IVec2::new(-3, -6),
    IVec2::new(-2, -6),
    IVec2::new(4, -6),
    IVec2::new(3, -6),
    IVec2::new(2, -6),
];

/// MARK: - Spaceship Life Forms
pub const GLIDER_SHAPE: [IVec2; 5] = [
    IVec2::new(0, 1),    
    IVec2::new(1, 0),    
    IVec2::new(2, 0),    
    IVec2::new(2, 1),    
    IVec2::new(2, 2),    
];
