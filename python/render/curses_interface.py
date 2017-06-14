

import curses

class helper:
    def __init__(self, screen):
        self.screen = screen

    def get_character(self):
        return self.screen.getch()

    def write_string( self, x, y, string ):
        self.screen.addstr(y,x,string)

    def clear( self ):
        self.screen.clear()

    def refresh( self ):
        self.screen.refresh()


class screen:

    def __enter__(self):
        self.screen = curses.initscr()
        self.screen.keypad( 1 )
        curses.cbreak()
        curses.noecho()
        return helper( self.screen )


    def __exit__(self, type, value, traceback):
        self.screen.keypad( 0 )
        curses.echo()
        curses.nocbreak()
        curses.endwin()


