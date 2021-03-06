fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    while_loop();
    for_loop();
}

fn while_loop() {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the while loop value is {}", a[index]);
        index += 1;
    };
}

fn for_loop() {
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("the for loop value is {}", element)
    };
}