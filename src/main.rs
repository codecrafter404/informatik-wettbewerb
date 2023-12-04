use std::io::Write;

use rand::Rng;

fn main() {
    let mut success = 0;
    for _ in 0..100 {
        let n1 = rand::thread_rng().gen_range(100..1000);
        let n2 = rand::thread_rng().gen_range(100..1000);
        let probe = probe(n1, n2);
        println!("{} * {} = {} | Probe: {}", n1, n2, n1 * n2, probe);
        if probe {
            success += 1;
        }
    }
    println!("----[Results]----");
    println!("{}/100 Probes succeeded ({}%)", success, success)

}
fn probe(n1: i32, n2: i32) -> bool {
    let n1: i32 = n1
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .sum();
    let n2: i32 = n2
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .sum();

    // println!("n1: {} | n2: {}", n1, n2);

    let r1 = n1 % 9;
    let r2 = n2 % 9;

    let top = (r1 * r2) % 9;
    let bottom = (n1 * n2) % 9;
    // println!(
    //     "r1: {} | r2: {} | res: {} | bottom: {}",
    //     r1, r2, top, bottom
    // );
    top == bottom
}
fn take_input() -> i32 {
    print!("Input Number: ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    match buffer.trim().parse::<i32>() {
        Ok(x) => {
            if x >= 100 && x <= 999 {
                return x;
            } else {
            }
        }
        _ => {}
    }
    panic!("Please input a valid number")
}
#[test]
fn test() {
    assert!(probe(911, 340))
}
