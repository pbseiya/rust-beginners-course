// ตัวอย่างโค้ด: การทำงานบน stack

// fn main() {
//     let x = 5; // i32 มีขนาดคงที่, ตัวแปร x อยู่บน stack
//     let y = 10; // i32 มีขนาดคงที่, ตัวแปร y อยู่บน stack
//     let sum = add(x, y); // ค่า x และ y ถูก copy ไปยัง add
//     println!("ผลรวมของ {} และ {} คือ {}", x, y, sum);
// }

// fn add(a: i32, b: i32) -> i32 {
//     let result = a + b; // ตัวแปร result อยู่บน stack
//     result
// } // ตัวแปร result, b, a ถูกทำลายจาก stack

// // ตัวอย่างโค้ด: การทำงานบน heap
// fn main() {
//     let s1 = String::from("Hello"); // String อยู่บน heap
//     let s2 = String::from("World"); // String อยู่บน heap
//     println!("The original strings are: {} {}", s1, s2); // s1 และ s2 ยังอยู่บน stack

//     // Callking the concatenate function
//     let result = concatenate(s1, s2); // ค่า s1 และ s2 ถูก move ไปยัง concatenate
//     println!("The concatenated string is: {}", result); // result เป็น String ใหม่ที่ถูกสร้างขึ้น

//     //s1 และ s2 ถูกย้ายไปยังฟังก์ชัน concatenate แล้ว ดังนั้นไม่สามารถใช้งานได้อีก
//     println!("The original strings are: {} {}", s1, s2); // จะเกิด Error ที่นี่
//     // แต่ข้อมูลใน heap ยังคงอยู่จนกว่าจะไม่มีตัวแปรใด อ้างอิงถึงมันอีกต่อไป
// } // ตัวแปร result ถูกลบออกจาก stack แต่ข้อมูลใน heap ยังคงอยู่

// #[allow(dead_code)]
// fn concatenate(a: String, b: String) -> String {
//     let result = format!("{} {}", a, b); // a,b และ result อยู่บน stack ของฟังก์ชัน concatenate
//     result // คืนค่า result ซึ่งเป็น String ใหม่
// } // ตัวแปร result, b, a ถูกทำลายจาก stack

// ตัวอย่างโค้ด: การทำงานกับ heap เพิ่มเติ
fn main() {
    // สร้างข้อมูลบน heap
    // b1 คือ Box<i32> ซึ่งเป็น pointer และถูกเก็บไว้บน Stack
    // ส่วนข้อมูล '5' จริงๆ ถูกเก็บไว้บน Heap
    let b1 = Box::new(5);
    println!("b1 = {}", b1);

    // ลองสร้างข้อมูลที่ซับซ้อนขึ้นมา
    let game = crate_game();
    //ตัวแปร game (struct Game) อยู่บน stack
    //แต่ field ที่ชื่อ 'name' (String) ที่มีข้อมูลจริงๆ ("Minecraft") อยู่บน heap
    println!("Playing {} with score {}", game.name, game.score);
} // เมื่อจบ main, game จะถูก drop -> String ภายในจะคืนหน่วยความจำบน heap
// b1 ก็จะถูก drop -> คืนหน่วยควาาจำของเลข 5 บน heap

struct Game {
    name: String, // String จัดการข้อมูลบน heap
    score: u32,   // u32 อยู่บน Stack (เป็นส่วนหนึ่งของ Struct)
}

fn crate_game() -> Game {
    let game_name = String::from("Minecraft"); // สร้าง String บน heap
    Game {
        name: game_name,
        score: 100,
    }
}
