use rand::{thread_rng, Rng};

fn main() {
    let choices = [true, false];
    let condition = thread_rng().choose(&choices).unwrap();
    //let condition = thread_rng();
    let value = { 
        if *condition {
            5
        }
        else {
            7
        }
    };
    println!("Value equals {}", value);
}

