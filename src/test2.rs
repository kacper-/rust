fn print_number(a: &[i8]) {
    for i in 0..a.len() {
        print!("{}", a[i]);
    }
    println!("");
}

fn fill(a: &mut [i8], s: usize) {
    for i in 0..s {
        a[i] = (i % 10) as i8;
    }
}

fn main() {
 const SIZE: usize = 200000;
 let mut x: [i8; SIZE/2] = [0; SIZE/2];
 let mut y: [i8; SIZE/2] = [0; SIZE/2];
 let mut r: [i8; SIZE] = [0; SIZE];
 let mut kpos: usize;

 fill(&mut x, SIZE/2);
 fill(&mut y, SIZE/2);

 for i in 0..x.len() {
    for j in 0..y.len() {
        r[r.len()-i-j-1] += x[x.len()-i-1]*y[y.len()-j-1];
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

