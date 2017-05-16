
mod curses;

use std::ffi::CString;

use curses::*;


enum DrawMe {
    Circle { radius : i32, x : i32, y : i32 },
    /*Rec { width : i32, height : i32, x : i32, y : i32 },
    VHall { length : i32, x : i32, y : i32 },
    HHall { length : i32, x : i32, y : i32 },*/
}

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

