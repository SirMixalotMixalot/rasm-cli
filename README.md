# rasm-cli
An  interpreter for the 'reduced' assembly language for AS level

## Current State
- Poorly designed
- Not optimized
- Current Textual Interface leaves alot to be desired

## Installaion
Head over to ['Releases'](https://github.com/SirMixalotMixalot/rasm-cli/releases) and download the rasm.zip  
Unzip it to a directory of your choice  
Go to that directory and on the command line (cmd/bash/etc...) type `./rasm-cli example.rasm` and it should work

# Goals
- Refactor code
  - [X] Stage 1 - Splitting code up into units with a lower cohesion
  - [ ] Stage 2 - Add tests
- [ ] Improve tui using [tui-rs](https://github.com/fdehau/tui-rs)
- [ ] Improve parsing of asm file 
- [ ] Build for x86_64 Apple

