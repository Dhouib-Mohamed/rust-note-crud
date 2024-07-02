pub(crate) fn error(message: &str) {
    eprintln!("\x1b[1;31mError:\x1b[0m {}", message);
}

pub(crate) fn panic(message: &str) -> ! {
    panic!("{}",message);
}
pub(crate) fn warning(message: &str) {
    eprintln!("\x1b[1;33mWarning:\x1b[0m {}", message);
}

pub(crate) fn info(message: &str) {
    eprintln!("\x1b[1;34mInfo:\x1b[0m {}", message);
}

pub(crate) fn debug(message: &str) {
    eprintln!("\x1b[1;32mSuccess:\x1b[0m {}", message);
}

