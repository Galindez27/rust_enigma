use rust_enigma::rotor::*;

#[test]
fn rotor_i_test() {
    let r: Rotor = match Rotor::new("I") {
        Some(r) => r,
        None => panic!("Failed to construct")
    };
    let a = ROTOR_CYPHERS_VALS[0].clone();

    let mut i = a.bytes().zip(ALPHABET.bytes());
    while let Some((ans, alpha)) = i.next() {
        let o = r.route_through(alpha);
        println!("{} {}", alpha as char, o as char);
        assert_eq!(ans, o);
    };
}

#[test]
fn rotor_i_stepping_test() {
    let mut r: Rotor = match Rotor::new("I") {
        Some(r) => r,
        None => panic!("Failed to construct")
    };
    
    let a: Vec<u8> = ROTOR_CYPHERS_VALS[0].bytes().collect();
    r.turnover();
    for c in b'A'..b'Z'+1 {
        let ans = a[((c - b'A' + 1) % 26) as usize];
        let out = r.route_through(c);
        assert_eq!(ans, out);
    }
}