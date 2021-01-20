class Reflector:
    def __init__(self, ukw_type):
        self.type = ukw_type
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
    
    def reflect_letter(self, letter):
        index = self.alphabet.index(letter)
        return self.type[index]