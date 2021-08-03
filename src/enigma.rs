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

    fn encrypt_letter(&mut self, letter: char) -> char {
        let v1 = self.rotor1.left_mov(letter, 0);
        let v2 = self.rotor2.left_mov(v1, self.rotor1.steps);
        let v3 = self.rotor3.left_mov(v2, self.rotor2.steps);
        let v4 = self.zusatzwalze.left_mov(v3, self.rotor3.steps);
        let v5 = self.reflector.reflect(v4, self.zusatzwalze.steps);
        let v6 = self.zusatzwalze.right_mov(v5);
        let v7 = self.rotor3.right_mov(v6);
        let v8 = self.rotor2.right_mov(v7);
        let v9 = self.rotor1.right_mov(v8);
        v9
    }

    pub fn run(&mut self, text: String) -> String {
        let to_process = self.plugboard.apply(text);
        let chars_text: Vec<char> = to_process.chars().collect();
        let mut temp = vec![];
        let mut temp_length: i32 = 0;
        for i in 0..chars_text.len() {
            if self.settings.alphabet.contains(chars_text[i]) {
                temp_length += 1;
                self.rotate_rotors();
                temp.push(self.encrypt_letter(chars_text[i]));
                if temp_length % 4 == 0 {
                    temp_length = 0;
                    temp.push(' ');
                }
            }
        }
        let tempstr: String = temp.into_iter().collect();
        self.plugboard.apply(tempstr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_enigma() -> Enigma {
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
        return Enigma::new(c);
    }

    #[test]
    fn should_create_a_new_enigma() {
        generate_enigma();
    }

    #[test]
    fn should_rotate_the_rotors() {
        let mut e = generate_enigma();
        for n in 1..100 {
            e.rotate_rotors()
        }
        assert_eq!(e.rotor2.sequence, "siruxblhwtmcqgznpyfvoeajdk".to_string())
    }
    #[test]
    fn should_encrypt_the_letter() {
        let mut e = generate_enigma();
        let r = e.encrypt_letter('h');
        assert_eq!(r, 'z')
    }

    #[test]
    fn should_run() {
        let mut e = generate_enigma();
        let r = e.run(String::from("have fun using this"));
        panic!("{:?}", r)
    }
}
