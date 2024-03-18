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
                \|__|     \|__|\|_______|\|_______|\|_______| 
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


let mut attack = String::new();
io::stdin().read_line(& mut attack).unwrap();
let num: i32 = attack.trim().parse().expect("Syntax Error");




if num == 1 {
	let com1 = "airodump-ng";
	let konsol = "konsole";
	let int = [" {} ", &interface];
	Command::new("sudo")
		.arg(konsol)
		.arg("--noclose")
		.arg("-e")
		.arg(com1)
		.args(&int)
		.spawn();
}
if num == 2 {
	let com2 = "airbase-ng";
	let com1 = "airodump-ng";
	let konsol = "konsole";
	let int = [" {} ", &interface];
	println!("Please choose the BSSID of the target router");
	Command::new("sudo")
		
		.arg(konsol)
		.arg("--noclose")
		.arg("-e")
		.arg(com1)
		.arg("wlan0")
		.spawn();
		thread::sleep(ten_mils);
		let mut bssid = String::new();
		io::stdin().read_line(& mut bssid).unwrap();



	Command::new("sudo")
		.arg(konsol)
		.arg("--noclose")
		.arg("-e")
		.arg(com2)
		.arg("-a")
		.arg(bssid)
		.arg("-e")
		.arg("FreeWifi")
		.arg("-c")
		.arg("6")
		.args(&int)

		.args(&int)
		.spawn();

}

else {
	println!("Please choose the correct module number");
}


}