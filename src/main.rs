mod plugboard;
mod reflector;
mod settings;

use crate::settings::{ConfStruct, RotorInput, Settings};

fn main() {
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

    let mut s = Settings::new();
    s.configure(&c);
    print!("{:#?}", s.config)
}
