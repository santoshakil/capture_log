fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    capture_log::get_chrome_history(args[1].as_str());

    capture_log::get_running_processes();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
