fn main() {
    let mut a: [i8; 10] = [42; 10]; // 42, 42 ,42 ,42 ..., 42
    a[5] = 0;
    println!("a: {:?}", a);
    let t: (i8, bool) = (7, false);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
    _ref();
    // DangleRef();
    _slices();
    _Stringvsstr();
}

fn _ref() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x; 
    *ref_x = 20;
    println!("x: {}", x);
}

// fn DangleRef() { // error occur
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         ref_x = &x;
//     }
//     println!("ref_x = {ref_x}");
// }

fn _slices() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}


fn _Stringvsstr() { // &str -> const char*, String -> std::string
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}