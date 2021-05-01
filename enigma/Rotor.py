class Rotor:
    def __init__(self, confObj):
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
        self.rotor = confObj.get("rotor").get("sequence")
        self.ringstellung = confObj.get("ringstellung", 0)
        self.starting_pos = confObj.get("starting_pos", 0)
        self.steps = 0

        if self.ringstellung != 0 and self.ringstellung > 0 and self.ringstellung <= 25:
            self.apply_ringstellung()

        # get the position of the notches after the ringstellung
        self.notches = [self.rotor[self.alphabet.index(n)] for n in confObj.get("rotor").get("notches")]

        if self.starting_pos > 0 and self.starting_pos <= 25:
            self.rotate(self.starting_pos)   
    
    def apply_ringstellung(self):
        # Shift the configuration of the rotor backwards
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
    
    def left_mov(self, letter, prev=None):
         #encrypts the letter that goes from the input towards the reflectos
        index = self.alphabet.index(letter)
        if prev == None:
            return self.rotor[index]
        else:
            # 1. position of the previous - position of the letter in the alphabet
            # 2. Make sure it is positive
            # 3. Make sure it is < 26
            # 4. Get the letter in teh rotor at that position
            return self.rotor[(26 - (prev - index)) % 26]
         
    def right_mov(self, letter):
        #encrypts the letter that goes from the reflector towards the output
        # 1. Get the position of the letter in the alphabet and add the steps of the rotor. Make sure that it is < 26.
        # 2. Get the letter with the obtained position in the alphabet.
        # 3. Get the index of that letter in the rotor.
        # 4. get the letter in the alphabet with that index.
        return self.alphabet[self.rotor.index(self.alphabet[(self.alphabet.index(letter) + self.steps) % 26])]