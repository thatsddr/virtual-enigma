from Enigma import Enigma

if __name__ == "__main__":
    # define an instance of the enigma machine with the config dictionary
    e = Enigma(
        {
            "reflector": "UKW-B-thin",
            "zus": {"rot": "gamma", "pos": "p", "ring": 11},
            "rot3": {"rot": "VI", "pos": "q", "ring": 21},
            "rot2": {"rot": "II", "pos": "l", "ring": 6},
            "rot1": {"rot": "IV", "pos": "e", "ring": 13},
            "plugboard": ["bq", "cr", "di", "ej", "kw", "mt", "os", "px", "uz", "gh"],
        }
    )
    
    #print some encrypted text
    print(e.run("have fun using this"))
