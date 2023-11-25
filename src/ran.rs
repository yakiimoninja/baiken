use rand::Rng;

pub async fn random_p() -> Option<String>{

    let mut rng = rand::thread_rng();

    let chance: u8 = rng.gen_range(1..101);

    if true {
        
        let extra = rng.gen_bool(1.0 / 3.0);

        if extra {
            let number = rng.gen_range(1..31);
            return Some("data/images/f/".to_owned() + &number.to_string() + ".png");
        }
        else{
            let number = rng.gen_range(1..31);
            return Some("data/images/f/neco/".to_owned() + &number.to_string() + ".png");
        } 
    }
    else {
        None
    }
}