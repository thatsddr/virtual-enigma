from Enigma import Enigma

# reflect the letter back for a second round of encryption
reflectors = {
    "UKW-B-thin": "enkqauywjicopblmdxzvfthrgs",
    "UKW-C-thin": "rdobjntkvehmlfcwzaxgyipsuq",
}
# moveable wheels
rotors = {
    "I": {"sequence": "ekmflgdqvzntowyhxuspaibrcj", "notches": ["r"]},
    "II": {"sequence": "ajdksiruxblhwtmcqgznpyfvoe", "notches": ["f"]},
    "III": {"sequence": "bdfhjlcprtxvznyeiwgakmusqo", "notches": ["w"]},
    "IV": {"sequence": "esovpzjayquirhxlnftgkdcmwb", "notches": ["k"]},
    "V": {"sequence": "vzbrgityupsdnhlxawmjqofeck", "notches": ["a"]},
    "VI": {"sequence": "jpgvoumfyqbenhzrdkasxlictw", "notches": ["a", "n"]},
    "VII": {"sequence": "nzjhgrcxmyswboufaivlpekqdt", "notches": ["a", "n"]},
    "VIII": {"sequence": "fkqhtlxocbjspdzramewniuygv", "notches": ["a", "n"]},
}
# left-most WHEEL, does not move
zusatzwalze = {
    "beta": {"sequence": "leyjvcnixwpbqmdrtakzgfuhos", "notches": []},
    "gamma": {"sequence": "fsokanuerhmbtiycwlqpzxvgjd", "notches": []},
}

# config file, modify this
config = {
    "rotors": {
        # rotors 1-3 + zusatzwalze configuration: specify the rotor type, starting position ang ringstellung (ring starting position)
        "rotor1": {"rotor": rotors["VI"], "starting_pos": 0, "ringstellung": 0},
        "rotor2": {"rotor": rotors["I"], "starting_pos": 2, "ringstellung": 0},
        "rotor3": {"rotor": rotors["V"], "starting_pos": 0, "ringstellung": 0},
        "zusatzwalze": {
            "rotor": zusatzwalze["gamma"],
            "starting_pos": 0,
            "ringstellung": 0,
            "rotating": False,
        },
    },
    # choose the type of reflector
    "reflector": reflectors["UKW-C-thin"],
    # choose the plugboard configuration (lsit of max 10 elements)
    "plugboard": ["fa", "hb", "er", "gv", "ip", "nz", "qt", "uj", "xm", "wl"],
}

conf2 = {
    "rotors": {
        "rotor1": {"rotor": rotors["I"], "starting_pos": 0, "ringstellung": 1},
        "rotor2": {"rotor": rotors["II"], "starting_pos": 0, "ringstellung": 0},
        "rotor3": {"rotor": rotors["III"], "starting_pos": 0, "ringstellung": 0},
        "zusatzwalze": {
            "rotor": zusatzwalze["beta"],
            "starting_pos": 0,
            "ringstellung": 0,
        },
    },
    "reflector": reflectors["UKW-B-thin"],
}


# define an instance of the enigma machine with the config dict and your text
e = Enigma(conf2, "a")
print(e.run(logging=False))
