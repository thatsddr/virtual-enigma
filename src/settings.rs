use std::collections::HashMap;

struct RotorData {
    sequence: String,
    notches: Vec<String>,
}

struct RotorSettings {
    rot: String,
    pos: String,
    ring: i16,
}

struct RotorsConfiguration {
    zusatzwalze: RotorSettings,
    rotor3: RotorSettings,
    rotor2: RotorSettings,
    rotor1: RotorSettings,
}

struct Configuration {
    reflector: String,
    plugboard: Vec<String>,
    rotors: RotorsConfiguration,
}

struct Settings {
    rotors: HashMap<String, RotorData>,
    reflectors: HashMap<String, String>,
    zusatzwalze: HashMap<String, RotorData>,
    alphabet: String,
    config: Option<Configuration>,
}

impl Settings {
    fn new() -> Self {
        let rotor1 = RotorData {
            sequence: "ekmflgdqvzntowyhxuspaibrcj".to_owned(),
            notches: vec!["r".to_owned()],
        };
        let rotor2 = RotorData {
            sequence: "ajdksiruxblhwtmcqgznpyfvoe".to_owned(),
            notches: vec!["f".to_owned()],
        };
        let rotor3 = RotorData {
            sequence: "bdfhjlcprtxvznyeiwgakmusqo".to_owned(),
            notches: vec!["w".to_owned()],
        };
        let rotor4 = RotorData {
            sequence: "esovpzjayquirhxlnftgkdcmwb".to_owned(),
            notches: vec!["k".to_owned()],
        };
        let rotor5 = RotorData {
            sequence: "vzbrgityupsdnhlxawmjqofeck".to_owned(),
            notches: vec!["a".to_owned()],
        };
        let rotor6 = RotorData {
            sequence: "jpgvoumfyqbenhzrdkasxlictw".to_owned(),
            notches: vec!["a".to_owned(), "n".to_owned()],
        };
        let rotor7 = RotorData {
            sequence: "nzjhgrcxmyswboufaivlpekqdt".to_owned(),
            notches: vec!["a".to_owned(), "n".to_owned()],
        };
        let rotor8 = RotorData {
            sequence: "fkqhtlxocbjspdzramewniuygv".to_owned(),
            notches: vec!["a".to_owned(), "n".to_owned()],
        };

        let mut rotors = HashMap::new();
        rotors.insert("I".to_owned(), rotor1);
        rotors.insert("II".to_owned(), rotor2);
        rotors.insert("III".to_owned(), rotor3);
        rotors.insert("IV".to_owned(), rotor4);
        rotors.insert("V".to_owned(), rotor5);
        rotors.insert("VI".to_owned(), rotor6);
        rotors.insert("VII".to_owned(), rotor7);
        rotors.insert("VIII".to_owned(), rotor8);

        let mut reflectors = HashMap::new();
        reflectors.insert(
            "UKW-B-thin".to_owned(),
            "enkqauywjicopblmdxzvfthrgs".to_owned(),
        );
        reflectors.insert(
            "UKW-C-thin".to_owned(),
            "rdobjntkvehmlfcwzaxgyipsuq".to_owned(),
        );

        let zus_beta = RotorData {
            sequence: "leyjvcnixwpbqmdrtakzgfuhos".to_owned(),
            notches: vec![],
        };
        let zus_gamma = RotorData {
            sequence: "fsokanuerhmbtiycwlqpzxvgjd".to_owned(),
            notches: vec![],
        };
        let mut zusatzwalze = HashMap::new();
        zusatzwalze.insert("beta".to_owned(), zus_beta);
        zusatzwalze.insert("gamma".to_owned(), zus_gamma);

        let alphabet = "abcdefghijklmnopqrstuvwxyz".to_owned();
        let config = Option::None;

        Settings {
            rotors,
            reflectors,
            zusatzwalze,
            alphabet,
            config,
        }
    }
}
