# virtual-enigma

![GitHub top language](https://img.shields.io/github/languages/top/thatsddr/virtual-enigma)
![GitHub repo size](https://img.shields.io/github/repo-size/thatsddr/virtual-enigma)
![GitHub](https://img.shields.io/github/license/thatsddr/virtual-enigma)

Python3 replica of the enigma M4 "Shark" machine.

## Project Status

This project is completed. The outputs of this enigma simulator match those of other online simulators.

## Usage
Prerequisites: ```python 3```.

After downloading the code:

1) Go in ```enigma/main.py``` and create a configuration object with the following structure, editing the values in < angular brackets >:

        configuration = {
                "rotors": {
                    "rot1": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 1-26 (int), or a-z (sting)>, "ringstellung": <range 1-26 (int), or a-z (sting)>},
                    "rot2": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 1-26 (int), or a-z (sting)>, "ringstellung": <range 1-26 (int), or a-z (sting)>},
                    "rot3": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 1-26 (int), or a-z (sting)>, "ringstellung": <range 1-26 (int), or a-z (sting)>},
                    "zus": {"rotor": zusatzwalze[<"gamma" or "beta">], "starting_pos": <range 1-26 (int), or a-z (sting)>, "ringstellung": <range 1-26 (int), or a-z (sting)>}
                },
                "reflector": reflectors[<"UKW-B-thin" or "UKW-C-thin">],
                "plugboard": [<max 10 couples of letters, as string>]
        }

2) Create an instance of enigma and pass the configuration object as the first parameter and the text you want to encode as the second parameter, e.g.:
`
        myEnigma = Enigma(configuration)
`;
3) Run the simulation and print the result, e.g.:
`
        print(myEnigma.run("your text here"))
`;
4) Run the python file (by running ```python3 main.py```) and you will see the output of the encryption in your terminal.
