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
    emitln("ADD (SP)+,D0");
}

fn subtract() {
    match_('-');
    term();
    emitln("SUB (SP)+,D0");
    emitln("NEG D0")
}

fn factor() {
    let look: &char;
    unsafe {
        look = &Look;
    }

    if *look == '(' {
        match_('(');
        expression();
        match_(')');
    } else {
        let line = format!("MOVE #{},D0", get_num());
        emitln(&line);
    }
}

fn multiply() {
    match_('*');
    factor();
    emitln("MULS (SP)+,D0");
}

fn divide() {
    match_('/');
    factor();
    emitln("MOVE (SP)+,D1");
    emitln("DIVS D1,D0");
}

fn term() {
    factor();

    let look: &char;
    unsafe {
        look = &Look;
    }

    while *look == '*' || *look == '/' {
        emitln("MOVE D0,-(SP)");

        match *look {
            '*' => multiply(),
            '/' => divide(),
            _ => expected("Mulop"),
        }
    }
}

fn is_addop(c: char) -> bool {
    c == '+' || c == '-'
}


/**
 * Parse and translate a math expression.
 */
fn expression() {
    let look: &char;
    unsafe {
        look = &Look;
    }

    if is_addop(*look) {
        emitln("CLR D0");
    } else {
        term();
    }

    while is_addop(*look) {
        emitln("MOVE D0,-(SP)");

        match *look {
            '+' => add(),
            '-' => subtract(),
            _ => expected("Addop"),
        }
    }
}

fn main() {
    init();
    expression();
}
