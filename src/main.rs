mod buffer;
mod file;
mod logger;
mod logging;

fn main() {
    let mut b = buffer::Buffer::new();
    println!("{}", b.len());
    println!("{}", b.capacity());
    b.append("abcdefg".as_bytes());
    println!("{}", b.len());
    println!("{}", b.capacity());

    // let path = String::from("test.txt");
    // let mut f = file::FileUtil::new(&path);

    // f.write("abcdefg".as_bytes()).unwrap();

    // debug!("{}", "abcd");
}
