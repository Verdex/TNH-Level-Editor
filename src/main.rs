

mod curses;

use std::ffi::CString;

use curses::*;

fn main() {
    let mut x = 0;
    let mut y = 0;
    unsafe {
        initscr();
        cbreak();
        noecho();

        let mut c = getch();

        while c as u8 as char != 'q' {
            if c as u8 as char == 'j' {
                y+=1;
                cursor_move( y, x );
            } else if c as u8 as char == 'k' {
                y-=1;
                cursor_move( y, x );
            } else if c as u8 as char == 'l' {
               x+=1;
               cursor_move( y, x );
            } else if c as u8 as char == 'h' {
               x-=1;
               cursor_move( y, x );
            }
            refresh();
            c = getch();
        }

        endwin();
    }
}

