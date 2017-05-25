
mod curses;

use std::ffi::CString;

use curses::*;


enum DrawMe {
    Rec { width : i32, height : i32, x : i32, y : i32 },
    /*
    Character
    */
}

enum Mode { Normal, EnterRec, MoveRec }

fn inc_width( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width + 1, height: height, x: x, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn inc_height( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height + 1, x: x, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn dec_width( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width - 1, height: height, x: x, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn dec_height( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height - 1, x: x, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn inc_x( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x + 1, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn inc_y( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x, y: y + 1 },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn dec_x( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x - 1, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn dec_y( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x, y: y - 1 },
        _ => panic!( "inc width encounters non-rec" ),
    }
}
fn mod_rec<F>( maybe_d : Option<DrawMe>, f : F ) -> Option<DrawMe> 
    where F : Fn( DrawMe ) -> DrawMe {

    match maybe_d {
        Some( d ) => Some( f( d ) ),
        None => { None },
    }
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
        mvprintw( y + h2 + UserY, x + w2 + 1 + UserX, other_d.as_ptr() );
        while w2 > 0 {
            mvprintw( y + h2 + UserY, x + w2 + UserX, d.as_ptr() );
            w2 -= 1;
        }
        mvprintw( y + h2 + UserY, x + UserX, other_d.as_ptr() );
        h2 -= 1;
    }
}

static mut UserX : i32 = 0;
static mut UserY : i32 = 0; 

fn main() {
    let mut x = 0;
    let mut y = 0;
    unsafe {
        initscr();
        cbreak();
        noecho();


        let mut c = getch();
        let mut shapes : Vec<DrawMe> = vec! [];
        let mut mode : Mode = Mode::Normal;
        let mut active_rec : Option<DrawMe> = None;


        while c as u8 as char != 'q' {
            let c2 = c as u8 as char;
            
            match c2 {
                'r' => {
                    match mode {
                        Mode::Normal => {
                            active_rec = Some( DrawMe::Rec { width: 0, height: 0, x: x - UserX, y: y - UserY });
                            mode = Mode::EnterRec;
                        },
                        Mode::EnterRec => {
                            mode = Mode::MoveRec; 
                        },
                        Mode::MoveRec => {
                            mode = Mode::Normal;
                            shapes.push( active_rec.unwrap() );
                            active_rec = None;
                        },
                    }
                },
                'j' => {
                    match mode {
                        Mode::Normal => {
                            y+=1;
                        },
                        Mode::EnterRec => {
                            active_rec = mod_rec( active_rec, inc_height );
                        },
                        Mode::MoveRec => {
                            active_rec = mod_rec( active_rec, inc_y );
                        },
                    }
                },
                'k' => {
                    match mode {
                       Mode::Normal => {
                           y-=1;
                       },
                       Mode::EnterRec => {
                            active_rec = mod_rec( active_rec, dec_height );
                       },
                       Mode::MoveRec => {
                            active_rec = mod_rec( active_rec, dec_y );
                       },
                    }
                },
                'l' => {
                    match mode {
                        Mode::Normal => {
                            x+=1;
                        },
                        Mode::EnterRec => {
                            active_rec = mod_rec( active_rec, inc_width );
                        },
                        Mode::MoveRec => {
                            active_rec = mod_rec( active_rec, inc_x );
                        },
                    }
                },
                'h' => {
                    match mode {
                        Mode::Normal => {
                            x-=1;
                        },
                        Mode::EnterRec => {
                            active_rec = mod_rec( active_rec, dec_width );
                        },
                        Mode::MoveRec => {
                            active_rec = mod_rec( active_rec, dec_x );
                        },
                    }
                },
                'J' => {
                    match mode {
                        Mode::Normal => {
                            UserY += 1;
                        },
                        _ => {},
                    }
                },
                'K' => {
                    match mode {
                        Mode::Normal => {
                            UserY -= 1;
                        },
                        _ => {},
                    }
                },
                'L' => {
                    match mode {
                        Mode::Normal => {
                            UserX += 1;
                        },
                        _ => {},
                    }
                },
                'H' => {
                    match mode {
                        Mode::Normal => {
                            UserX -= 1;
                        },
                        _ => {},
                    }
                },
                _ => (),
            }

            clear();

            match active_rec {
                Some( ref shape ) => render_shape( &shape ),
                None => {},
            }

            for s in &shapes {
                render_shape( s );
            }
            
            cursor_move( y, x );
            refresh();

            c = getch();
        }


        endwin();
    }
}

