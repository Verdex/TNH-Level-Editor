
from render.curses_interface import screen
from util.input_engine import input_handler
from util.cursor import loc

# TODO make letter action register thingy

def main():
    
    input_h = input_handler() 

    cursor = loc()

    input_h.register( 'j', loc.inc_y )
    input_h.register( 'k', loc.dec_y )
    input_h.register( 'l', loc.inc_x )
    input_h.register( 'h', loc.dec_x )

    input_h.register( 'J', loc.inc_screen_y )
    input_h.register( 'K', loc.dec_screen_y )
    input_h.register( 'L', loc.inc_screen_x )
    input_h.register( 'H', loc.dec_screen_x )

    with screen() as s:
        c = s.get_character() 
        while c != ord( 'q' ):

            input_h.handle_input( c )
        
            if c == ord( 'c' ):
                pass

            elif c == ord( 'r' ):
                pass

            else:
                pass


            s.clear()
            s.move_cursor( cursor.x, cursor.y )
            s.refresh()

            c = s.get_character()




if __name__ == '__main__':
    main()


