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

# allows to use letters instead of numbers as rotors and rings positions
def confMaker(reflector, zus, rot3, rot2, rot1, plugboard=[]):
    alphabet = "abcdefghijklmnopqrstuvwxyz"
    return {
        "reflector": reflectors[reflector],
        "plugboard": plugboard,
        "rotors": {
            "zusatzwalze": {
                "rotor": zusatzwalze[zus.get("rot")],
                "starting_pos": alphabet.index(zus.get("pos").lower()),
                "ringstellung": alphabet.index(zus.get("ring").lower()),
            },
            "rotor3": {
                "rotor": rotors[rot3.get("rot")],
                "starting_pos": alphabet.index(rot3.get("pos").lower()),
                "ringstellung": alphabet.index(rot3.get("ring").lower()),
            },
            "rotor2": {
                "rotor": rotors[rot2.get("rot")],
                "starting_pos": alphabet.index(rot2.get("pos").lower()),
                "ringstellung": alphabet.index(rot2.get("ring").lower()),
            },
            "rotor1": {
                "rotor": rotors[rot1.get("rot")],
                "starting_pos": alphabet.index(rot1.get("pos").lower()),
                "ringstellung": alphabet.index(rot1.get("ring").lower()),
            },
        },
    }

cx = confMaker(
    "UKW-B-thin",
    {"rot": "gamma", "pos": "p", "ring": "k"},
    {"rot": "VI", "pos": "q", "ring": "u"},
    {"rot": "II", "pos": "l", "ring": "f"},
    {"rot": "IV", "pos": "e", "ring": "m"},
    plugboard=["bq", "cr", "di", "ej", "kw", "mt", "os", "px", "uz", "gh"],
)

# define an instance of the enigma machine with the config dict and your text
e = Enigma(cx, "have fun using this")
print(e.run(logging=False))
