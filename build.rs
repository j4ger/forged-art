use std::{env, fs, path::Path};

//        | Free | Fist | Marked | Circle | Double |
// Purple:| 3    | 2    | 2      | 3      | 2      |
// Blue:  | 3    | 3    | 3      | 2      | 2      |
// Red:   | 3    | 3    | 3      | 3      | 2      |
// Green: | 3    | 3    | 3      | 3      | 3      |
// Yellow:| 4    | 3    | 3      | 3      | 3      |

const SOURCE: [(&'static str, [(&'static str, usize); 5]); 5] = [
    (
        "Purple",
        [
            ("Free", 3),
            ("Fist", 2),
            ("Marked", 2),
            ("Circle", 3),
            ("Double", 2),
        ],
    ),
    (
        "Blue",
        [
            ("Free", 3),
            ("Fist", 3),
            ("Marked", 3),
            ("Circle", 2),
            ("Double", 2),
        ],
    ),
    (
        "Red",
        [
            ("Free", 3),
            ("Fist", 3),
            ("Marked", 3),
            ("Circle", 3),
            ("Double", 2),
        ],
    ),
    (
        "Green",
        [
            ("Free", 3),
            ("Fist", 3),
            ("Marked", 3),
            ("Circle", 3),
            ("Double", 3),
        ],
    ),
    (
        "Yellow",
        [
            ("Free", 4),
            ("Fist", 3),
            ("Marked", 3),
            ("Circle", 3),
            ("Double", 3),
        ],
    ),
];

fn gen_card(id: usize, color: &str, ty: &str) -> String {
    format!("Card {{ id: {id}, color: CardColor::{color}, ty: AuctionType::{ty} }}")
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("card_list.rs");
    let mut content = String::new();
    content.push_str(
        "
            use crate::common::card::{Card, CardColor, AuctionType};
            pub const CARD_LIST: [Card; 70] = [
        ",
    );
    let mut id = 1;
    for (color, variants) in SOURCE {
        for (variant, count) in variants {
            for _ in 0..count {
                content.push_str(&gen_card(id, color, variant));
                content.push(',');
                id += 1;
            }
        }
    }
    content.push_str("];");
    fs::write(&dest_path, &content).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

