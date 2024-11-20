fn print_number(a: &[i8]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
    }
    println!("");
}

fn main() {
 let x: [i8; 3] = [1,2,3];
 let y: [i8; 3] = [1,2,3];
 let mut r: [i8; 6] = [0,0,0,0,0,0];
 let mut temp: i8;
 let mut kpos: usize;

 for i in 0..x.len() {
    for j in 0..y.len() {
        temp = x[x.len()-i-1]*y[y.len()-j-1];
        r[r.len()-i-j-1] += temp;
    }
    for k in 0..r.len()-1 {
        kpos = r.len()-k-1;
        r[kpos-1] += r[kpos] / 10;
        r[kpos] = r[kpos] % 10; 
    }
 }
 println!("");  
 print!("x = ");
 print_number(&x);
 print!("y = ");
 print_number(&y);
 print!("r = ");
 print_number(&r);
}

