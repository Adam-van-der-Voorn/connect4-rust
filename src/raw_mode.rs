use crossterm::terminal;
pub struct RawModeAnchor;

pub fn set_raw_mode() -> RawModeAnchor {
    terminal::enable_raw_mode().expect("should be able to enable raw mode, this is required to get user input");

    impl Drop for RawModeAnchor {
        fn drop(&mut self) {
            terminal::disable_raw_mode().unwrap_or_else(|_| {
                println!("Failed to disable raw mode. You may want to restart your terminal.")
            });
        }
    }
    return RawModeAnchor {};
}