use std::collections::HashMap;

pub struct Plugboard {
    dict: HashMap<char, char>,
}

impl Plugboard {
    pub fn new(couples: &Vec<String>) -> Self {
        if couples.len() < 11 {
            for c in couples {
                if c.len() != 2 {
                    panic!("A couple of letters contains the wrong number of characters")
                }
            }
            let plugboard = Plugboard {
                dict: Plugboard::dictify(couples.clone()),
            };
            return plugboard;
        }
        panic!("Too many couples in the plugboard")
    }

    pub fn dictify(list: Vec<String>) -> HashMap<char, char> {
        let mut dict: HashMap<char, char> = HashMap::new();
        for c in list.clone() {
            let chars: Vec<char> = c.chars().collect();
            if !dict.contains_key(&chars[0]) && !dict.contains_key(&chars[1]) {
                dict.insert(chars[0], chars[1]);
                dict.insert(chars[1], chars[0]);
            } else {
                panic!("Duplicated letter in the plugboard")
            };
        }
        return dict;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_plugboard() -> Vec<String> {
        return [
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
        .to_vec();
    }

    #[test]
    #[should_panic]
    pub fn should_panic_for_the_length_of_the_couple() {
        let couples = ["abc".to_owned(), "lk".to_owned()].to_vec();
        Plugboard::new(&couples);
    }

    #[test]
    #[should_panic]
    pub fn should_panic_for_the_length_of_the_list() {
        let couples = [
            "ab".to_owned(),
            "cd".to_owned(),
            "ef".to_owned(),
            "gh".to_owned(),
            "ij".to_owned(),
            "kl".to_owned(),
            "mn".to_owned(),
            "op".to_owned(),
            "qr".to_owned(),
            "st".to_owned(),
            "uv".to_owned(),
            "wx".to_owned(),
            "yz".to_owned(),
        ]
        .to_vec();
        Plugboard::new(&couples);
    }

    #[test]
    #[should_panic]
    pub fn should_panic_for_the_duplicated_letter() {
        let couples = ["ab".to_owned(), "cb".to_owned()].to_vec();
        Plugboard::new(&couples);
    }

    #[test]
    pub fn should_generate_a_plugboard() {
        let couples = generate_plugboard();
        Plugboard::new(&couples);
    }
}