

class loc:

    def __init__(self):
        self.y = 0
        self.x = 0
        self.sx = 0
        self.sy = 0

    def inc_y(self):
        self.y += 1

    def dec_y(self):
        self.y -= 1

    def inc_x(self):
        self.x += 1

    def dec_x(self):
        self.x -= 1

    def inc_screen_y(self):
        self.sy += 1

    def dec_screen_y(self):
        self.sy -= 1

    def inc_screen_x(self):
        self.sx += 1

    def dec_screen_x(self):
        self.sx -= 1


