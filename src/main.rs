
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
    let dot = CString::new( "." ).unwrap();
    let h_line = CString::new( "-" ).unwrap();
    let v_line = CString::new( "|" ).unwrap();
    let plus = CString::new( "+" ).unwrap();

    let mut h2 = h + 2;
    while h2 > 0 {
        let d : &CString = if h2 == h + 2 {
            &h_line
        } else if h2 == 1 {
            &h_line
        } else {
            &dot
        };
        let mut w2 = w;

        let other_d : &CString = if h2 == h + 2 {
            &plus
        } else if h2 == 1 {
            &plus
        } else {
            &v_line
        };
        mvprintw( y + h2, x + w2 + 1, other_d.as_ptr() );
        while w2 > 0 {
            mvprintw( y + h2, x + w2, d.as_ptr() );
            w2 -= 1;
        }
        mvprintw( y + h2, x, other_d.as_ptr() );
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
        let mut shapes : Vec<DrawMe> = vec! [];

        while c as u8 as char != 'q' {
            let c2 = c as u8 as char;
            
            match c2 {
                /*'c' => {
                    shapes.push( DrawMe::Circle { radius: 5, x: x, y: y } );
                },*/
                'r' => {
                    shapes.push( DrawMe::Rec { width: 5, height: 8, x: x, y: y } );
                },
                'j' => {
                    y+=1;
                },
                'k' => {
                    y-=1;
                },
                'l' => {
                    x+=1;
                },
                'h' => {
                    x-=1;
                },
                _ => (),
            }

            for s in &shapes {
                render_shape( s );
            }
            
            cursor_move( y, x );
      //      clear();
            refresh();

            c = getch();
        }


        endwin();
    }
}

