
from render.curses_interface import screen


# TODO make letter action register thingy

def main():
    
    with screen() as s:
        c = s.get_character() 
        while c != ord( 'q' ):
        
            if c == ord( 'c' ):
                pass

            elif c == ord( 'r' ):
                pass
            elif c == ord( 'j' ):
                pass

            elif c == ord( 'k' ):
                pass

            elif c == ord( 'l' ):
                pass

            elif c == ord( 'h' ):
                pass

            elif c == ord( 'J' ):
                pass

            elif c == ord( 'K' ):
                pass

            elif c == ord( 'L' ):
                pass

            elif c == ord( 'H' ):
                pass


            else:
                pass


            s.clear()
            s.move_cursor( 10, 10 )
            s.refresh()

            c = s.get_character()




if __name__ == '__main__':
    main()


