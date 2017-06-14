
from render.curses_interface import screen



def main():
    
    with screen() as ikky:
        ikky.write_string( 0, 0, "blah" )
        ikky.get_character()


if __name__ == '__main__':
    main()


