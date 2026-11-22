fn saikat_loop() {
    let mut count = 0;

    loop {
        count += 1;
        println!("{}", count);

        if count == 5 {
            break;
        }
    }
}

fn saikat_loop_return() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 5 {
            break count * 2;
        }
    };

    println!("Result: {}", result);
}

fn saikat_while_loop(){
    let mut count = 1;

    while count <= 5{
        println!("{}",count);
        count    += 1;
    }
}

fn main() {
    let marks = 85;

    if marks >= 80 {
        println!("Grade: A+");
    } else if marks >= 70 {
        println!("Grade: A");
    } else if marks >= 60 {
        println!("Grade: B");
    } else {
        println!("Grade: F");
    }

    let age = 20;
    let status = if age >= 18 { "Adult" } else { "Minor" };

    saikat_loop();
    println!("{}", status);
}