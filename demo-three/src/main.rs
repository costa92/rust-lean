fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn main() {
    let pair = (1, true);
    let rs = reverse(pair);
    println!("Pair is {:?}", rs);
}
