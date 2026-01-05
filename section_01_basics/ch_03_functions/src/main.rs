fn main() {
    println!("message from main function");
    // การเรียกใช้ฟังก์ชัน (Calling function)
    println!("{}", "-".repeat(40));
    another_function();
    // การเรียกใช้ฟังก์ชันที่มีพารามิเตอร์ (Calling function with parameters)
    println!("{}", "-".repeat(40));
    print_sum(10, 20);
    // การเรียกใช้ฟังก์ชันที่มีค่าส่งกลับ (Calling function with return value)
    println!("{}", "-".repeat(40));
    let result = subtract(10, 20);
    println!("Result of subtract: {}", result);
    // การคืนค่าแบบแฝง (Returning value implicitly)
    println!("{}", "-".repeat(40));
    let result = multiply(10, 20);
    println!("Result of multiply: {}", result);

    // การประเมินค่าของบล็อค (Block evaluation)
    println!("{}", "-".repeat(40));
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("value of x: {}", x);
    println!("value of y: {}", y);
}

// การประกาศฟังก์ชัน (Declaring function)
fn another_function() {
    println!("message from another_function");
}

// การประกาศฟังก์ชันที่มีพารามิเตอร์ (Declaring function with parameters)
fn print_sum(x: i32, y: i32) {
    println!("Sum value: {} + {} = {}", x, y, x + y);
}

// การประกาศฟังก์ชันที่มีค่าส่งกลับ (Declaring function with return value)
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

// การคืนค่าแบบแฝง (Returning value implicitly)
fn multiply(x: i32, y: i32) -> i32 {
    x * y // ไม่ต้องใช้ return และไม่ต้องใช้ ; ปดิท้ายบรรทัด
}
