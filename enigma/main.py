from Enigma import Enigma

# reflect the letter back for a second round of encryption
reflectors = {
    "UKW-B-thin": "enkqauywjicopblmdxzvfthrgs",
    "UKW-C-thin": "rdobjntkvehmlfcwzaxgyipsuq",
}
# moveable wheels
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
# left-most WHEEL, does not move
zusatzwalze = {
    "beta": "leyjvcnixwpbqmdrtakzgfuhos",
    "gamma": "fsokanuerhmbtiycwlqpzxvgjd",
}

#config file, modify this
config = {
    "rotors": {
        #rotors 1-3 + zusatzwalze configuration: specify the rotor type, starting position ang ringstellung (ring starting position)
        "rotor1": {"rotor": rotors["VI"], "starting_pos": 9, "ringstellung": 12},
        "rotor2": {"rotor": rotors["I"], "starting_pos": 22, "ringstellung": 0},
        "rotor3": {"rotor": rotors["V"], "starting_pos": 14, "ringstellung": 7},
        "zusatzwalze": {
            "rotor": zusatzwalze["gamma"],
            "starting_pos": 8,
            "ringstellung": 0,
            "rotating": False,
        },
    },
    #choose the type of reflector
    "reflector": reflectors["UKW-C-thin"],
    #choose the plugboard configuration (lsit of max 10 elements)
    "plugboard": ["fa", "hb", "er", "gv", "ip", "nz", "qt", "uj", "xm", "wl"]
    
}

#define an instance of the enigma machine with the config dict and your text
e = Enigma(config, "The quick brown fox jumps over the lazy dog")
print(e.run()) #-> lzkn gulh hijm glft fauh ajff qdwg sics ach
