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


    def run(self, text):
        self.text = self.plugboard.apply(text.strip().replace(" ", "").lower())
        temp = []
        temp_length = 0
        for i in range(0, len(self.text)):
            if self.text[i] in self.settings.alphabet:
                temp_length += 1
                self.rotateRotors()
                temp.append(self.encryptLetter(self.text[i]))
                if temp_length % 4 == 0:
                    temp_length = 0
                    temp.append(" ")
        tempstr = "".join(temp)
        return self.plugboard.apply(tempstr)
