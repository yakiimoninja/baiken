use colored::Colorize;
use nanorand::{Rng, WyRand};

pub async fn ran_p() -> Option<String>{

    let mut rng = WyRand::new();

    let chance = rng.generate_range(1_u8..=100);
    println!("chance {}", chance);

    if chance == 1 {
        
        let neco = rng.generate_range(1_u8..=2);
        println!("neco chance {}", neco);

        if neco == 1 {
            let number = rng.generate_range(1_u8..=30);
            println!("number {}", number);
            println!("EG link: {}", ("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/".to_owned() + &number.to_string() + ".png").yellow());
            Some("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/".to_owned() + &number.to_string() + ".png")
        }
        else {
            let number = rng.generate_range(1_u8..=30);
            println!("number {}", number);
            println!("EG link: {}", ("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/neco/".to_owned() + &number.to_string() + ".png").yellow());
            Some("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/neco/".to_owned() + &number.to_string() + ".png")
        } 
    }
    else {
        None
    }
}
