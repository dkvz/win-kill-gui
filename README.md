# win-kill-gui
Enter the PID of some program, you can now forcefully kill it by pressing the button. 

I made this to be able to kill old games that freeze in fullscreen and make it impossible to interact with the task manager making me have to "log out", closing all the other open programs.

Careful that this is forceful kill, can technically corrupt saves.

Meant to be built on Windows, for Windows. 

I use the GNU toolchain instead of MSVC with no issues, I probably won't test building with MSVC at all.

## Resources
* Crate doc: https://docs.rs/native-windows-gui/1.0.11/native_windows_gui

# TODO
- [ ] I need some kind of "hotkey" shortcut or whatnot
- [ ] Should spawn at the bottom of the screen
- [ ] Add some thingy to first try non-forceful kill, then do forceful kill