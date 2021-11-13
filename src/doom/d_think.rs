//! Original file: d_think.h
//! 
//! Description: MapObj data. Map Objects or mobjs are actors, entities,
//! thinker, take-your-pick... anything that moves, acts, or suffers
//! state changes of more or less violent nature.

use std::ffi::c_void;

// Experimental stuff. To compile this as "ANSI C with classes"
// we will need to handle the various action functions cleanly.
pub type ActionFv = fn() -> ();
pub type ActionFp1 = fn(_: c_void) -> ();
pub type ActionFp2 = fn(_: c_void, _: c_void) -> ();

pub union ActionF
{
    acp1: ActionFp1,
    acv: ActionFv,
    acp2: ActionFp2,
}

// Historically, "think_t" is yet another function pointer
// to a routine to handle an actor.
pub type Think = ActionF;

// Doubly linked list of actors.
pub struct Thinker
{
    prev: *mut Thinker,
    next: *mut Thinker,
    function: Think,
}
