extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut items = HashMap::new();

    items.insert("Full Moon Sword", 2);
    items.insert("Poison Sword", 3);
    items.insert("Zodiac Sword", 4);
    items.insert("Snake Sword", 5);

    println!("Metin2 SWORD ENCHANT\tPICK A SWORD:\nFull Moon Sword '1'\tPoison Sword '2'\tZodiac Sword '3'\t Snake Sword '4'");
    let mut input = inputFunc_int();
    let mut x = 0;

    if 1 == input {
        let mut plus_item1 = Plus_item {
            plus: 0,
            chance: items["Full Moon Sword"],
            max_Plus: 9,
        };
        println!("Full Moon Sword");

        loop {
            if plus_item1.max_Plus == plus_item1.plus {
                println!("The item is MAX VALUE");
                break;
            }
            plus_item1.plus_job();
            println!("Full Moon Sword +{}", plus_item1.plus);
        }
    } else if 2 == input {
        let mut plus_item1 = Plus_item {
            plus: 0,
            chance: items["Poison Sword"],
            max_Plus: 10,
        };
        println!("Poison Sword");

        loop {
            if plus_item1.max_Plus == plus_item1.plus {
                println!("The item is MAX VALUE");
                break;
            }
            plus_item1.plus_job();
            println!("Poison Sword +{}", plus_item1.plus);
        }
    } else if 3 == input {
        let mut plus_item1 = Plus_item {
            plus: 0,
            chance: items["Zodiac Sword"],
            max_Plus: 11,
        };
        println!("Zodiac Sword");

        loop {
            if plus_item1.max_Plus == plus_item1.plus {
                println!("The item is MAX VALUE");
                break;
            }
            plus_item1.plus_job();
            println!("Zodiac Sword +{}", plus_item1.plus);
        }
    } else if 4 == input {
        let mut plus_item1 = Plus_item {
            plus: 0,
            chance: items["Snake Sword"],
            max_Plus: 12,
        };
        println!("Snake Sword");

        loop {
            if plus_item1.max_Plus == plus_item1.plus {
                println!("The item is MAX VALUE");
                break;
            }
            plus_item1.plus_job();
            println!("Snake Sword +{}", plus_item1.plus);
        }
    } else {
        println!("Wrong Entry!")
    }
}

fn inputFunc_int() -> i32 {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to readline");

    let a: i32 = a.trim().parse().expect("failed to readline");

    a
}

struct Plus_item {
    plus: u32,
    chance: u32,
    max_Plus: u32,
}

impl Plus_item {
    fn plus_job(&mut self) {
        println!(
            "BLESSING PAPER '1'\tBLACKSMITH BOOK '2'\tDRAGON PAPER '3'\tWAR PAPER '4'\tMAGIC METAL '5'\tELIT METAL '6'"
        );
        let mut inp = inputFunc_int();
        if inp == 1 {
            self.kutsama_plus_value();
        } else if inp == 2 {
            self.demirci_plus_value();
        } else if inp == 3 {
            self.ejderha_plus_value();
        } else if inp == 4 {
            if self.plus > 3 {
                println!("You can't use War Paper after +4");
            } else {
                self.savasKagidi_plus_value();
            }
        } else if inp == 5 {
            self.metal_plus_value();
        } else if inp == 6 {
            self.elitMetal_plus_value();
        } else {
            println!("Wrong entry!")
        }
    }
    fn kutsama_plus_value(&mut self) {
        let z = rand::thread_rng().gen_weighted_bool(u32::pow(self.chance, self.plus));
        if z {
            self.plus += 1;
            println!("SUCCESFULL");
        } else {
            self.plus -= 1;
            println!("UNSUCCESFULL");
        }
    }
    fn demirci_plus_value(&mut self) {
        let mut z = rand::thread_rng().gen_weighted_bool(u32::pow(self.chance, self.plus));
        if z == false {
            z = rand::thread_rng().gen_weighted_bool(5);
        }
        if z {
            self.plus += 1;
            println!("SUCCESFULL");
        } else {
            self.plus -= 1;
            println!("UNSUCCESFULL");
        }
    }
    fn ejderha_plus_value(&mut self) {
        let mut z = rand::thread_rng().gen_weighted_bool(u32::pow(self.chance, self.plus));
        if z == false {
            z = rand::thread_rng().gen_weighted_bool(10);
        }
        if z {
            self.plus += 1;
            println!("SUCCESFULL");
        } else {
            self.plus -= 1;
            println!("UNSUCCESFULL");
        }
    }
    fn savasKagidi_plus_value(&mut self) {
        self.plus = self.plus + 1;
        println!("SUCCESFULL");
    }
    fn metal_plus_value(&mut self) {
        let z = rand::thread_rng().gen_weighted_bool(u32::pow(self.chance, self.plus));

        if z {
            self.plus += 1;
            println!("SUCCESFULL");
        } else {
            self.plus += 0;
            println!("UNSUCCESFULL");
        }
    }
    fn elitMetal_plus_value(&mut self) {
        let mut z = rand::thread_rng().gen_weighted_bool(u32::pow(self.chance, self.plus));
        if z != true {
            z = rand::thread_rng().gen_weighted_bool(4);
        }
        if z {
            self.plus += 1;
            println!("SUCCESFULL");
        } else {
            self.plus += 0;
            println!("UNSUCCESFULL");
        }
    }
}
