
# Cargo winrun

1. Make sure you run WSL version 2.
1. Install with `cargo install --git https://github.com/zylkowski/cargo-winrun.git`.
2. Make sure you have `x86_64-pc-windows-gnu` target installed for rustc.
3. Make sure you have `gcc-mingw-w64-x86-64` installed.
4. Make sure (on the Windows side) that you have proper `.dll`s installed. 
You can easily tell if you just copy over the `.exe` and try to run in by hand.
On Windows missing `.dll`s errors might not be visible in the console but in a popup window after startup.

In order to pass temporary env vars call the function like this `WIN_MY_ENV_VAR=true cargo winrun`.
You need to prepend all your env vars with `WIN_` otherwise they are not going to be passed.



