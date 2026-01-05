fn main() {
    // การแสดงข้อความพื้นฐานขึ้นบรรทัดใหม่
    println!("Hello, world! 888");
    // การแสดงข้อความพื้นฐานบนบรรทัดเดียวกัน
    print!("welcome Rustacean");
    print!("{} days", 21);
    // การแสดงข้อความพื้นฐานบนบรรทัดเดียวกันแล้วขึ้นบรรทัดใหม่
    print!("abc\n");
    // การแสดงข้อความพื้นฐานบนบรรทัดเดียวกันแล้วขยับไปทางขวา แล้วขึ้นบรรทัดใหม่ 2 บรรทัด
    print!("\tdef\n\n");
    // การแทรกค่าลงในข้อความ
    println!("Welcome {} {}", "Seiya", "Aiwa");

    // {0} จะอ้างอิงถึง "Alice"
    // {1} จะอ้างอิงถึง "Bob"
    println!("{0}, that is {1}. {1}, you are {0}", "Alice", "Bob");

    // --- Name Arguments ---
    println!("--- Name Arguments ---");
    println!(
        "{subject} {verb} {object}",
        subject = "the lazy dog",
        verb = "jumps over",
        object = "the fence"
    );

    // --- Formatting Numbers ---
    println!("-- Formatting Numbers --");
    println!("Pi is approximately {pi:.3}", pi = 3.141592);
    println!("Pi is approximately {:.3}", 3.141592);

    // --- Padding ---
    println!("-- Padding --");
    println!("{number:0>5}", number = 1);
    println!("{number:5}", number = 1);
    println!("{number:05}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0^5}", number = 1);
    println!("{number:0^6}", number = 1);
    println!("{number:0^6}", number = 1);
    println!("Binary: {:b}", 32); // แสดงเป็นเลขฐาน 2
    println!("Hexadecimal: {:x}", 32); // แสดงเป็นเลขฐาน 16
    println!("Octal: {:o}", 32); // แสดงเป็นเลขฐาน 8

    println!("Left align '{:<10}'", "Hello"); // จัดชิดซ้าย
    println!("Right align '{:>10}'", "Hello"); // จัดชิดขวา
    println!("Center align '{:^10}'", "Hello"); // จัดกลาง
    println!("Center align '{:0}'", "Hello"); // จัดพอดี

    // การใช้ escape characters
    println!("-- Escape Charactors --");
    println!("Quote: \"Hello, world!\"");
    println!("Backslash: \\ ");
    println!("Newline: \nSecond line");
    println!("Tab: \tTabbed text");
}
