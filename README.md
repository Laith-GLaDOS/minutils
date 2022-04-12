# minutils  
A minimal rewrite of (most) the GNU coreutils in Rust  
## Notes:  
 - I'm not going to rewrite `rmdir` because `rm -r` replaces it, like come on, you don't need another binary for deleting directories when `rm -r` exists!  
 - I'm going to try and add Windows builds  
# Building  
Make sure you have the `rustup` stable toolchain installed  
Step 1: `git clone https://github.com/Laith-GLaDOS/minutils.git`  
Step 2: `cd minutils`  
Step 3: `./build_tar (version, eg 1.2.3) (architecture)`  
