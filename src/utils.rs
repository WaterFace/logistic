use std::fmt::Write;

pub fn format_f64(x: f64) -> String {
    let mut buf = String::new();
    write_format_f64(&mut buf, x).unwrap();
    buf
}

pub fn write_format_f64(w: &mut impl Write, x: f64) -> Result<(), std::fmt::Error> {
    write!(w, "{:0.1}", x)
}
