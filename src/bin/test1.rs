use std::time::Instant;

fn main() {
    const FROM: u32 = 1000000000;
    const TO: u32 = 1000100000;
    let limit: u32 = (TO as f64).sqrt() as u32;
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
    

    for n in FROM..TO {
        if a[(n - FROM) as usize] == 0 {
            println!("{}",n);
        }
    }

    println!("time = {}", elapsed.as_millis());
}