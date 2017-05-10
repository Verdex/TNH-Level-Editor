
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let out = env::var( "OUT_DIR" ).unwrap();

    Command::new("clang").args( &[ "-arch", "x86_64", "-c", "src/curses_wrapper.c", "-o" ] )
                         .arg( &format!( "{}/curses_wrapper.o", out ) )
                         .status()
                         .unwrap();

    Command::new( "libtool" ).args( &[ "-dynamic", "-lncurses", "curses_wrapper.o", "-o", "libcurses_wrapper.dylib" ] )
                             .current_dir( &Path::new( &out ) )
                             .status()
                             .unwrap();

    println!( "cargo:rustc-link-search=native={}", out );
    println!( "cargo:rustc-link-lib=dylib=curses_constants" );

}
