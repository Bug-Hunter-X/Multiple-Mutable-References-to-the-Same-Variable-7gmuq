fn main() {
    let mut x = 5;
    { //create a new scope for y
        let y = &mut x;
        *y += 1;
    }
    { //create a new scope for z
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
