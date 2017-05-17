
use std::os::raw::c_char;

#[link( name = "ncurses" )]
extern {
    pub fn initscr();
    pub fn refresh();
    pub fn endwin();
    pub fn getch() -> i32;
    pub fn printw( format : *const c_char, ... );
    pub fn cbreak();
    pub fn noecho();
    pub fn keypad();
    pub fn clear();
    pub fn mvprintw( y : i32, x : i32, format : *const c_char, ... );
}

extern {
    pub static error : i32;
    fn move_wrapper( y : i32, x : i32 );
}

pub unsafe fn cursor_move( y : i32, x : i32 ) {
    move_wrapper( y, x );
}


