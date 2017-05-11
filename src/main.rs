

mod curses;

use std::ffi::CString;

use curses::*;

fn main() {
    let mut x = 0;
    let mut y = 0;
    unsafe {
        initscr();
        let mut c = getch();

        /* while c != final {
            if c == up then
                x++
            cursor_move( 10, 10 );
        */

        refresh();
        getch();
        endwin();
    }
}
