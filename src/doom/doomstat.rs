//! Original file: doomstat.c / doomstat.h
//! 
//! Description: All the global variables that store the internal state.
//! Theoretically speaking, the internal state of the engine should be found
//! by looking at the variables collected here, and every relevant module will
//! have to include this header file. In practice, things are a bit messy.

use super::doomdata::*;
