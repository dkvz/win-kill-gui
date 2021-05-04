# win-kill-gui
Enter the PID of some program, you can now forcefully kill it by pressing the button. The "k" key also does it, provided the text field doesn't have the focus - I know it works after an alt+tab which is what I needed.

I made this to be able to kill old games that freeze in fullscreen and make it impossible to interact with the task manager making me have to "log out", closing all the other open programs.

Careful that this is forceful kill, can technically corrupt saves.

Meant to be built on Windows, for Windows since I'm using the native winapi bindings.

I use the GNU toolchain to compile instead of MSVC and have no issues, I probably won't test building with MSVC at all.

## Resources
* Crate doc: https://docs.rs/native-windows-gui/1.0.11/native_windows_gui
* Even better: https://gabdube.github.io/native-windows-gui/native-windows-docs/getting_started.html

# TODO
- [x] I need some kind of "hotkey" shortcut or whatnot
- [ ] The "hotkey" won't always fire depending on what's got the focus, also I can't use Ctrl+something
- [ ] Should spawn at the bottom of the screen
- [ ] Add some thingy to first try non-forceful kill, then do forceful kill