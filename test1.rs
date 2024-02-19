use std::time::Instant;

fn main() {
    const FROM: u128 = 10000000;
    const TO: u128 = 10001000;
    let limit: u128 = (TO as f64).sqrt() as u128;
    let mut a: [u8; (TO - FROM) as usize] = [0; (TO - FROM) as usize]; 

    let start = Instant::now();

    for n in FROM..TO {
        for i in 2..limit {
            if n % i == 0 {
                a[(n - FROM) as usize] = 1;
                break;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("time = {}", elapsed.as_millis());

    for n in FROM..TO {
        if a[(n - FROM) as usize] == 0 {
            println!("{}",n);
        }
    }
}