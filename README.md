# virtual-enigma

![GitHub top language](https://img.shields.io/github/languages/top/thatsddr/virtual-enigma)
![GitHub repo size](https://img.shields.io/github/repo-size/thatsddr/virtual-enigma)
![GitHub](https://img.shields.io/github/license/thatsddr/virtual-enigma)

Rust replica of the enigma M4 "Shark" machine.

## Project Status

This project is completed. The outputs of this enigma simulator match those of other online simulators.

## Usage
Prerequisites: ```rust```. [Here](https://doc.rust-lang.org/book/ch01-01-installation.html) you can find the official installation guide.

After downloading the code:

1) Go to ```src/main.rs``` and create a configuration object with the following structure, editing the values in < angular brackets >:

        let configuration = ConfStruct {
                reflector: <"UKW-B-thin" or "UKW-C-thin" (String)>,
                zus: RotorInput {
                        rot: <"beta" or "gamma" (String)>,
                        pos: <'a'-'z' (char)>,
                        ring: <1-26 (i16)>
                },
                rot3: RotorInput {
                        rot: <"beta" or "gamma" (String)>,
                        pos: <'a'-'z' (char)>,
                        ring: <1-26 (i16)>
                },
                rot2: RotorInput {
                        rot: <"beta" or "gamma" (String)>,
                        pos: <'a'-'z' (char)>,
                        ring: <1-26 (i16)>
                },
                rot1: RotorInput {
                        rot: <"beta" or "gamma" (String)>,
                        pos: <'a'-'z' (char)>,
                        ring: <1-26 (i16)>
                },
                plugboard: [<max 10 couples of letters, with no repeated letters (Vec<String>)>],
        }

2) Create an instance of enigma and pass the configuration object, e.g.:

        let mut myEnigma = Enigma::new(configuration);

3) Use the ```.run()``` method to run the simulation. The method accepts 2 arguents: the text to encode, and a boolenan to run the encoding in normal mode (false) or debug mode(true) e.g.:

        println!(myEnigma.run(String::from("your text here"), false));

4) Run the rust file (by running ```cargo run``` in the ```src``` folder) and you will see the output of the encryption in your terminal.
