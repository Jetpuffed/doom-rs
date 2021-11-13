//! Original file: am_map.c / am_map.h
//! 
//! Description: The automap code

use super::doomdef::*;
use super::m_fixed::*;

// For use if I do walls with outsides/insides
const REDS: i32 = 256 - 5 * 16;
const RED_RANGE: i32 = 16;
const BLUES: i32 = 256 - 4 * 16 + 8;
const BLUE_RANGE: i32 = 8;
const GREENS: i32 = 7 * 16;
const GREEN_RANGE: i32 = 16;
const GRAYS: i32 = 6 * 16;
const GRAY_RANGE: i32 = 16;
const BROWNS: i32 = 4 * 16;
const BROWN_RANGE: i32 = 16;
const YELLOWS: i32 = 256 - 32 + 7;
const YELLOW_RANGE: i32 = 1;
const BLACK: i32 = 0;
const WHITE: i32 = 256 - 47;

// Automap colors
const BACKGROUND: i32 = BLACK;
const YOUR_COLORS: i32 = WHITE;
const YOUR_RANGE: i32 = 0;
const WALL_COLORS: i32 = REDS;
const WALL_RANGE: i32 = RED_RANGE;
const TS_WALL_COLORS: i32 = GRAYS;
const TS_WALL_RANGE: i32 = GRAY_RANGE;
const FD_WALL_COLORS: i32 = BROWNS;
const FD_WALL_RANGE: i32 = BROWN_RANGE;
const CD_WALL_COLORS: i32 = YELLOWS;
const CD_WALL_RANGE: i32 = YELLOW_RANGE;
const THING_COLORS: i32 = GREENS;
const THING_RANGE: i32 = GREEN_RANGE;
const SECRET_WALL_COLORS: i32 = WALL_COLORS;
const SECRET_WALL_RANGE: i32 = WALL_RANGE;
const GRID_COLORS: i32 = GRAYS + GRAY_RANGE / 2;
const GRID_RANGE: i32 = 0;
const X_HAIR_COLORS: i32 = GRAYS;

// Drawing stuff
const FB: i32 = 0;

const AM_PAN_DOWN_KEY: i32 = KEY_DOWN_ARROW;
const AM_PAN_UP_KEY: i32 = KEY_UP_ARROW;
const AM_PAN_RIGHT_KEY: i32 = KEY_RIGHT_ARROW;
const AM_PAN_LEFT_KEY: i32 = KEY_LEFT_ARROW;
const AM_ZOOM_IN_KEY: i32 = 61;     /* char '=' */
const AM_ZOOM_OUT_KEY: i32 = 45;    /* char '-' */
const AM_START_KEY: i32 = KEY_TAB;
const AM_END_KEY: i32 = KEY_TAB;
const AM_GO_BIG_KEY: i32 = 48;      /* char '0' */
const AM_FOLLOW_KEY: i32 = 102;     /* char 'f' */
const AM_GRID_KEY: i32 = 103;       /* char 'g' */
const AM_MARK_KEY: i32 = 109;       /* char 'm' */
const AM_CLEAR_MARK_KEY: i32 = 99;  /* char 'c' */
const AM_NUM_MARK_POINTS: i32 = 10;

// Scale on entry
const INIT_SCALE_MTOF: f32 = 0.2 * FRAC_UNIT as f32;

// How much the automap moves window per tick in framebuffer coordinates
// Moves 140 pixels in 1 second
const F_PAN_IN_C: i32 = 4;

// How much zoom-in per tick
// Goes to 2x in 1 second
const M_ZOOM_IN: i32 = (1.02 * FRAC_UNIT as f32) as i32;

// How much zoom-out per tick
// Pulls out to 0.5x in 1 second
const M_ZOOM_OUT: i32 = (FRAC_UNIT as f32 / 1.02) as i32;

// Translates between framebuffer and map distances
/* TODO (starts at line 119) */

// Translates between framebuffer and map coordinates
/* TODO (starts at line 122) */

// The following is crap
/* TODO: (starts at line 126) */

struct FPoint(i32, i32);

struct FLine(FPoint, FPoint);

struct MPoint(Fixed, Fixed);

struct MLine(MPoint, MPoint);

struct ISlope(Fixed, Fixed);

// The vector graphics for the automap.
// A line drawing of the player pointing right, starting from the middle.
/* TODO: (starts on line 160) */
