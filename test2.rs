fn print_number(a: &[i8]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
    }
}

fn main() {
 let mut x: [i8; 10] = [1,2,3,4,5,6,7,8,9,0];
 let mut y: [i8; 10] = [1,2,3,4,5,6,7,8,9,0];
 let mut r: [i8; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
 let mut temp: i8 = 0;

 for i in 0..x.len() {
    for j in 0..y.len() {
        temp = x[i]*y[j];
        r[i+j] += temp % 10;
        if r[i+j] > 9 {
            r[i+j+1] += r[i+j] / 10; 
            r[i+j] += r[i+j] % 10;
        }
        r[i+j+1] += temp/10;
        if r[i+j+1] > 9 {
            r[i+j+2] += r[i+j+1] / 10; 
            r[i+j+1] += r[i+j+1] % 10; 
        }
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

