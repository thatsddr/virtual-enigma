use std::collections::HashMap;

pub struct RotorData {
    pub sequence: String,
    pub notches: Vec<String>,
}

pub struct RotorSettings {
    pub rot: String,
    pub pos: String,
    pub ring: i16,
}

pub struct RotorsConfiguration {
    pub zusatzwalze: RotorSettings,
    pub rotor3: RotorSettings,
    pub rotor2: RotorSettings,
    pub rotor1: RotorSettings,
}

pub struct Configuration {
    pub reflector: String,
    pub plugboard: Vec<String>,
    pub rotors: RotorsConfiguration,
}

pub struct ConfStruct {
    pub reflector: String,
    pub zus: RotorSettings,
    pub rot3: RotorSettings,
    pub rot2: RotorSettings,
    pub rot1: RotorSettings,
    pub plugboard: Vec<String>,
}

pub struct Settings {
    pub rotors: HashMap<String, RotorData>,
    pub reflectors: HashMap<String, String>,
    pub zusatzwalze: HashMap<String, RotorData>,
    pub alphabet: String,
    pub config: Option<Configuration>,
}

impl Settings {
    pub fn new() -> Self {
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
