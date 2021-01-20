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

class Reflector:
    def __init__(self, ukw_type, alphabet):
        self.type = ukw_type
        self.alphabet = alphabet
    
    def reflect_letter(self, letter):
        index = alphabet.index(letter)
        return self.type[index]

class Rotor:
    def __init__(self, rotor, alphabet, starting_pos=0, ringstellung=0, rotating=True):
        self.alphabet = alphabet
        self.rotor = rotor
        self.dotpos = self.rotor.index("a")
        self.ringstellung = ringstellung
        self.rotating = True
        if self.ringstellung != 0 and self.ringstellung > 0 and self.ringstellung <= 25:
            self.apply_ringstellung()
        if starting_pos >= 0 and starting_pos <= 25:
            self.rotate(starting_pos)
    
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
        print(f"rotor after applying ringstellung: {self.rotor}, with dot position: {self.dotpos}")
    
    def rotate(self, step=1):
        if self.rotating == True: #checks if the rotor is not the zusatzwalde
            self.rotor = self.rotor[-step:] + self.rotor[:-step]
        print(f"current rotor posiiton after rotation of {step}: {self.rotor}")
    
    def return_letter(self, letter):
        index = self.alphabet.index(letter)
        return self.rotor[index]

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
        

r = Reflector(reflectors["UKW-B-thin"], alphabet)
print(r.reflect_letter("a"))

p = Plugboard(["fa", "hb", "er", "gv", "ip", "nz", "qt", "uj", "xm", "wl"], "the quick broen fox jumps over the lazy dog")
p.apply()
print(p.text)


t = Rotor(rotors["I"], alphabet, ringstellung=1, starting_pos=12)
t.rotate()
print(t.return_letter("a"))
print(t.return_letter("c"))
