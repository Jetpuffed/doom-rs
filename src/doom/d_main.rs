//! Original file: d_main.c / d_main.h
//! 
//! Description: DOOM main program (D_DoomMain) and game loop (D_DoomLoop),
//! plus functions to determine game mode (shareware, registered), parse
//! command line parameters, configure game parameters (turbo), and call
//! the startup functions.

const BG_COLOR: i32 = 7;
const FG_COLOR: i32 = 8;

/*
    Module completion status:
        doomdef.rs      YES
        doomstat.rs     NO
        dstrings.rs     NO
        sounds.rs       NO
        z_zone.rs       NO
        w_wad.rs        NO
        s_sound.rs      NO
        v_video.rs      NO
        f_finale.rs     NO
        f_wipe.rs       NO
        m_argv.rs       NO
        m_misc.rs       NO
        m_menu.rs       NO
        i_system.rs     NO
        i_sound.rs      NO
        i_video.rs      NO
        g_game.rs       NO
        hu_stuff.rs     NO
        wi_stuff.rs     NO
        st_stuff.rs     NO
        am_map.rs       NO
        p_setup.rs      NO
        r_local.rs      NO
*/
