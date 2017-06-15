

class input_handler:
    
    def __init__(self):
        self.actions = {} 
        self.default = []

    def register(self, char, action):
        a = self.actions
        if char in a:
            a[char].append( action )
        else:
            a[char] = [ action ]

    def register_default(self, action):
        self.default.append( action )

    def handle_input(self, input):
        a = self.actions
        if input in a:
            for alet in a[ input ]:
                alet( input )
        else:
            for d in self.default:
                d( input )


