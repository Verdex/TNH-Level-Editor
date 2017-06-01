
mod curses;

use std::ffi::CString;

use std::env::args;
use std::path::Path;
use std::fs::File;

use curses::*;

// TODO delete active rectangle
// TODO save to file (just save shape coords and types, etc)
// TODO load file

// TODO use struct with common fields and use an enum for unshared ones
enum DrawMe {
    Rec { width : i32, height : i32, x : i32, y : i32 },
    Character { c : char, x : i32, y : i32 },
}

enum Mode { Normal, EnterRec, MoveRec, EnterCharacter }

fn inc_width( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width + 1, height: height, x: x, y: y },
        _ => panic!( "inc width encounters non-rec" ),
    }
}

fn inc_height( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height + 1, x: x, y: y },
        _ => panic!( "inc height encounters non-rec" ),
    }
}

fn dec_width( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width - 1, height: height, x: x, y: y },
        _ => panic!( "dec width encounters non-rec" ),
    }
}

fn dec_height( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height - 1, x: x, y: y },
        _ => panic!( "dec height encounters non-rec" ),
    }
}

fn inc_x( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x + 1, y: y },
        _ => panic!( "inc x encounters non-rec" ),
    }
}

fn inc_y( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x, y: y + 1 },
        _ => panic!( "inc y encounters non-rec" ),
    }
}

fn dec_x( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x - 1, y: y },
        _ => panic!( "dec x encounters non-rec" ),
    }
}

fn dec_y( d : DrawMe ) -> DrawMe {
    match d {
        DrawMe::Rec { width, height, x, y } => DrawMe::Rec { width: width, height: height, x: x, y: y - 1 },
        _ => panic!( "dec y encounters non-rec" ),
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
        &DrawMe::Character { c, x, y } => render_character( c, x, y ),
        _ => (),
    }
}

unsafe fn render_character( c : char, x : i32, y : i32 ) {
    mvaddch( y + UserY, x + UserX, c );   
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

    let mut level_file : Option<File> = None;
    for a in args() {
        let p = Path::new( &a );
        if p.exists() {
            let fr = File::open( p );
            match fr {
                Ok( f ) => { level_file = Some( f ); },
                _ => panic!( "error opening file"),
            }
            break;
        }
    }

    // TODO check to make sure the file exists
    // TODO parse the contents
    

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
                'c' => {
                    match mode {
                        Mode::Normal => {
                            mode = Mode::EnterCharacter;
                        },
                        Mode::EnterRec => {},
                        Mode::MoveRec => {},
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'c', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                    }
                },
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
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'r', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
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

                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'j', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
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

                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'k', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
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
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'l', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
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
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'h', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                    }
                },
                'J' => {
                    match mode {
                        Mode::Normal => {
                            UserY += 1;
                        },
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'J', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                        _ => {},
                    }
                },
                'K' => {
                    match mode {
                        Mode::Normal => {
                            UserY -= 1;
                        },
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'K', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                        _ => {},
                    }
                },
                'L' => {
                    match mode {
                        Mode::Normal => {
                            UserX += 1;
                        },
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'L', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                        _ => {},
                    }
                },
                'H' => {
                    match mode {
                        Mode::Normal => {
                            UserX -= 1;
                        },
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: 'H', x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                        _ => {},
                    }
                },
                c3 @ _ => {
                    match mode {
                        Mode::EnterCharacter => {
                            shapes.push( DrawMe::Character{ c: c3, x: x - UserX, y: y - UserY } );
                            mode = Mode::Normal;
                        },
                        _ => {},
                    }
                },

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

