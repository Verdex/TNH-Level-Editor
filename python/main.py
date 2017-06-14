
from render.curses_screen import curses_screen



def main():
    
    with curses_screen() as ikky:
        ikky.write_string( 0, 0, "blah" )
        ikky.get_character()


if __name__ == '__main__':
    main()


