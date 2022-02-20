# minutils  
A minimal rewrite of (most) the GNU coreutils in Rust  
## Notes:  
 - I'm not going to rewrite `rmdir` because `rm -r` replaces it, like come on, you don't need another binary for deleting directories when `rm -r` exists!  