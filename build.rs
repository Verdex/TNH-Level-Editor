
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let out = env::var( "OUT_DIR" ).unwrap();

    Command::new("clang").args( &[ "-arch", "x86_64", "-c", "src/curses_constants.c", "-o" ] )
                         .arg( &format!( "{}/curses_constants.o", out ) )
                         .status()
                         .unwrap();

    Command::new( "libtool" ).args( &[ "-static", "curses_constants.o", "-o", "libcurses_constants.a" ] )
                             .current_dir( &Path::new( &out ) )
                             .status()
                             .unwrap();

    println!( "cargo:rustc-link-search=native={}", out );
    println!( "cargo:rustc-link-lib=static=curses_constants" );

}
