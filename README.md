# Suppressing the console window when using duct with `windows_subsystem = "windows"`

This is a demo of a workaround for suppressing the console window when using [duct](https://github.com/oconnor663/duct.rs) with `![windows_subsystem = "windows"]` at the top of `src/main.rs` on Windows. I was able to remove the console window with the [`CREATE_NO_WINDOW`](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#flags) flag by using `before_spawn` (as mentioned in duct issue [#70](https://github.com/oconnor663/duct.rs/issues/70)). Issue [#99](https://github.com/oconnor663/duct.rs/issues/99) is also related as that is where the example code I based this example on is provided from.

This sample code demonstrates a workaround for an issue I was facing when attempting to use `duct` in a [Tauri](https://github.com/tauri-apps/tauri) app but realizing that the console windows were appearing when I did not want them to.

There may be a more elegant solution that I am unware of as I have to add the `.create_no_window()` method before each `.run()`.
