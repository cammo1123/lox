pub static mut HAD_ERROR: bool = false;

pub fn error(line: usize, message: &str) {
	report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
	eprintln!("[line {line}] Error{where_}: {message}");
	unsafe {
		HAD_ERROR = true;
	}
}	