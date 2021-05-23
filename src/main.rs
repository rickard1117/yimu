mod buffer;
mod file;
mod logger;
mod macros;

fn main() {
    let content = "abcd";
    let content2 = String::from("abcd");
    debug!("{}", "content2");
    debug!("{}", content);
    debug!("{}", content2);
    logger::Logger::setLevel(logger::LogLevel::INFO);
    debug!("abcefg");
    debug!("{}", content2);
}
