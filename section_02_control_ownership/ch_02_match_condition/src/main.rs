fn main() {
    let status_code: u32 = 500;
    match status_code {
        200 => println!("OK"),
        404 => {
            println!("Error: Page not found");
            println!("Please check the URL and try again.");
            report_error_to_system(404);
        }
        500 => {
            println!("Error: Internal Server Error");
            println!("Please try again later.");
            report_error_to_system(500);
        }
        _ => {
            println!("Unknown Status Code: {status_code}");
        }
    }

    // ตัวอย่าง match แบบเงื่อนไข
    println!("{}", "-".repeat(40));
    let http_method = "PUT";
    match http_method {
        "GET" => println!("{} Method Request data...", http_method),
        "POST" | "PUT" => println!("{} Method Send data...", http_method),
        "DELETE" => println!("{} Method Delete data...", http_method),
        _ => println!("Unknown HTTP method"),
    }

    // ตัวอย่าง match แบบเงื่อนไข
    new_topic();
    let day = 1;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }

    // ตัวอย่าง match แบบเงื่อนไข
    new_topic();
    let number = 22;
    match number {
        1..=10 => println!("Number is between 1 and 10"),
        11..=20 => println!("Number is between 11 and 20"),
        21..=30 => println!("Number is between 21 and 30"),
        _ => println!("Number is not between 1 and 30"),
    }

    // Some คือการใช้ Option เพื่อการจัดการกับค่าที่อาจจะมีหรือไม่มี
    // ในที่นี้เราจะใช้ match เพื่อจัดการกับค่าที่เป็น Option
    // โดยจะมีเคสสำหรับค่าที่เป็น Some และเคสสำหรับค่าที่เป็น None
    // จะตก Some เมื่อมีค่า
    // จะตก None เมื่อไม่มีค่า
    new_topic();
    let value = Some(42);
    // ตัวอย่างตกไปที่ None
    // let value: Option<i32> = None;
    match value {
        Some(v) => println!("The value is: {}", v),
        None => println!("No value"),
    }
}

// ฟังก์ชันสมมติสำหรับรายงานข้อผิดพลาด
fn report_error_to_system(code: u32) {
    println!("-> Reporting error code {} to monitoring system...", code)
}

fn new_topic() {
    println!("{}", "-".repeat(40));
}
