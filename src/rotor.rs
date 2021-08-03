use crate::settings::{RotorData, RotorExport};

pub struct Rotor {
    alphabet: String,
    sequence: String,
    notches: Vec<char>,
    ringstellung: i16,
    starting_position: i16,
    steps: i16,
}

impl Rotor {
    pub fn new(rotor: RotorExport) -> Self {
        Rotor {
            alphabet: "abcdefghijklmnopqrstuvwxyz".to_string(),
            sequence: rotor.rotor.sequence,
            notches: rotor.rotor.notches,
            ringstellung: rotor.ringstellung,
            starting_position: rotor.starting_position,
            steps: 0,
        }
    }

    pub fn init(&mut self) {
        //apply ringstellung
        if self.ringstellung > 0 && self.ringstellung < 26 {
            self.apply_ringstellung();
        }

        //update nothces
        let mut notches = vec![];
        let chars_sequence: Vec<char> = self.sequence.chars().collect();
        for n in self.notches.clone() {
            if let Some(i) = self.alphabet.find(n) {
                notches.push(chars_sequence[i]);
            } else {
                panic!("Character {:?} not in alphabet", n)
            };
        }
        self.notches = notches;

        //apply initial rotation
        if self.starting_position > 0 && self.starting_position < 26 {
            self.rotate(self.starting_position)
        }
    }

    fn apply_ringstellung(&mut self) {
        //string to array of chars
        let chars_sequence: Vec<char> = self.sequence.chars().collect();
        //first substring
        let mut new: String = chars_sequence[(chars_sequence.len() - self.ringstellung as usize)..]
            .into_iter()
            .collect();
        //second ssubstring
        let new2: String = chars_sequence[..(chars_sequence.len() - self.ringstellung as usize)]
            .into_iter()
            .collect();
        // join substrings
        new.push_str(&new2);
        // replace
        self.sequence = new
    }

    fn update_steps(&mut self, num: i16) {
        if (self.steps + num) > 25 {
            self.steps = (self.steps + num) % 26
        } else {
            self.steps += num
        }
    }

    pub fn rotate(&mut self, num: i16) {
        // string to array of characters
        let chars_sequence: Vec<char> = self.sequence.chars().collect();
        // first substring
        let mut new: String = chars_sequence[(num as usize)..].into_iter().collect();
        // second substring
        let new2: String = chars_sequence[..(num as usize)].into_iter().collect();
        // join substrings
        new.push_str(&new2);
        // replace
        self.sequence = new;
        // update steps
        self.update_steps(num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_apply_ringstellung() {
        let r = RotorExport {
            rotor: RotorData {
                sequence: "ekmflgdqvzntowyhxuspaibrcj".to_owned(),
                notches: vec!['r'],
            },
            ringstellung: 3,
            starting_position: 0,
        };
        let mut rotor = Rotor::new(r);
        rotor.apply_ringstellung();
        assert_eq!(rotor.sequence, "rcjekmflgdqvzntowyhxuspaib".to_owned())
    }

    #[test]
    pub fn should_update_steps() {
        let r = RotorExport {
            rotor: RotorData {
                sequence: "ekmflgdqvzntowyhxuspaibrcj".to_owned(),
                notches: vec!['r'],
            },
            ringstellung: 0,
            starting_position: 0,
        };
        let mut rotor = Rotor::new(r);
        rotor.update_steps(25);
        assert_eq!(rotor.steps, 25);
        rotor.update_steps(2);
        assert_eq!(rotor.steps, 1);
        rotor.update_steps(1);
        assert_eq!(rotor.steps, 2);
    }

    #[test]
    pub fn should_rotate() {
        let r = RotorExport {
            rotor: RotorData {
                sequence: "ekmflgdqvzntowyhxuspaibrcj".to_owned(),
                notches: vec!['r'],
            },
            ringstellung: 0,
            starting_position: 0,
        };
        let mut rotor = Rotor::new(r);
        rotor.rotate(2);
        assert_eq!(rotor.sequence, "mflgdqvzntowyhxuspaibrcjek".to_owned());
    }

    #[test]
    pub fn should_init() {
        let r = RotorExport {
            rotor: RotorData {
                sequence: "ekmflgdqvzntowyhxuspaibrcj".to_owned(),
                notches: vec!['r'],
            },
            ringstellung: 1,
            starting_position: 0,
        };
        let mut rotor = Rotor::new(r);
        rotor.init();
        assert_eq!(rotor.notches, vec!['x'].to_vec());
    }
}