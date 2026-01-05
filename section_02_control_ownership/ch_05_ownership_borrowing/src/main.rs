/*
fn main() {
    // Ownership: การเป็นเจ้าของ
    let s1 = String::from("Hello"); // s1 เป็นเจ้าอขง String "Hello"
    println!("s1: {}", s1);
    // let s2 = s1; // s2 กลายเป็นเจ้าของ String "Hello" ที่เดิม
    // // s1 ไม่สามารถใช้งานได้แล้ว
    // println!("s2: {}", s2);
    // // println!("{}", s1); // Error ไม่สามารถใช้งาน s1 ได้

    new_topic();
    takes_ownership(s1);
    // println!("{}", s1); // Error ไม่สามารถใช้งาน s1 ไม่สามารถใช้งานได้แล้ว

    new_topic();
    let x = 5;
    makes_copy(x);
    println!("x is still accessible {}", x); // สามารถใช้งานได้
}

#[allow(unused)]
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string ไม่สามารถใช้งานได้แล้ว เพราะหลุดออกจาก scope, หน่วยความจำถูกคืน

#[allow(unused)]
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer สามารถใช้งานได้เพราะข้อมูลอยู่บน stack

#[allow(unused)]
fn new_topic() {
    println!("{}", "-".repeat(40));
}
*/

// Borrowing: การยืม
// การยืมมี 2 แบบ
// 1. ยืมแบบไม่เปลี่ยนแปลง (immutable borrow)
// 2. ยืมแบบเปลี่ยนแปลง (mutable borrow)

/*
// 1. การยืมแบบไม่เปลี่ยนแปลง (immutable borrow)
fn main() {
    let s = String::from("Hello");
    let len = calculate_lenght(&s); // ยืม s แบบ immutable
    println!("{}", len);
    println!("{}", s);
}

#[allow(unused)]
fn calculate_lenght(s: &String) -> usize {
    // s เป็น refference (การยืม) ของ String
    s.len()
} // s หลุดออกจาก scope, แต่ไม่ได้เป็นเจ้าของค่า จึงไม่มีอะไรถูกทิ้ง
*/

// 2. การยืมแบบเปลี่ยนแปลง (mutable borrow)
fn main() {
    let mut s = String::from("Hello");
    change(&mut s); // ยืม s แบบ mutable
    println!("{}", s); // s ถูกแก้ไขแล้ว

    let mut r = String::from("สวัสดี");
    change(&mut r); // ยืม r แบบ mutable
    println!("{}", r); // r ถูกแก้ไขแล้ว
}

fn change(some_string: &mut String) {
    some_string.push_str(" World");
} // some_string หลุดออกจาก scope
