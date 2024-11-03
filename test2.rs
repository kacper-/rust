fn print_number(a: &[i8]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
    }
}

fn main() {
 let x: [i8; 10] = [1,2,3,4,5,6,7,8,9,0];
 let y: [i8; 10] = [1,2,3,4,5,6,7,8,9,0];
 let r: [i8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

 for i in 0..x.len() {
    for j in 0..y.len() {

    }
 }
 print!("x = ");
 print_number(&x);
 println!("");
 print!("y = ");
 print_number(&y);
 println!("");
 print!("r = ");
 print_number(&r);
 println!("");
}

