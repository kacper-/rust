fn main() {
    const FROM: u128 = 200;
    const TO: u128 = 220;
    let limit: u128 = (TO as f64).sqrt() as u128;
    let mut a: [u8; (TO - FROM) as usize] = [0; (TO - FROM) as usize]; 

    for n in 0..(TO - FROM) as usize {
        a[n] = 0;
    }

    for n in FROM..TO {
        for i in 2..limit {
            if n % i == 0 {
                a[(n - FROM) as usize] = 1;
                break;
            }
        }
    }

    for n in FROM..TO {
        if a[(n - FROM) as usize] == 0 {
            println!("{}",n);
        }
    }
}