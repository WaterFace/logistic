use std::fmt::Write;

#[allow(unused)]
pub fn format_number(x: f64) -> String {
    let mut buf = String::new();
    write_format_number(&mut buf, x).unwrap();
    buf
}

#[allow(unused)]
pub fn write_format_number(w: &mut impl Write, x: f64) -> Result<(), std::fmt::Error> {
    if x < 1.0 {
        write!(w, "{}", x)
    } else if x < 1e3 {
        write!(w, "{:0.0}", x)
    } else if x < 1e6 {
        write!(w, "{:0.1} K", x / 1e3)
    } else if x < 1e9 {
        write!(w, "{:0.1} M", x / 1e6)
    } else {
        write!(w, "{:0.3e}", x)
    }
}

#[allow(unused)]
pub fn format_quantity(x: crate::quantity::Quantity) -> String {
    let mut buf = String::new();
    write_format_quantity(&mut buf, x).unwrap();
    buf
}

#[allow(unused)]
pub fn write_format_quantity(
    w: &mut impl Write,
    x: crate::quantity::Quantity,
) -> Result<(), std::fmt::Error> {
    let x = x.value();
    write_format_number(w, x)
}
