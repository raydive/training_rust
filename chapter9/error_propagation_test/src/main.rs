use std::io;
use std::io::Read;
use std::fs::File;

/*
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:13:39
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

?を使うとFromトレイトに定義されたfrom関数をエラーが通る→from関数で変換できる

*/
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let s = read_username_from_file().unwrap();
    println!("{}", s);
}
