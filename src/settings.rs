use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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
    pub rotors: HashMap<String, RotorActionableSettings>,
}

pub struct ConfStruct {
    pub reflector: String,
    pub zus: RotorSettings,
    pub rot3: RotorSettings,
    pub rot2: RotorSettings,
    pub rot1: RotorSettings,
    pub plugboard: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RotorExport {
    pub rotor: RotorData,
    pub starting_position: i16,
    pub ringstellung: i16,
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
        let mut rotors_map = HashMap::new();
        rotors_map.insert(
            "zusatzwalze".to_owned(),
            RotorActionableSettings {
                rot: conf.zus.rot.clone(),
                pos: self.letter_to_position(&conf.zus.pos),
                ring: self.number_to_position(conf.zus.ring),
            },
        );
        rotors_map.insert(
            "rotor3".to_owned(),
            RotorActionableSettings {
                rot: conf.rot3.rot.clone(),
                pos: self.letter_to_position(&conf.rot3.pos),
                ring: self.number_to_position(conf.rot3.ring),
            },
        );
        rotors_map.insert(
            "rotor2".to_owned(),
            RotorActionableSettings {
                rot: conf.rot2.rot.clone(),
                pos: self.letter_to_position(&conf.rot2.pos),
                ring: self.number_to_position(conf.rot2.ring),
            },
        );
        rotors_map.insert(
            "rotor1".to_owned(),
            RotorActionableSettings {
                rot: conf.rot1.rot.clone(),
                pos: self.letter_to_position(&conf.rot1.pos),
                ring: self.number_to_position(conf.rot1.ring),
            },
        );

        let config = Configuration {
            plugboard: conf.plugboard.clone(),
            reflector: conf.reflector.clone(),
            rotors: rotors_map,
        };
        self.config = Some(config.clone());
    }

    pub fn get_rotor(&self, rotor_name: &String) -> Option<RotorExport> {
        if rotor_name == &"rotor1".to_owned()
            || rotor_name == &"rotor2".to_owned()
            || rotor_name == &"rotor3".to_owned()
        {
            //if a configuration exists
            if let Some(conf) = &self.config {
                //if the name of the rotors (1-3) is in the rotors hashmap
                if let Some(rot) = conf.rotors.get(rotor_name) {
                    // if the corrisponding rotor (I-VIII) is in the config
                    if let Some(data) = self.rotors.get(&rot.rot) {
                        //return
                        return Some(RotorExport {
                            rotor: data.clone(),
                            ringstellung: rot.ring,
                            starting_position: rot.pos,
                        });
                    }
                }
            }
        } else if rotor_name == "zusatzwalze" {
            //if a configuration exists
            if let Some(conf) = &self.config {
                //if the name of the rotors (1-3) is in the rotors hashmap
                if let Some(rot) = conf.rotors.get(rotor_name) {
                    // if the corrisponding rotor (I-VIII) is in the config
                    if let Some(data) = self.zusatzwalze.get(&rot.rot) {
                        //return
                        return Some(RotorExport {
                            rotor: data.clone(),
                            ringstellung: rot.ring,
                            starting_position: rot.pos,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn get_reflector(&self) -> Option<String> {
        if let Some(conf) = &self.config {
            if let Some(refl) = &self.reflectors.get(&conf.reflector) {
                return Some(refl.to_string());
            }
        };
        return None;
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

    #[test]
    fn should_return_rotor_export() {
        let mut s = Settings::new();
        let c = ConfStruct {
            reflector: "UKW-B-thin".to_owned(),
            zus: RotorSettings {
                rot: "gamma".to_owned(),
                pos: "p".to_owned(),
                ring: 11,
            },
            rot3: RotorSettings {
                rot: "VI".to_owned(),
                pos: "q".to_owned(),
                ring: 21,
            },
            rot2: RotorSettings {
                rot: "II".to_owned(),
                pos: "l".to_owned(),
                ring: 6,
            },
            rot1: RotorSettings {
                rot: "IV".to_owned(),
                pos: "e".to_owned(),
                ring: 13,
            },
            plugboard: [
                "bq".to_owned(),
                "cr".to_owned(),
                "di".to_owned(),
                "ej".to_owned(),
                "kw".to_owned(),
                "mt".to_owned(),
                "os".to_owned(),
                "px".to_owned(),
                "uz".to_owned(),
                "gh".to_owned(),
            ]
            .to_vec(),
        };
        s.configure(&c);
        let r = s.get_rotor(&"rotor1".to_owned()).clone();
        print!("{:?}", r);
        assert_eq!(
            r,
            Some(RotorExport {
                rotor: RotorData {
                    sequence: "esovpzjayquirhxlnftgkdcmwb".to_owned(),
                    notches: vec!["k".to_owned()]
                },
                starting_position: 4,
                ringstellung: 12
            })
        );
        let r = s.get_rotor(&"zusatzwalze".to_owned()).clone();
        assert_eq!(
            r,
            Some(RotorExport {
                rotor: RotorData {
                    sequence: "fsokanuerhmbtiycwlqpzxvgjd".to_owned(),
                    notches: vec![]
                },
                starting_position: 15,
                ringstellung: 10
            })
        )
    }

    #[test]
    pub fn should_return_reflector() {
        let mut s = Settings::new();
        let c = ConfStruct {
            reflector: "UKW-B-thin".to_owned(),
            zus: RotorSettings {
                rot: "gamma".to_owned(),
                pos: "p".to_owned(),
                ring: 11,
            },
            rot3: RotorSettings {
                rot: "VI".to_owned(),
                pos: "q".to_owned(),
                ring: 21,
            },
            rot2: RotorSettings {
                rot: "II".to_owned(),
                pos: "l".to_owned(),
                ring: 6,
            },
            rot1: RotorSettings {
                rot: "IV".to_owned(),
                pos: "e".to_owned(),
                ring: 13,
            },
            plugboard: [
                "bq".to_owned(),
                "cr".to_owned(),
                "di".to_owned(),
                "ej".to_owned(),
                "kw".to_owned(),
                "mt".to_owned(),
                "os".to_owned(),
                "px".to_owned(),
                "uz".to_owned(),
                "gh".to_owned(),
            ]
            .to_vec(),
        };
        s.configure(&c);
        let r = s.get_reflector();
        assert_eq!(r.unwrap(), "enkqauywjicopblmdxzvfthrgs".to_string())
    }
}
