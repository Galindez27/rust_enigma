pub mod plugboard;
pub mod rotor;

//Place holder for now
pub fn main() {
    let p: plugboard::Plugboard = plugboard::Plugboard::new(&vec!((b'a', b'b')));
    println!("{:?}", p);
}
