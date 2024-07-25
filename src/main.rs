#![windows_subsystem = "windows"]

use duct::{cmd, Expression};
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub trait ExpressionExt {
    fn create_no_window(&self) -> Expression;
}

impl ExpressionExt for Expression {
    fn create_no_window(&self) -> Expression {
        self.before_spawn(|cmd| {
            #[cfg(windows)]
            cmd.creation_flags(CREATE_NO_WINDOW);
            Ok(())
        })
    }
}

const PYTHON_PROGRAM: &str = r#"
import time
import sys

print("Here is some text you may see as stdout.")
print("Here is some text you may see as stderr.", file=sys.stderr)
print("This program run should end within about 5 seconds.")

time.sleep(5)
"#;

fn main() {
    // Shows a console window.
    cmd!("python", "-c", PYTHON_PROGRAM).run().unwrap();

    // Doesn't show a console window.
    // For example notice how 'cargo run --release' doesn't end until 5 seconds
    // after the previous program ended (verified by seeing the previous console window terminate).
    cmd!("python", "-c", PYTHON_PROGRAM)
        .create_no_window()
        .run()
        .unwrap();
}
