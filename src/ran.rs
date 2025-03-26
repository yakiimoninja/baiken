use colored::Colorize;
use rand::Rng;

pub async fn ran_p() -> Option<String>{

    let mut rng = rand::rng();

    let chance = rng.random_range(1..101);

    if chance == 1 {
        
        let neco = rng.random_bool(1.0 / 3.0);

        if !neco {
            let number = rng.random_range(1..31);
            println!("EG link: {}", ("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/".to_owned() + &number.to_string() + ".png").yellow());
            Some("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/".to_owned() + &number.to_string() + ".png")
        }
        else{
            let number = rng.random_range(1..31);
            println!("EG link: {}", ("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/neco/".to_owned() + &number.to_string() + ".png").yellow());
            Some("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/f/neco/".to_owned() + &number.to_string() + ".png")
        } 
    }
    else {
        None
    }
}
