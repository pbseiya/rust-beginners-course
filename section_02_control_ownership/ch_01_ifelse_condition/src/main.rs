fn main() {
    // ตัวอย่างการใช้ if-else เพื่อเช็คเงื่อนไข
    let number: i32 = 8; // 8,9,10

    if number % 4 == 0 {
        println!("ตัวเลข {} เป็นเลขที่หารด้วย 4 ลงตัว", number);
    } else if number % 3 == 0 {
        println!("ตัวเลข {} เป็นเลขที่หารด้วย 3 ลงตัว", number);
    } else {
        println!("ตัวเลข {} เป็นเลขที่ไม่สามารถหารด้วย 3 หรือ 4 ลงตัว", number);
    }

    println!("{}", "-".repeat(50));
    // การใช้ if ในการกำหนดค่าตัวแปร
    // คล้ายกับการใช้ ternary operator ในภาษาอื่นๆ
    // เช่นใน JavaScript
    // let number = condition ? 4 : 5;
    // ใน Rust ใช้ if-else แทน
    // และต้องมีการระบุทั้งสองกรณี (if และ else)
    // หากไม่ระบุ else จะทำให้เกิด error
    // เนื่องจาก Rust ต้องการให้แน่ใจว่าทุกเส้นทางของโค้ดจะส่งค่ากลับมา
    // ตัวอย่าง
    let condtion: bool = false;
    let value = if condtion { 5 } else { 6 };
    println!("ค่าที่ได้จาก if-else คือ {}", value);
}
