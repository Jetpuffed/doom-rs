//! Original file: doomdef.c / doomdef.h
//! 
//! Description: Internally used data structures for virtually everything,
//! key definitions, lots of other stuff.

/*
    Global parameters/defines.
*/

// DOOM version
const VERSION: i32 = 110;

// Game mode handling - identify IWAD version
// to handle IWAD dependent animations, etc..
enum GameMode
{
    Shareware,      // DOOM 1 shareware, E1, M9
    Registered,     // DOOM 1 registered, E3, M27
    Commercial,     // DOOM 2 retail, E1, M34
    Retail,         // DOOM 1 retail, E4, M36
    Indetermined,   // Well, no IWAD found.
}

// Mission packs - might be useful for TC stuff?
enum GameMission
{
    Doom,       // DOOM 1
    Doom2,      // DOOM 2
    PackTnt,    // TNT mission pack
    PackPlut,   // Plutonia pack
    None,
}

// Identify language to use, software localization.
enum Language
{
    English,
    French,
    German,
    Unknown,
}

/*
    The original source has a `#define RANGECHECK` here, omitting it for now
*/

// Do or do not use external soundserver.
// The sndserver binary to be run separately has been introduced by Dave Taylor.
// The integrated sound support is experimental, and unfinished. Default is synchronous.
// Experimental asynchronous timer based is handled by SNDINTR.
const SNDSERV: i32 = 1;
/* const SNDINTR: i32 = 1; */

// This one switches between MIT SHM (no proper mouse) and XFree86 DGA (mickey sampling).
// The original linuxdoom used SHM, which is default.
/* const X11_DGA: i32 = 1; */

// For resize of screen, at start of game. It will not work dynamically, see visplanes.
const BASE_WIDTH: i32 = 320;

// It is educational but futile to change this scaling e.g. to 2. Drawing of status bar,
// menus, etc., is tied to the scale implied by the graphics.
const SCREEN_MUL: i32 = 1;
const INV_ASPECT_RATIO: f32 = 0.625;    // 0.75, ideally

// Defines suck. C sucks. C++ might suck for OOP, but it sure is a better C. So there.
const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 200;

// The maximum number of players, multiplayer/networking.
const MAX_PLAYERS: i32 = 4;

// State updates, number of ticks / second.
const TICK_RATE: i32 = 35;

// The current state of the game: whether we are playing, gazing at the intermission
// screen, the game final animation, or a demo.
enum GameState
{
    Level,
    Intermission,
    Finale,
    DemoScreen,
}

/*
    Difficulty/skill settings and filters.
*/

// Skill flags.
const MTF_EASY: i32 = 1;
const MTF_NORMAL: i32 = 2;
const MTF_HARD: i32 = 4;

// Deaf monsters do not react to sound.
const MTF_AMBUSH: i32 = 8;

enum Skill
{
    Baby,
    Easy,
    Medium,
    Hard,
    Nightmare,
}

// Key cards.
enum Card
{
    BlueCard,
    YellowCard,
    RedCard,
    BlueSkull,
    YellowSkull,
    RedSkull,

    NumCards,
}

// The defined weapons, including a marker indicating user has not changed weapon.
enum WeaponType
{
    Fist,
    Pistol,
    Shotgun,
    Chaingun,
    Missile,
    Plasma,
    Bfg,
    Chainsaw,
    SuperShotgun,

    NumWeapons,

    // No pending weapon change.
    NoChange,
}

// Ammunition types defined.
enum AmmoType
{
    Clip,   // Pistol / chaingun ammo.
    Shell,  // Shotgun / double barreled shotgun.
    Cell,   // Plasma rifle, BFG.
    Misl,   // Missile launcher.
    NumAmmo,
    NoAmmo, // Unlimited for chainsaw / fist.
}

// Power up artifacts.
enum PowerType
{
    Invulnerability,
    Strength,
    Invisibility,
    IronFeet,
    AllMap,
    Infrared,
    NumPowers,
}

// Power up durations, how many seconds till expiration, assuming TICKRATE is 35 ticks / second.
enum PowerDuration
{
    Tickx30 = 30 * TICK_RATE as isize,      /* INVULNTICS */
    Tickx60 = 60 * TICK_RATE as isize,      /* INVISTICS & IRONTICS */
    Tickx120 = 120 * TICK_RATE as isize,    /* INFRATICS */
}

/*
    Because the Rust compiler doesn't like enumerations with duplicate discriminants,
    I had to work around this issue by creating associated constants.
*/
impl PowerDuration
{
    const INVULN_TICKS: PowerDuration = PowerDuration::Tickx30;
    const INVIS_TICKS: PowerDuration = PowerDuration::Tickx60;
    const INFRA_TICKS: PowerDuration = PowerDuration::Tickx120;
    const IRON_TICKS: PowerDuration = PowerDuration::Tickx60;
}

// DOOM keyboard definition. This is the stuff configured by Setup.exe.
// Most key data are simple ASCII (uppercased).
const KEY_RIGHT_ARROW: i32 = 0xAE;
const KEY_LEFT_ARROW: i32 = 0xAC;
const KEY_UP_ARROW: i32 = 0xAD;
const KEY_DOWN_ARROW: i32 = 0xAF;
const KEY_ESCAPE: i32 = 27;
const KEY_ENTER: i32 = 13;
const KEY_TAB: i32 = 9;
const KEY_F1: i32 = 0x80 + 0x3B;
const KEY_F2: i32 = 0x80 + 0x3C;
const KEY_F3: i32 = 0x80 + 0x3D;
const KEY_F4: i32 = 0x80 + 0x3E;
const KEY_F5: i32 = 0x80 + 0x3F;
const KEY_F6: i32 = 0x80 + 0x40;
const KEY_F7: i32 = 0x80 + 0x41;
const KEY_F8: i32 = 0x80 + 0x42;
const KEY_F9: i32 = 0x80 + 0x43;
const KEY_F10: i32 = 0x80 + 0x44;
const KEY_F11: i32 = 0x80 + 0x57;
const KEY_F12: i32 = 0x80 + 0x58;
const KEY_BACKSPACE: i32 = 127;
const KEY_PAUSE: i32 = 0xFF;
const KEY_EQUALS: i32 = 0x3D;
const KEY_MINUS: i32 = 0x2D;
const KEY_RSHIFT: i32 = 0x80 + 0x36;
const KEY_RCTRL: i32 = 0x80 + 0x1D;
const KEY_RALT: i32 = 0x80 + 0x38;
const KEY_LALT: i32 = KEY_RALT;

// DOOM basic types (boolean), and max/min values.
/* doomtype.rs */

// Fixed point.
/* m_fixed.rs */

// Endianess handling.
/* m_swap.rs */

// Binary angles, sine/cosine/atan lookups.
/* tables.rs */

// Event type.
/* d_event.rs */

// Game function, skills.
/* g_game.rs */

// All external data is defined here.
/* doomdata.rs */

// All important printed strings. Language selection (message strings).
/* d_strings.rs */

// Player is a special actor.
/* struct Player */

/*
The original source has omitted includes, but I'll just put them here for completeness...
    d_items.rs
    d_player.rs
    p_mobj.rs
    d_net.rs
*/

// PLAY
/* p_tick.rs */

// Header, generated by sound utility. The utility was written by Dave Taylor.
/* sounds.rs */
