//! Original file: d_items.c / d_items.h
//! 
//! Description: Items, key cards, artifacts, weapon, ammunition.

use super::doomdef::*;

// Weapon info: sprite frames, ammunition use.
pub struct WeaponInfo
{
    ammo: AmmoType,
    up_state: i32,
    down_state: i32,
    ready_state: i32,
    atk_state: i32,
    flash_state: i32,
}
