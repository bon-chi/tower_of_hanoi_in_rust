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

fn main() {
    let disk_numbers = 10;
    let from = 0;
    let to = 2;
    h_move(disk_numbers, from, to);
}
