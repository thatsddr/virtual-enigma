# input -> plugboard -> move rotors -> encrypt -> show

alphabet = "abcdefghijklmnopqrstuvwxyz"

#reflect the letter back for a second round of encryption
reflectors = {
    "UKW-B-thin": "enkqauywjicopblmdxzvfthrgs",
    "UKW-C-thin": "rdobjntkvehmlfcwzaxgyipsuq",
}

#moveable wheels
rotors = {
    "I": "ekmflgdqvzntowyhxuspaibrcj",
    "II": "ajdksiruxblhwtmcqgznpyfvoe",
    "III": "bdfhjlcprtxvznyeiwgakmusqo",
    "IV": "esovpzjayquirhxlnftgkdcmwb",
    "V": "vzbrgityupsdnhlxawmjqofeck",
    "VI": "jpgvoumfyqbenhzrdkasxlictw",
    "VII": "nzjhgrcxmyswboufaivlpekqdt",
    "VIII": "fkqhtlxocbjspdzramewniuygv",
}

#left-most WHEEL, does not move
zusatzwalze = {
    "beta": "leyjvcnixwpbqmdrtakzgfuhos",
    "gamma": "fsokanuerhmbtiycwlqpzxvgjd",
}

class Rotor:
    def __init__(self, rotor, alphabet, starting_pos=0, ringstellung=0, rotating=True):
        self.alphabet = alphabet
        self.rotor = rotor
        self.dotpos = self.rotor.index("a")
        self.ringstellung = ringstellung
        self.rotating = True
        if starting_pos >= 0 and starting_pos <= 25:
            self.pos = starting_pos
        if self.ringstellung != 0 and self.ringstellung > 0 and self.ringstellung <= 25:
            self.apply_ringstellung()
    
    def apply_ringstellung(self):
        new_wiring = []
        for char in self.rotor:
            new_wiring.append(self.alphabet[(self.alphabet.index(char) + self.ringstellung) % 25])
            self.dotpos = (self.dotpos + self.ringstellung) % 25
        j = 0
        new_wiring = "".join(new_wiring)
        while new_wiring[self.dotpos] != alphabet[self.ringstellung]:
            j += 1;
            new_wiring = new_wiring[-1:] + new_wiring[:-1]
        self.rotor = new_wiring
        print(self.rotor, self.dotpos)
    
    def rotate(self):
        if self.rotating == True: #checks if the rotor is not the zusatzwalde
            if self.pos < 25:
                self.pos += 1
            else:
                self.pos = 0
    
    def return_letter(self):
        return self.rotor[self.pos]

class Plugboard:
    def __init__(self, couples, text):
        self.list = couples
        if len(self.list) - 1 < 10:
            self.dict = {}
            self.dictify()
            self.text = text
        else:
            raise Exception("too many couples of letters ("+str(len(self.list))+")")
        
    def dictify(self):
        for c in self.list:
            if not self.dict.get(c[0]):
                self.dict[c[0]] = c[1]
                self.dict[c[1]] = c[0]
            else:
                self.dict = {}
                raise Exception("Duplicated letter", c[0])
    
    def apply(self):
        new_text = []
        for c in self.text:
            if self.dict.get(c):
                new_text.append(self.dict[c])
            else:
                new_text.append(c)
        self.text = "".join(new_text)
        print(self.text)


p = Plugboard(["fa", "hb", "er", "gv", "ip", "nz", "qt", "uj", "xm", "wl"], "era una gioranta soleggiata")
p.apply()
p.apply()


#t = Rotor(rotors["I"], alphabet, ringstellung=0, starting_pos=23)