mod settings;

use crate::settings::{ConfStruct, RotorSettings, Settings};

fn main() {
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
    let s = Settings::new();
    if let Some(i) = s.rotors.get("I") {
        print!("{:?}", i.sequence)
    }
}
