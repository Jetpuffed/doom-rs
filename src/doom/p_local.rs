//! Original file: p_local.h
//! 
//! Description: Play functions, animation, and global header.

use super::m_fixed::*;
use super::d_think::*;

const FLOAT_SPEED: i32 = FRAC_UNIT * 4;
const MAX_HEALTH: i32 = 100;
const VIEW_HEIGHT: i32 = 41 * FRAC_UNIT;

// Map blocks are used to check movement against lines and things
const MAP_BLOCK_UNITS: i32 = 128;
const MAP_BLOCK_SIZE: i32 = MAP_BLOCK_UNITS * FRAC_UNIT;
const MAP_BLOCK_SHIFT: i32 = FRAC_BITS + 7;
const MAP_BMASK: i32 = MAP_BLOCK_SIZE - 1;
const MAP_B_TO_FRAC: i32 = MAP_BLOCK_SHIFT - FRAC_BITS;

// Player radius for movement checking
const PLAYER_RADIUS: i32 = 16 * FRAC_UNIT;

// MAX_RADIUS is for pre-calculated sector block boxes
// The spider demon is larger, but we do not have any moving sectors nearby
const MAX_RADIUS: i32 = 32 * FRAC_UNIT;
const GRAVITY: i32 = FRAC_UNIT;
const MAX_MOVE: i32 = 30 * FRAC_UNIT;
const USE_RANGE: i32 = 64 * FRAC_UNIT;
const MELEE_RANGE: i32 = 64 * FRAC_UNIT;
const MISSILE_RANGE: i32 = 32 * 64 * FRAC_UNIT;

// Follow a player exclusively for 3 seconds
const BASE_THRESHOLD: i32 = 100;

// Both the head and tail of the thinker list

