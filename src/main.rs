fn main() {
    capture_log::get_chrome_history(
        "C:\\Users\\AGL\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\History",
    );

    capture_log::get_running_processes();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
