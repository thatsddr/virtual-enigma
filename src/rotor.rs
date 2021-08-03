use crate::settings::{RotorData, RotorExport};

pub struct Rotor {
    alphabet: String,
    sequence: String,
    ringstellung: i16,
    starting_position: i16,
    steps: i16,
}

impl Rotor {
    pub fn new(rotor: RotorExport) -> Self {
        Rotor {
            alphabet: "abcdefghijklmnopqrstuvwxyz".to_string(),
            sequence: rotor.rotor.sequence,
            ringstellung: rotor.ringstellung,
            starting_position: rotor.starting_position,
            steps: 0,
        }
    }

    pub fn init(self) {}

    fn apply_ringstellung(&mut self) {
        //string to array of chars
        let chars_sequence: Vec<char> = self.sequence.chars().collect();
        //first substring
        let mut new1: String = chars_sequence
            [(chars_sequence.len() - self.ringstellung as usize)..]
            .into_iter()
            .collect();
        //second ssubstring
        let new2: String = chars_sequence[..(chars_sequence.len() - self.ringstellung as usize)]
            .into_iter()
            .collect();
        // join substrings
        new1.push_str(&new2);
        // replace
        self.sequence = new1
    }

    fn update_steps(&mut self, num: i16) {
        if (self.steps + num) > 25 {
            self.steps = (self.steps + num) % 26
        } else {
            self.steps += num
        }
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
                notches: vec!["r".to_owned()],
            },
            ringstellung: 3,
            starting_position: 0,
        };
        let mut rotor = Rotor::new(r);
        rotor.apply_ringstellung();
        assert_eq!(rotor.sequence, "rcjekmflgdqvzntowyhxuspaib".to_owned())
    }

    pub fn should_update_steps() {
        let r = RotorExport {
            rotor: RotorData {
                sequence: "ekmflgdqvzntowyhxuspaibrcj".to_owned(),
                notches: vec!["r".to_owned()],
            },
            ringstellung: 3,
            starting_position: 0,
        };
        let mut rotor = Rotor::new(r);
        rotor.update_steps(25);
        assert_eq!(rotor.steps, 25);
        rotor.update_steps(2);
        assert_eq!(rotor.steps, 2);
        rotor.update_steps(1);
        assert_eq!(rotor.steps, 3);
    }
}
