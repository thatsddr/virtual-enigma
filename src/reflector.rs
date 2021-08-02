pub struct Reflector {
    alphabet: String,
    sequence: String,
}

impl Reflector {
    pub fn new(sequence: String) -> Self {
        Reflector {
            alphabet: "abcdefghijklmnopqrstuvwxyz".to_string(),
            sequence: sequence,
        }
    }

    pub fn reflect(self, letter: char, zus_step: i32) -> char {
        if let Some(position) = self.alphabet.find(letter) {
            let chars: Vec<char> = self.sequence.chars().collect();
            return chars[((26 - (zus_step - (position as i32))) % 26) as usize];
        } else {
            panic!("Character {:?} not in alphabet", letter)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_reflect() {
        let r = Reflector::new("enkqauywjicopblmdxzvfthrgs".to_string());
        let output = r.reflect('a', 2);
        assert_eq!(output, 'g')
    }

    #[test]
    #[should_panic]
    pub fn should_panic() {
        let r = Reflector::new("enkqauywjicopblmdxzvfthrgs".to_string());
        r.reflect('ù', 2);
    }
}
