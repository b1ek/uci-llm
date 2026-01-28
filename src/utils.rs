pub fn flush() {
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}

#[macro_export]
macro_rules! output {
    ($($arg:tt)*) => {
        print!($($arg)*);
        $crate::utils::flush();
    };
}

#[macro_export]
macro_rules! outputln {
    ($($arg:tt)*) => {        
        println!($($arg)*);
        $crate::utils::flush();
    };
}

pub fn consume_args(mut args: Vec<String>) -> Vec<String> {
    args.remove(0);
    args
}