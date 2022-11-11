extern crate termsize;

pub fn cls_string() -> String {
    match termsize::get() {
        Some(size) => "\n".repeat(size.rows.try_into().unwrap_or(50)),
        None => "\n".repeat(50)
    }
}
