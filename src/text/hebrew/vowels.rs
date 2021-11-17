pub const SCHEWA: String = String::from("\u{05B0}");
pub const HATEF_SEGOL: String = String::from("\u{05B1}");
pub const HATEF_PATAH: String = String::from("\u{05B2}");
pub const HATEF_QAMETS: String = String::from("\u{05B3}");
pub const HIREQ: String = String::from("\u{05B4}");
pub const TSERE: String = String::from("\u{05B5}");
pub const SEGOL: String = String::from("\u{05B6}");
pub const PATAH: String = String::from("\u{05B7}");
pub const QAMETS: String = String::from("\u{05B8}");
pub const HOLEM: String = String::from("\u{05B9}");
pub const HOLEM_WAW: String = String::from("\u{05BA}\u{05D5}");
pub const SHUREQ: String = String::from("\u{05BC}\u{05D5}");
pub const QUIBBUTS: String = String::from("\u{05BB}");
pub const DAGESH: String = String::from("\u{05BC}");
pub const VOWELS: [String; 14] = [
    SCHEWA,
    HATEF_SEGOL,
    HATEF_PATAH,
    HATEF_QAMETS,
    HIREQ,
    TSERE,
    SEGOL,
    PATAH,
    QAMETS,
    HOLEM,
    HOLEM_WAW,
    SHUREQ,
    QUIBBUTS,
    DAGESH
];

pub fn is_vowel(s: String) -> bool {
    VOWELS.contains(&s)
}
