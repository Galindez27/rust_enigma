pub static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Clone, Copy)]
pub enum RotorModel {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII
}

pub static ROTOR_CYPHERS_VALS: [&str; 8] = [
    "EKMFLGDQVZNTOWYHXUSPAIBRCJ", // I
    "AJDKSIRUXBLHWTMCQGZNPYFVOE", // II
    "BDFHJLCPRTXVZNYEIWGAKMUSQO", // III
    "ESOVPZJAYQUIRHXLNFTGKDCMWB", // IV
    "VZBRGITYUPSDNHLXAWMJQOFECK", // V
    "JPGVOUMFYQBENHZRDKASXLICTW", // VI
    "NZJHGRCXMYSWBOUFAIVLPEKQDT", // VII
    "FKQHTLXOCBJSPDZRAMEWNIUYGV", // VIII
];

pub static ROTOR_TURNOVERS: [&str;8] = ["Q", "E", "V", "J", "Z", "ZM", "ZM", "ZM"];

#[allow(dead_code)] // For now
pub static RFLCT_CYPHERS: [&str;8] = [
    "EJMZALYXVBWFCRQUONTSPIKHGD", // Reflector A
    "YRUHQSLDPXNGOKMIEBFZCWVJAT", // Reflector B
    "FVPJIAOYEDRZXWGCTKUQSBNMHL", // Reflector C
    "ENKQAUYWJICOPBLMDXZVFTHRGS", // Reflector B Thin
    "RDOBJNTKVEHMLFCWZAXGYIPSUQ", // Reflector C Thin
    "LEYJVCNIXWPBQMDRTAKZGFUHOS", // Beta
    "FSOKANUERHMBTIYCWLQPZXVGJD", // Gamma
    "ABCDEFGHIJKLMNOPQRSTUZWXYZ" // Enigma 1
];

pub struct Rotor {
    cypher_values: Vec<u8>,
    pos: u8,
    turnover_pos: Vec<u8>,
}

fn to_model(s: &str) -> Option<RotorModel> {
    match s {
        "I" => Some(RotorModel::I),
        "II" => Some(RotorModel::II),
        "III" => Some(RotorModel::III),
        "IV" => Some(RotorModel::IV),
        "V" => Some(RotorModel::V),
        "VI" => Some(RotorModel::VI),
        "VII" => Some(RotorModel::VII),
        "VIII" => Some(RotorModel::VIII),
        _ => None
    }
} 

impl Rotor {
    pub fn new(model: &str) -> Option<Rotor> {
        let model = match to_model(model) {
            Some(m) => m,
            None => return None
        };
        let cypher = ROTOR_CYPHERS_VALS[model as usize];
        let turnovers = ROTOR_TURNOVERS[model as usize];

        Some(Rotor{
            cypher_values: cypher.bytes().collect(),
            pos: 0,
            turnover_pos: turnovers.bytes().collect()
        })
    }

    pub fn route_through(&self, c: u8) -> u8 {
        let cypher_index = ((c - b'A' + self.pos) % 26u8) as usize;
        self.cypher_values[cypher_index]
    }

    pub fn turnover(&mut self) -> bool {
        let mut to_ret = false;
        for c in &self.turnover_pos {
            if *c == self.pos { to_ret = true; break; }
        }
        self.pos += 1;
        to_ret
    }
}