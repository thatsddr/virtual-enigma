class Reflector:
    def __init__(self, ukw_type):
        self.type = ukw_type
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
    
    def reflect(self, letter, zusStep):
        # 1. get the position of the zusatzwalze - the position of the letter in the alphabet
        # 2. Make sure it is positive an < 26
        # 3. return the letter at that position in the reflector
        return self.type[(26 - (zusStep - self.alphabet.index(letter))) % 26]