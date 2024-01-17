use lista6::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }
    let path0 = args.get(1).unwrap();
    let path1 = args.get(2).unwrap();
    get_errors(path0, path1);
}
