class Rotor:
    def __init__(self, confObj):
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
        self.rotor = confObj.get("rotor")
        self.dotpos = self.rotor.index("a")
        self.ringstellung = confObj.get("ringstellung", 0)
        self.starting_pos = confObj.get("starting_pos", 0)
        self.rotating = confObj.get("rotating", True)
        if self.ringstellung != 0 and self.ringstellung > 0 and self.ringstellung <= 25:
            self.apply_ringstellung()
        if self.starting_pos >= 0 and self.starting_pos <= 25:
            self.rotate(self.starting_pos)
    
    def apply_ringstellung(self):
        new_wiring = []
        for char in self.rotor:
            new_wiring.append(self.alphabet[(self.alphabet.index(char) + self.ringstellung) % 26])
            self.dotpos = (self.dotpos + self.ringstellung) % 26
        j = 0
        new_wiring = "".join(new_wiring)
        while new_wiring[self.dotpos] != self.alphabet[self.ringstellung]:
            j += 1;
            new_wiring = new_wiring[-1:] + new_wiring[:-1]
        self.rotor = new_wiring
        #print(f"rotor after applying ringstellung: {self.rotor}, with dot position: {self.dotpos}")
    
    def rotate(self, step=1):
        if self.rotating == True: #checks if the rotor is not the zusatzwalze
            self.rotor = self.rotor[-step:] + self.rotor[:-step]
        #print(f"current rotor posiiton after rotation of {step}: {self.rotor}")
    
    def return_letter(self, letter):
        index = self.alphabet.index(letter)
        return self.rotor[index]
    
    def return_reverse(self, letter):
        index = self.rotor.index(letter)
        return self.alphabet[index]
