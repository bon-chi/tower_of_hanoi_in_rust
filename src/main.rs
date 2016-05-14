use std::iter::Map;
use std::ops::RangeFrom;

fn h_move(n: i32, from: usize, to: usize) {
    let towers = ["LEFT", "CENTER", "RIGHT"];
    let temp = 3 - (to + from);
    if n == 0 {
        return;
    }
    h_move(n - 1, from, temp);
    println!("move {} from {} to {}", n, towers[from], towers[to]);
    h_move(n - 1, temp, to);
}

fn hanoi_stream(n: i32, stream: Vec<i32>) -> Vec<i32> {
    if n == 0 {
        return vec![];
    }
    let h1 = stream.clone();
    let h2 = stream.clone();

    let mut new_stream: &mut Vec<i32> = &mut hanoi_stream(n - 1, h1);
    new_stream.push(n);
    new_stream.append(&mut hanoi_stream(n - 1, h2));

    let new_stream2: Vec<i32> = new_stream.clone();
    return new_stream2;
}

fn stream_nth(n: i32, stream: &Vec<i32>) -> Option<&i32> {
    return stream.get(n as usize - 1);
}

fn nth_hanoi_element(n: i32) -> i32 {
    match stream_nth(n, &hanoi_stream(n, vec![])) {
        Some(x) => return x.clone(),
        None => return 0,
    }
}

fn hanoi() -> Map<RangeFrom<i32>, fn(i32) -> i32> {
    return (1..).map(nth_hanoi_element);
}

fn main() {
    let disk_numbers = 3;
    let n = 10;
    let from = 0;
    let to = 2;
    h_move(disk_numbers, from, to);
    println!("{:?}", hanoi_stream(disk_numbers, vec![]));
    println!("{}", nth_hanoi_element(n));
    println!("{:?}", hanoi().take(n as usize).collect::<Vec<i32>>());
}
