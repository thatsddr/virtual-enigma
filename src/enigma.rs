use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;
use crate::settings::{ConfStruct, RotorInput, Settings};

pub struct Enigma {
    settings: Settings,
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    zusatzwalze: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(c: ConfStruct) -> Self {
        let mut settings = Settings::new();
        settings.configure(&c);

        Enigma {
            settings: settings.clone(),
            rotor1: Rotor::new(settings.get_rotor("rotor1").unwrap()),
            rotor2: Rotor::new(settings.get_rotor("rotor2").unwrap()),
            rotor3: Rotor::new(settings.get_rotor("rotor3").unwrap()),
            zusatzwalze: Rotor::new(settings.get_rotor("zusatzwalze").unwrap()),
            reflector: Reflector::new(settings.get_reflector().unwrap()),
            plugboard: Plugboard::new(&settings.get_plugboard().unwrap()),
        }
    }

    pub fn rotate_rotors(&mut self) {
        self.rotor1.rotate(1);
        if self
            .rotor1
            .notches
            .contains(&self.rotor1.sequence.chars().next().unwrap())
        {
            self.rotor2.rotate(1);
        }
        if self
            .rotor2
            .notches
            .contains(&self.rotor2.sequence.chars().next().unwrap())
        {
            self.rotor3.rotate(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_enigma() {
        let c = ConfStruct {
            reflector: "UKW-B-thin".to_owned(),
            zus: RotorInput {
                rot: "gamma".to_owned(),
                pos: 'p',
                ring: 11,
            },
            rot3: RotorInput {
                rot: "VI".to_owned(),
                pos: 'q',
                ring: 21,
            },
            rot2: RotorInput {
                rot: "II".to_owned(),
                pos: 'l',
                ring: 6,
            },
            rot1: RotorInput {
                rot: "IV".to_owned(),
                pos: 'e',
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
        Enigma::new(c);
    }

    #[test]
    fn should_rotate_the_rotors() {
        let c = ConfStruct {
            reflector: "UKW-B-thin".to_owned(),
            zus: RotorInput {
                rot: "gamma".to_owned(),
                pos: 'p',
                ring: 11,
            },
            rot3: RotorInput {
                rot: "VI".to_owned(),
                pos: 'q',
                ring: 21,
            },
            rot2: RotorInput {
                rot: "II".to_owned(),
                pos: 'l',
                ring: 6,
            },
            rot1: RotorInput {
                rot: "IV".to_owned(),
                pos: 'e',
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
        let mut e = Enigma::new(c);
        for n in 1..100 {
            e.rotate_rotors()
        }
        assert_eq!(e.rotor2.sequence, "siruxblhwtmcqgznpyfvoeajdk".to_string())
    }
}
