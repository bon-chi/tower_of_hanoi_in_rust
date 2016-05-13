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

fn h_str(n: i32, stream: Vec<i32>) -> Vec<i32> {
    if n == 0 {
        return vec![];
    }
    let h1 = stream.clone();
    let h2 = stream.clone();

    let mut new_stream: &mut Vec<i32> = &mut h_str(n - 1, h1);
    new_stream.push(n);
    new_stream.append(&mut h_str(n - 1, h2));

    let new_stream2: Vec<i32> = new_stream.clone();
    return new_stream2;
}

fn main() {
    let disk_numbers = 5;
    let from = 0;
    let to = 2;
    h_move(disk_numbers, from, to);
    println!("{:?}", h_str(disk_numbers, vec![]));
}
