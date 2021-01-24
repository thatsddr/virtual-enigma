# virtual-enigma
Python3 replica of the enigma M4 "Shark" machine.

> No bugs are currently known and it should work fine with other simulations.

# usage
After downloading the code:

1) Go in enigma/main.py and create a configuration object that with the following structure, editing the content in <angular brackets>:
`
configuration = {
        "rotors": {
            "rotor1": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 0-25, integer>, "ringstellung": <range 0-25, integer>},
            "rotor2": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 0-25, integer>, "ringstellung": <range 0-25, integer>},
            "rotor3": {"rotor": rotors[<range I - VIII, string>], "starting_pos": <range 0-25, integer>, "ringstellung": <range 0-25, integer>},
            "zusatzwalze": {"rotor": zusatzwalze[<"gamma" or "beta">], "starting_pos": <range 0-25, integer>, "ringstellung": <range 0-25, integer>}
        },
        "reflector": reflectors[<"UKW-B-thin" or "UKW-C-thin">],
        "plugboard": [<max 10 couples of letters, as string>]
}
`
Alternatively, use the `confMaker` function if you want to pass letters instead of numbers in the configuration, like in the example below:
`
  configuration = confMaker(
        <"UKW-B-thin" or "UKW-C-thin">,
        {"rot": <"beta" or "gamma">, "pos": <range a-z, as string>, "ring": <range a-z, as string>},
        {"rot": <range I - VIII, string>, "pos": <range a-z, as string>, "ring": <range a-z, as string>},
        {"rot": <range I - VIII, string>, "pos": <range a-z, as string>, "ring": <range a-z, as string>},
        {"rot": <range I - VIII, string>, "pos": <range a-z, as string>, "ring": <range a-z, as string>},
        plugboard=[<max 10 couples of letters, as string>]
  ) 
`
2) Create an instance of enigma and pass the configuration object as the first parameter and the text you want to encode as the second parameter, e.g.:
`
myEnigma = Enigma(configuration, "text here")
`
3) Run the simulation and print the result, e.g.:
`
print(myEnigma.run())
`
