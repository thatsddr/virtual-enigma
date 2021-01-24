class Rotor:
    def __init__(self, confObj):
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
        self.rotor = confObj.get("rotor").get("sequence")
        self.ringstellung = confObj.get("ringstellung", 0)
        self.starting_pos = confObj.get("starting_pos", 0)
        self.steps = 0
        if self.ringstellung != 0 and self.ringstellung > 0 and self.ringstellung <= 25:
            self.apply_ringstellung()
        if self.starting_pos > 0 and self.starting_pos <= 25:
            self.rotate(self.starting_pos)  
        self.notches = [self.rotor[self.alphabet.index(n)] for n in confObj.get("rotor").get("notches")]
    
    def apply_ringstellung(self):
        self.rotor = self.rotor[-self.ringstellung:] + self.rotor[:-self.ringstellung]
        self.steps = 26 - self.ringstellung
    
    def updateSteps(self, num=1):
        if self.steps + num > 25:
            self.steps = (self.steps + num) % 26
        else:
            self.steps += num

    def rotate(self, step=1):
        self.rotor = self.rotor[step:] + self.rotor[:step]
        self.updateSteps(step)
        return self.rotor
    
    #encrypts the letter that goes from the input towards the reflectos
    def left_mov(self, letter, prev=None):
        index = self.alphabet.index(letter)
        if prev == None:
            return self.rotor[index]
        else:
            return self.rotor[(26 - (prev - index)) % 26]
        
    #encrypts the letter that goes from the reflector towards the output
    def right_mov(self, letter):
        return self.alphabet[self.rotor.index(self.alphabet[(self.alphabet.index(letter) + self.steps) % 26])]