# Piezo
A Wi-Fi testing tool that runs on rust and bash, So it's pretty fast
# Usage
RUN THE SCRIPT AS ROOT OR IT WILL NOT WORK
cd into the piezo_wifi directory
You can either compile the main.rs file yourself using rustc or just execute the main file directly
If you want to edit the code, edit the main.rs file, compile it, then run it.
# Required stuff
- konsole
- rustc (optional)
- aircrack-ng
- bettercap

# Modules
Currently, it has:
- Simple Wi-Fi scan
- Evil Twin hotspot
- Deauthentication attack
- ARP Spoofing (using bettercap)
