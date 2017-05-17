
mod curses;

use std::ffi::CString;

use curses::*;


enum DrawMe {
    //Circle { radius : i32, x : i32, y : i32 },
    Rec { width : i32, height : i32, x : i32, y : i32 },
    /*VHall { length : i32, x : i32, y : i32 },
    HHall { length : i32, x : i32, y : i32 },*/
    // TODO add color
    // TODO add background color
    // TODO add point case
    // TODO add overlap
    // TODO can probably have a struct represent common items (x,y, color, etc)
}

unsafe fn render_shape( s : &DrawMe ) {
    match s {
        &DrawMe::Rec { width, height, x, y } => render_rec( width, height, x, y ),
        _ => (),
    }
}

unsafe fn render_rec( w : i32, h : i32, x : i32, y : i32 ) {
    let c = CString::new( "." ).unwrap();

    let mut h2 = h;
    while h2 > 0 {
        let mut w2 = w;
        while w2 > 0 {
            mvprintw( y + h2, x + w2, c.as_ptr() );
            w2 -= 1;
        }
        h2 -= 1;
    }
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
            let c2 = c as u8 as char;
            
            match c2 {
                'j' => {
                    y+=1;
                    cursor_move( y, x );
                },
                'k' => {
                    y-=1;
                    cursor_move( y, x );
                },
                'l' => {
                    x+=1;
                    cursor_move( y, x );
                },
                'h' => {
                    x-=1;
                    cursor_move( y, x );
                },
                _ => (),
            }
            refresh();
            c = getch();
        }

        endwin();
    }
}

