use std::collections::HashMap;

#[derive(Clone)]
pub struct RotorData {
    pub sequence: String,
    pub notches: Vec<String>,
}

pub struct RotorSettings {
    pub rot: String,
    pub pos: String,
    pub ring: i16,
}

#[derive(Clone, Debug)]
pub struct RotorActionableSettings {
    pub rot: String,
    pub pos: i16,
    pub ring: i16,
}

#[derive(Clone, Debug)]
pub struct RotorsConfiguration {
    pub zusatzwalze: RotorActionableSettings,
    pub rotor3: RotorActionableSettings,
    pub rotor2: RotorActionableSettings,
    pub rotor1: RotorActionableSettings,
}

#[derive(Clone, Debug)]
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

#[derive(Clone)]
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

    pub fn letter_to_position(&self, letter: &String) -> i16 {
        if let Some(i) = self.alphabet.find(letter) {
            return i as i16;
        };
        panic!("Character {:?} not in alphabet", letter)
    }

    pub fn number_to_position(&self, num: i16) -> i16 {
        if num > 0 && num < 26 {
            return num - 1;
        }
        panic!("Number {:?} either too big or too small", num)
    }

    pub fn configure(&mut self, conf: &ConfStruct) -> () {
        let config = Configuration {
            plugboard: conf.plugboard.clone(),
            reflector: conf.reflector.clone(),
            rotors: RotorsConfiguration {
                zusatzwalze: RotorActionableSettings {
                    rot: conf.zus.rot.clone(),
                    pos: self.letter_to_position(&conf.zus.pos),
                    ring: self.number_to_position(conf.zus.ring),
                },
                rotor3: RotorActionableSettings {
                    rot: conf.rot3.rot.clone(),
                    pos: self.letter_to_position(&conf.rot3.pos),
                    ring: self.number_to_position(conf.rot3.ring),
                },
                rotor2: RotorActionableSettings {
                    rot: conf.rot2.rot.clone(),
                    pos: self.letter_to_position(&conf.rot2.pos),
                    ring: self.number_to_position(conf.rot2.ring),
                },
                rotor1: RotorActionableSettings {
                    rot: conf.rot1.rot.clone(),
                    pos: self.letter_to_position(&conf.rot1.pos),
                    ring: self.number_to_position(conf.rot1.ring),
                },
            },
        };
        self.config = Some(config.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_to_position_works() {
        let s = Settings::new();
        let p = s.letter_to_position(&"d".to_owned());
        assert_eq!(p, 3);
    }

    #[test]
    fn number_to_position_works() {
        let s = Settings::new();
        let p = s.number_to_position(14);
        assert_eq!(p, 13)
    }

    #[test]
    #[should_panic]
    fn number_to_position_panics() {
        let s = Settings::new();
        let p = s.number_to_position(44);
        assert_eq!(p, 13)
    }
}
