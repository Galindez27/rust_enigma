use std::fmt;
// use colored::*;



#[derive(Clone)]
pub struct Plugboard {
    charachters: Vec<u8>
}

pub type Plug = (u8, u8);


#[allow(dead_code)]
impl Plugboard {
    pub fn blank() -> Plugboard {
        let mut temp = Vec::with_capacity((b'Z' - b'A'+1) as usize);
        for i in b'A'..b'Z' {
            temp.push(i)  
        };
        Plugboard {
            charachters: temp
        }
    }

    pub fn new(plugs: &Vec<Plug>) -> Plugboard {
        let mut temp = Plugboard::blank();
        for plug in plugs {
            assert!(b'A' <= plug.0 && b'Z' >= plug.0, "Plug contains invalid charachter!");
            assert!(b'A' <= plug.1 && b'Z' >= plug.1, "Plug contains invalid charachter!");

            // Panic if a plug is trying to overwrite anothr plug
            assert_eq!(temp.charachters[(plug.0 - b'A') as usize], plug.0, "{}", format!("A plug has already been specified for {}",plug.0 as char));
            assert_eq!(temp.charachters[(plug.1 - b'A') as usize], plug.1, "{}", format!("A plug has already been specified for {}",plug.1 as char));
            

            temp.charachters[(plug.0 - b'A') as usize] = plug.1;
            temp.charachters[(plug.1 - b'A') as usize] = plug.0;
        };
        temp
    }

    pub fn route_through(&self, charachter: u8) -> u8 {
        self.charachters[(charachter - b'A') as usize]
    }

    pub fn clone(self) -> Plugboard {
        Plugboard{
            charachters: self.charachters.clone()
        }
    }
}

impl fmt::Debug for Plugboard{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut plugged: Vec<String> = Vec::new();
        let t = {
            let mut x: String = String::new();
            for i in 0..self.charachters.len() {
                let a = (i as u8 + b'A') as char;
                let b = self.charachters[i] as char;
                let q = format!("('{}' -> '{}') ", a,b);

                if a != b { plugged.push(q.clone()); }
                x += q.as_str();
            }
            x
        };
        f.debug_struct("Plugboard").field("Board values", &t).field("Plugs", &plugged).finish()
    }
}