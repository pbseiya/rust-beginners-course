fn main() {
    // การใช้ loop เพื่อทำงานซ้ำ
    let mut counter = 0;
    loop {
        counter += 1;
        println!("รอบที่ {}", counter);
        if counter == 3 {
            println!("ข้ามารอบนี้ไป!");
            continue;
        }
        if counter == 5 {
            println!("จบการทำงานที่รอบที่ {}", counter);
            break;
        }
    }
    new_topic();
    // การใช้ while loop
    let mut counter = 3;
    let max_number = 8;
    while counter < 10 && counter < max_number {
        if counter == 5 {
            println!("ข้ามารอบนี้ไป!");
            counter += 1;
            continue;
        } else if counter == 8 {
            println!("จบการทำงานที่รอบที่ {}", counter);
            break;
        } else {
            println!("ค่าปัจจุบัน {}", counter);
        }
        counter += 1;
    }
    new_topic();
    // การใช้ for loop
    for counter in 1..5 {
        println!("รอบที่ {}", counter);
    }
    new_topic();
    // การใช้ for loop
    for counter in 1..=5 {
        println!("รอบที่ {}", counter);
    }
    new_topic();
    // การใช้ for loop
    for counter in (1..=5).rev() {
        println!("รอบที่ {}", counter);
    }
    println!("จบการทำงานของ Loop");
    new_topic();
    // step by with for loop
    for counter in (2..10).step_by(2) {
        println!("รอบที่ {}", counter);
    }
    new_topic();
    // step by with for loop
    for counter in (2..10).rev().step_by(2) {
        println!("รอบที่ {}", counter);
    }
    new_topic();
    // step by with for loop
    for counter in (2..10).step_by(2).rev() {
        println!("รอบที่ {}", counter);
    }
    new_topic();
    // step by with for loop
    for counter in (2..=10).step_by(2) {
        println!("รอบที่ {}", counter);
    }
}

fn new_topic() {
    println!("{}", "-".repeat(40));
}
