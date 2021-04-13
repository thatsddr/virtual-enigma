from Enigma import Enigma

if __name__ == "__main__":
    # define an instance of the enigma machine with the config dictionary
    e = Enigma(
        {
            "reflector": "UKW-B-thin",
            "zus": {"rot": "gamma", "pos": "p", "ring": "k"},
            "rot3": {"rot": "VI", "pos": "q", "ring": "u"},
            "rot2": {"rot": "II", "pos": "l", "ring": "f"},
            "rot1": {"rot": "IV", "pos": "e", "ring": "m"},
            "plugboard": ["bq", "cr", "di", "ej", "kw", "mt", "os", "px", "uz", "gh"],
        }
    )
    
    #print some encrypted text
    print(e.run("have fun using this"))
