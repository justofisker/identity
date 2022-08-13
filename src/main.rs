#[allow(dead_code)]
#[derive(Debug)]
struct Identity<'a> {
    gender: &'a str,
    pronouns: Pronouns,
}

bitflags::bitflags! {
    struct Pronouns: u8 {
        const IT_ITS = 0b00000001;
        const THEY_THEM = 0b00000010;
        const SHE_HER = 0b00000100;
    }
}

fn main() {
    let just = Identity {
        gender: "Agender",
        pronouns: Pronouns::IT_ITS,
    };
    let trixie = Identity {
        gender: "Demigirl",
        pronouns: Pronouns::SHE_HER | Pronouns::IT_ITS | Pronouns::THEY_THEM,
    };
    let bea = Identity {
        gender: "Girl",
        pronouns: Pronouns::SHE_HER,
    };
    dbg!(just);
    dbg!(trixie);
    dbg!(bea);
}