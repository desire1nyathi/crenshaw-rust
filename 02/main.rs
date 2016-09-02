mod cradle;

use cradle::emitln;
use cradle::expected;
use cradle::get_num;
use cradle::init;
use cradle::Look;
use cradle::match_;

fn add() {
    match_('+');
    term();
    emitln("ADD D1,D0");
}

fn subtract() {
    match_('-');
    term();
    emitln("SUB D1,D0");
}

fn term() {
    let line = format!("MOVE #{},D0", get_num());
    emitln(&line);
}

/**
 * Parse and translate a math expression.
 */
fn expression() {
    term();
    emitln("MOVE D0,D1");

    let look: char;
    unsafe {
        look = Look;
    }

    match look {
        '+' => add(),
        '-' => subtract(),
        _ => expected("Addop"),
    }
}

fn main() {
    init();
    expression();
}
