use std::thread;

pub fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}
