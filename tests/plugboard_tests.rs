use rust_enigma::plugboard::*;
use rand;
use std::collections::hash_set;

#[test]
fn route_plugboard_blank_test() {
    // assert_eq!(1, 1)
    let p: Plugboard = Plugboard::blank();
    for c in b'A'..b'Z' {
        assert_eq!(c, p.route_through(c));
    }
}

#[test]
fn route_plugboard_one_plug_test() {
    let p: Vec<Plug> = vec!((b'A', b'B'));
    let pb: Plugboard = Plugboard::new(&p);
    for plug in p {
        assert_eq!(pb.route_through(plug.0), plug.1);
        assert_eq!(pb.route_through(plug.1), plug.0);
    }
}

// This is a terrible way to do this, but thats okay for now
fn gen_random_unique_plug(used: &mut hash_set::HashSet<u8>) -> Plug {
    let a = {
        let mut temp: u8 = rand::random::<u8>();
        while used.contains(&temp) || temp < b'A' || temp > b'Z' {
            temp = rand::random::<u8>();
        }
        temp
    };
    let b = {
        let mut temp: u8 = rand::random::<u8>();
        while used.contains(&temp) || temp < b'A' || temp > b'Z' {
            temp = rand::random::<u8>();
        }
        temp
    };

    used.insert(a);
    used.insert(b);
    (a,b)
}

#[test]
fn route_plugboard_four_random_test() {
    let mut used: hash_set::HashSet<u8> = hash_set::HashSet::new();
    let mut plugs: Vec<Plug> = Vec::with_capacity(4);

    for _n in 0..4 {
        let plug = gen_random_unique_plug(&mut used);
        plugs.push(plug);
    }
    
    let pb: Plugboard = Plugboard::new(&plugs);
    for plug in plugs {
        assert_eq!(pb.route_through(plug.0), plug.1);
        assert_eq!(pb.route_through(plug.1), plug.0);
    }
    for c in b'A'..b'Z' {
        if !used.contains(&c) {
            assert_eq!(c, pb.route_through(c));
        }
    }
}

#[test]
#[should_panic]
fn plugging_plugboard_collision_test() {
    let p: Vec<Plug> = vec!((b'A', b'B'), (b'A', b'B'));
    Plugboard::new(&p);
}

#[test]
#[should_panic]
fn plugging_plugboard_invalid_char() {
    let p = vec!((6u8, 7u8));
    Plugboard::new(&p);
}