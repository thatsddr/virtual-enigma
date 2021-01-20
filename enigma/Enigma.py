from Plugboard import Plugboard
from Reflector import Reflector
from Rotor import Rotor

class Enigma:
    def __init__(self, configObj, text):
        self.rotor1 = Rotor(configObj["rotors"]["rotor1"])
        self.rotor2 = Rotor(configObj["rotors"]["rotor2"])
        self.rotor3 = Rotor(configObj["rotors"]["rotor3"])
        self.zusatzwalze = Rotor(configObj["rotors"]["zusatzwalze"])
        self.reflector = Reflector(configObj["reflector"])
        self.text = text.strip().replace(" ", "").lower()
        self.plugboardConfig = configObj.get("plugboard", [])
        self.plugboard = Plugboard(self.plugboardConfig, self.text)
        self.iteration = 1

    def rotateRotors(self):
        self.rotor1.rotate()
        if self.iteration % 26 == 0:
            self.rotor2.rotate()
        if self.iteration % 26 * 26 == 0:
            self.rotor3.rotate()
        self.iteration += 1

    def encryptLetter(self, letter):

        value = self.rotor1.return_reverse(
            self.rotor2.return_reverse(
                self.rotor3.return_reverse(
                    self.zusatzwalze.return_reverse(
                        self.reflector.reflect_letter(
                            self.zusatzwalze.return_letter(
                                self.rotor3.return_letter(
                                    self.rotor2.return_letter(
                                        self.rotor1.return_letter(letter)
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
        return value

    def run(self):
        # step 1: plugboard
        self.text = self.plugboard.apply()
        temp = []
        for i in range(1, len(self.text) +1):
            self.rotateRotors()
            temp.append(self.encryptLetter(self.text[i-1]))
            if i%4 == 0:
                temp.append(" ")
        tempstr = "".join(temp)
        self.plugboard2 = Plugboard(self.plugboardConfig, tempstr)
        return(self.plugboard2.apply())