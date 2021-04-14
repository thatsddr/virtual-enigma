class Settings:
    def __init__(self):
        # Rotors = Moveable wheels
        self.rotors = {
            "I": {"sequence": "ekmflgdqvzntowyhxuspaibrcj", "notches": ["r"]},
            "II": {"sequence": "ajdksiruxblhwtmcqgznpyfvoe", "notches": ["f"]},
            "III": {"sequence": "bdfhjlcprtxvznyeiwgakmusqo", "notches": ["w"]},
            "IV": {"sequence": "esovpzjayquirhxlnftgkdcmwb", "notches": ["k"]},
            "V": {"sequence": "vzbrgityupsdnhlxawmjqofeck", "notches": ["a"]},
            "VI": {"sequence": "jpgvoumfyqbenhzrdkasxlictw", "notches": ["a", "n"]},
            "VII": {"sequence": "nzjhgrcxmyswboufaivlpekqdt", "notches": ["a", "n"]},
            "VIII": {"sequence": "fkqhtlxocbjspdzramewniuygv", "notches": ["a", "n"]},
        }
        # Reflector = Reflect the letter back for a second round of encryption
        self.reflectors = {
            "UKW-B-thin": "enkqauywjicopblmdxzvfthrgs",
            "UKW-C-thin": "rdobjntkvehmlfcwzaxgyipsuq",
        }
        # zusatzwalze = left-most WHEEL, does not move
        self.zusatzwalze = {      
            "beta": {"sequence": "leyjvcnixwpbqmdrtakzgfuhos", "notches": []},
            "gamma": {"sequence": "fsokanuerhmbtiycwlqpzxvgjd", "notches": []},
        }
        self.alphabet = "abcdefghijklmnopqrstuvwxyz"
        self.config = None
    
    def to_position(self, param):
        if type(param) is str and param in self.alphabet:
            return  self.alphabet.index(param.lower())
        elif type(param) is int and param > 0 and param < 27:
            return param - 1
        else:
            raise Exception("Invalid param. '" + str(param) + "' is not a valid argument")

    
    def configure(self, configObj):
        self.config = {
        "reflector": configObj["reflector"],
        "plugboard": configObj["plugboard"],
        "rotors": {
            "zusatzwalze": {
                "rotor": configObj["zus"].get("rot"),
                "starting_pos": self.to_position(configObj["zus"].get("pos")),
                "ringstellung": self.to_position(configObj["zus"].get("ring")),
            },
            "rotor3": {
                "rotor": configObj["rot3"].get("rot"),
                "starting_pos": self.to_position(configObj["rot3"].get("pos")),
                "ringstellung": self.to_position(configObj["rot3"].get("ring")),
            },
            "rotor2": {
                "rotor": configObj["rot2"].get("rot"),
                "starting_pos": self.to_position(configObj["rot2"].get("pos")),
                "ringstellung": self.to_position(configObj["rot2"].get("ring")),
            },
            "rotor1": {
                "rotor": configObj["rot1"].get("rot"),
                "starting_pos": self.to_position(configObj["rot1"].get("pos")),
                "ringstellung": self.to_position(configObj["rot1"].get("ring")),
            },
        },
    }

    def get_rotor(self, rotorName):
        if self.config != None:
            if (rotorName in ["rotor1", "rotor2", "rotor3"]):
                return {
                    "rotor": self.rotors[self.config["rotors"][rotorName]["rotor"]],
                    "starting_pos": self.config["rotors"][rotorName]["starting_pos"],
                    "ringstellung": self.config["rotors"][rotorName]["ringstellung"] 
                }
            elif (rotorName == "zusatzwalze"):
                return {
                    "rotor": self.zusatzwalze[self.config["rotors"][rotorName]["rotor"]],
                    "starting_pos": self.config["rotors"][rotorName]["starting_pos"],
                    "ringstellung": self.config["rotors"][rotorName]["ringstellung"] 
                }
        return None

    def get_reflector(self):
        if self.config != None and self.config.get("reflector"):
            return self.reflectors[self.config["reflector"]]
        return None
    
    def plugboard(self):
        if self.config != None:
            return self.config.get("plugboard", [])
        return None