extern crate rust_toys; // hypens to underscores

/// Prints a welcome message.
///
/// # A Heading
/// Documentation comments start on a line with three slashes. Comments are
/// written in Markdown, so we can have *bold* and **italic** text, etc.
///
pub fn main() {
    rust_toys::greet();
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_works() {
    }
}
