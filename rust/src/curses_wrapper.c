
#include<ncurses.h>

const int error = ERR;

void move_wrapper( int y, int x ) {
    move( y, x );
}
