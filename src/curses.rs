
use std::ffi::CString;
use std::os::raw::c_char;

#[link( name = "ncurses" )]
extern {
    pub fn initscr();
    pub fn refresh();
    pub fn endwin();
    pub fn getch() -> i32;
    pub fn printw( format : *const c_char, ... );
}

extern {
    pub static error : i32;
}


