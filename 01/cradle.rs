use std::io;
use std::io::Read;
use std::process;

static mut Look: char = '\0';

/**
 * Read new character from input stream.
 */
fn get_char() {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    unsafe {
        Look = buffer[0] as char;
    }
}

/**
 * Report an error.
 */
fn error(s: &str) {
    println!("{}", s);
}

/**
 * Report error and halt.
 */
fn abort(s: &str) {
    error(s);
    process::exit(1);
}

/**
 * Report what was expected.
 */
fn expected(s: &str) {
    abort(&(s.to_string() + " Expected"));
}

/**
 * Match a specific input character.
 */
fn match_(x: char) {
    unsafe {
        if Look == x {
            get_char();
        }
    }
    expected(&format!("'{}'", x));
}

/**
 * To upper case.
 */
fn upper(c: char) -> char {
    let a = c as u8;
    if a >= 97 {
        c
    } else {
        (a + 32) as char
    }
}

/**
 * Recognize an alpha character.
 */
fn is_alpha(c: char) -> bool {
    let a = upper(c) as u8;
    a >= 97 && a <= 122
}

/**
 * Recognize a decimal digit.
 */
fn is_digit(c: char) -> bool {
    let a = c as u8;
    a >= 48 && a <= 57
}

/**
 * Get an identifier.
 */
fn get_name() -> char {
    let look: char;
    unsafe {
        look = Look;
    }

    if !is_alpha(look) {
        expected("Name");
    }

    get_char();
    upper(look)
}

/**
 * Get a number.
 */
fn get_num() -> char {
    let look: char;
    unsafe {
        look = Look;
    }

    if !is_digit(look) {
        expected("Integer");
    }

    get_char();
    look
}

/**
 * Output a string with tab.
 */
fn emit(s: &str) {
    print!("\t{}", s);
}

/**
 * Output a string with tab and a new line.
 */
fn emitln(s: &str) {
    println!("\t{}", s);
}

/**
 * Initialize.
 */
fn init() {
    get_char();
}

/**
 * Parse and translate a math expression.
 */
fn expression() {
    let text = format!("MOVE #{},D0", get_num());
    emitln(&text);
}

fn main() {
    init();
    expression();
}
