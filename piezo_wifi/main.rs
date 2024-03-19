use std::io;
use std::{thread, time};
fn main() {
use std::process::Command;
Command::new("clear")
	.spawn();

let mut ten_mils = time::Duration::from_millis(100);
thread::sleep(ten_mils);

println!(r" ________  ___  _______   ________  ________     
                |\   __  \|\  \|\  ___ \ |\_____  \|\   __  \    
                \ \  \|\  \ \  \ \   __/| \|___/  /\ \  \|\  \   
                \ \   ____\ \  \ \  \_|/__   /  / /\ \  \\\  \  
                \ \  \___|\ \  \ \  \_|\ \ /  /_/__\ \  \\\  \ 
                \ \__\    \ \__\ \_______\\________\ \_______\
                \|__|     \|__|\|_______|\|_______|\|_______|  wifi version
            ");
println!("Welcome to Piezo, the wifi testing toolkit");
println!("Please enter your Wireless interface below: ");

let mut interface = String::new();
io::stdin().read_line(& mut interface).unwrap();

thread::sleep(ten_mils);

println!("Please choose a module below: ");
println!("------------------------------");
println!("Simple Wi-Fi scan(1)");
println!("------------------------------");
println!("Evil Twin hotspot(2)");
println!("------------------------------");
println!("Deauthentication Attack(3)");
println!("------------------------------");
println!("ARP spoofing utilizing bettercap(4)");
println!("------------------------------");

let mut attack = String::new();
io::stdin().read_line(&mut attack).unwrap();
let atnum: i32 = attack.trim().parse().expect("Failed");

if atnum == 1 {
	Command::new("sudo")
		.args(["konsole", "--noclose", "-e", "bash", "wifiscan.sh"])
		.spawn();
}
if atnum == 2 {
	Command::new("sudo")
		.args(["konsole", "--noclose", "-e", "bash", "eviltwin.sh"])
		.spawn();
}

if atnum == 3 {
	Command::new("sudo")
		.args(["konsole", "--noclose", "-e", "bash", "deauth.sh"])
		.spawn();
}

if atnum == 4 {
	println!("You need to do some stuff manually, so enter the following command, *net.recon on*,*net.show*(to see avaible devices), *set arp.spoof.duplex true*, *set arp.spoof.target TARGETIP*, *arp.spoof on*");
	Command::new("sudo")
		.args(["konsole", "--noclose", "-e", "bash", "arpspoof.sh"])
		.spawn();
}



}