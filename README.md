# virtual-enigma
Python3 replica of the enigma M4 "Shark" machine.

> No bugs are currently known and it should work fine with other simulations.

# usage
After downloading the code:

1) Go in enigma/main.py and create a configuration object that with the following structure, editing the content in <angular brackets>:

        configuration = {
                "rotors": {
                    "rot1": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 0-25 (int), or a-z (sting)>, "ringstellung": <range 0-25 (int), or a-z (sting)>},
                    "rot2": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 0-25 (int), or a-z (sting)>, "ringstellung": <range 0-25 (int), or a-z (sting)>},
                    "rot3": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 0-25 (int), or a-z (sting)>, "ringstellung": <range 0-25 (int), or a-z (sting)>},
                    "zus": {"rotor": zusatzwalze[<"gamma" or "beta">], "starting_pos": <range 0-25 (int), or a-z (sting)>, "ringstellung": <range 0-25 (int), or a-z (sting)>}
                },
                "reflector": reflectors[<"UKW-B-thin" or "UKW-C-thin">],
                "plugboard": [<max 10 couples of letters, as string>]
        }

2) Create an instance of enigma and pass the configuration object as the first parameter and the text you want to encode as the second parameter, e.g.:
`
        myEnigma = Enigma(configuration)
`
3) Run the simulation and print the result, e.g.:
`
        print(myEnigma.run("your text here"))
`
4) Run the python file and see the result of the encryption in your terminal.
