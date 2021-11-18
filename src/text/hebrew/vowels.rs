pub const SCHEWA: &str = "\u{05B0}";
pub const HATEF_SEGOL: &str= "\u{05B1}";
pub const HATEF_PATAH: &str = "\u{05B2}";
pub const HATEF_QAMETS: &str = "\u{05B3}";
pub const HIREQ: &str = "\u{05B4}";
pub const TSERE: &str = "\u{05B5}";
pub const SEGOL: &str = "\u{05B6}";
pub const PATAH: &str = "\u{05B7}";
pub const QAMETS: &str = "\u{05B8}";
pub const HOLEM: &str = "\u{05B9}";
pub const HOLEM_WAW: &str = "\u{05BA}\u{05D5}";
pub const SHUREQ: &str = "\u{05BC}\u{05D5}";
pub const QUIBBUTS: &str = "\u{05BB}";
pub const DAGESH: &str = "\u{05BC}";
pub const VOWELS: [&str; 14] = [
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

pub fn is_vowel(s: &str) -> bool {
    VOWELS.contains(&s)
}
