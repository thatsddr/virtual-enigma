from Plugboard import Plugboard
from Reflector import Reflector
from Rotor import Rotor
from Settings import Settings


class Enigma:
    def __init__(self, configObj):
        self.settings = Settings()
        self.settings.configure(configObj)

        self.rotor1 = Rotor(self.settings.get_rotor("rotor1"))
        self.rotor2 = Rotor(self.settings.get_rotor("rotor2"))
        self.rotor3 = Rotor(self.settings.get_rotor("rotor3"))
        self.zusatzwalze = Rotor(self.settings.get_rotor("zusatzwalze"))

        self.reflector = Reflector(self.settings.get_reflector())
        self.plugboardConfig = self.settings.plugboard()
        self.plugboard = Plugboard(self.plugboardConfig)

    def rotateRotors(self):
        self.rotor1.rotate()
        if self.rotor1.rotor[0] in self.rotor1.notches:
            self.rotor2.rotate()
        if self.rotor2.rotor[0] in self.rotor2.notches:
            self.rotor3.rotate()

    def encryptLetter(self, letter):
        v1 = self.rotor1.left_mov(letter)
        v2 = self.rotor2.left_mov(v1, self.rotor1.steps)
        v3 = self.rotor3.left_mov(v2, self.rotor2.steps)
        v4 = self.zusatzwalze.left_mov(v3, self.rotor3.steps)
        v5 = self.reflector.reflect(v4, self.zusatzwalze.steps)
        v6 = self.zusatzwalze.right_mov(v5)
        v7 = self.rotor3.right_mov(v6)
        v8 = self.rotor2.right_mov(v7)
        v9 = self.rotor1.right_mov(v8)
        return v9
    
    def log_encrypt(self, letter):
         v1=self.rotor1.left_mov(letter)
         print(f"{self.rotor1.alphabet.upper()}\n{self.rotor1.rotor.upper()}\nAfter rotor 1: {v1.upper()}\n")
         v2= self.rotor2.left_mov(v1, self.rotor1.steps)
         print(f"{self.rotor2.alphabet.upper()}\n{self.rotor2.rotor.upper()}\nAfter rotor 2: {v2.upper()}\nprev: {self.rotor1.steps}\n")
         v3 = self.rotor3.left_mov(v2, self.rotor2.steps)
         print(f"{self.rotor3.alphabet.upper()}\n{self.rotor3.rotor.upper()}\nAfter rotor 3: {v3.upper()}\nprev: {self.rotor2.steps}\n")
         v4 = self.zusatzwalze.left_mov(v3, self.rotor3.steps)
         print(f"{self.zusatzwalze.alphabet.upper()}\n{self.zusatzwalze.rotor.upper()}\nAfter zusatzwalze: {v4.upper()}\nprev: {self.rotor3.steps}\n")
         v5 = self.reflector.reflect(v4, self.zusatzwalze.steps)
         print(f"{self.rotor1.alphabet}\n{self.reflector.type}\nAfter reflector: {v5.upper()}\n")
         v6 = self.zusatzwalze.right_mov(v5)
         print(f"{self.zusatzwalze.rotor.upper()}\n{self.zusatzwalze.alphabet.upper()}\nAfter inversed zusatzwalze: {v6.upper()}\n")
         v7 = self.rotor3.right_mov(v6)
         print(f"{self.rotor3.rotor.upper()}\n{self.rotor3.alphabet.upper()}\nAfter inversed rotor 3: {v7.upper()}\n")
         v8 = self.rotor2.right_mov(v7)
         print(f"{self.rotor2.rotor.upper()}\n{self.rotor2.alphabet.upper()}\nAfter inversed rotor 2: {v8.upper()}\n")
         v9 = self.rotor1.right_mov(v8)
         print(f"{self.rotor1.rotor.upper()}\n{self.rotor1.alphabet.upper()}\nAfter inversed rotor 1: {v9.upper()}\n")
         print("--------------------------")
         return v9


    def run(self, text, logging=False):
        self.text = self.plugboard.apply(text.strip().replace(" ", "").lower())
        temp = []
        temp_length = 0
        for i in range(0, len(self.text)):
            if self.text[i] in self.settings.alphabet:
                temp_length += 1
                self.rotateRotors()
                if logging==False:
                    temp.append(self.encryptLetter(self.text[i]))
                else:
                    temp.append(self.log_encrypt(self.text[i]))
                if temp_length % 4 == 0:
                    temp_length = 0
                    temp.append(" ")
        tempstr = "".join(temp)
        return self.plugboard.apply(tempstr)
