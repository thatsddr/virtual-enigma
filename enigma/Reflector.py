class Reflector:
    def __init__(self, ukw_type):
        self.type = ukw_type
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
    
    def reflect(self, letter, zusStep):
        return self.type[(26 - (zusStep - self.alphabet.index(letter))) % 26]