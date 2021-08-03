use std::{
    env,
    io::{stdout, BufWriter, Write},
};
fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        // TODO error の実装
        println!("enter a file name");
        return;
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", dhash::calculate(&args[1])).unwrap();
}
