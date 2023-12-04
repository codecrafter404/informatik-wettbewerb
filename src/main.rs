use std::io::Write;

fn main() {
    let n1 = take_input();
    let n2 = take_input();
    println!(
        "{}*{} = {} | Probe: {}",
        n1,
        n2,
        n1 * n2,
        if probe(n1, n2) {
            "successful"
        } else {
            "failed"
        }
    );
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
    std::io::stdout().flush();
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
