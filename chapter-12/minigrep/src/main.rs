use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析参数出错：{}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("application err: {}", e);
        process::exit(1);
    }
}
