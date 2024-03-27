use std::io;
use std::process::Command;
use std::{thread, time};


fn main() {
    println!(r"

________  ___  ___  ________  _________    ___    ___ ___  __    ________   ___  ________ _______      
|\   __  \|\  \|\  \|\   ____\|\___   ___\ |\  \  /  /|\  \|\  \ |\   ___  \|\  \|\  _____\\  ___ \     
\ \  \|\  \ \  \\\  \ \  \___|\|___ \  \_| \ \  \/  / | \  \/  /|\ \  \\ \  \ \  \ \  \__/\ \   __/|    
 \ \   _  _\ \  \\\  \ \_____  \   \ \  \   \ \    / / \ \   ___  \ \  \\ \  \ \  \ \   __\\ \  \_|/__  
  \ \  \\  \\ \  \\\  \|____|\  \   \ \  \   \/  /  /   \ \  \\ \  \ \  \\ \  \ \  \ \  \_| \ \  \_|\ \ 
   \ \__\\ _\\ \_______\____\_\  \   \ \__\__/  / /      \ \__\\ \__\ \__\\ \__\ \__\ \__\   \ \_______\
    \|__|\|__|\|_______|\_________\   \|__|\___/ /        \|__| \|__|\|__| \|__|\|__|\|__|    \|_______|
                       \|_________|       \|___|/                                                       
                                                                                                        
                                                                                                        

             ");

    println!("Please choose a section below: ");
    println!("------------------------------");
    println!("Wi-Fi related(1)");
    println!("------------------------------");
    println!("Website related(2)");
    println!("------------------------------");
    println!("Social-Engineering related(3)");
    println!("------------------------------");
    println!("Malware(4)");
    println!("------------------------------");

    let mut tmp = String::new();
    io::stdin().read_line(& mut tmp).unwrap();
        let sect: i32 = tmp.trim().parse().expect("Error");


    

    if sect == 1 {
        wifisection();
    }

    if sect == 2 {
        websitesection();
    }
    if sect == 3 {
        socengsection();

    }
    if sect == 4 {
        malwaresection();
    }




}













pub fn wifisection() {
    Command::new("clear")
        .spawn();
        
    let wait = time::Duration::from_millis(50);
    thread::sleep(wait);






    println!("Select an attack:  ");
    println!("-----------------------");
    println!("Simple Wi-Fi networks scan(1)");
    println!("-----------------------");
    println!("Evil Twin Hotspot(2)");
    println!("-----------------------");
    println!("Deauthentication Signal Attack(3)");
    println!("-----------------------");
    println!("ARP Spoofing using Bettercap(4)");
    println!("-----------------------");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    let choice: i32 = temp.trim().parse().expect("Error");


    if choice == 1 {
        wifinetworkscan();
    }
    if choice == 2 {
        eviltwinhotspot();
    }
    if choice == 3 {
        deauthsignal();
    }
    if choice == 4 {
        arpspoofing();
    }

}


pub fn websitesection() {
    println!("website");

}


pub fn socengsection() {
    println!("soceng");
}

pub fn malwaresection() {
    println!("malware");

}

pub fn clearscreen() {
    Command::new("clear")
        .spawn();

    let wait = time::Duration::from_millis(50);
    thread::sleep(wait);
}





pub fn wifinetworkscan() {
    clearscreen();    
    println!("A Scan will begin");
    Command::new("sudo")
        .args(["konsole", "--noclose", "-e", "bash", "wifinetworkscan.sh"])
        .spawn();

}

pub fn eviltwinhotspot() {
    clearscreen();
    Command::new("sudo")
        .args(["konsole", "--noclose","-e","bash","eviltwinhotspot.sh"])
        .spawn();


}


pub fn deauthsignal() {
    clearscreen();
    Command::new("sudo")
        .args(["konsole", "--noclose", "-e", "bash", "deauthsignal.sh" ])
        .spawn();
}

pub fn arpspoofing() {
    clearscreen();
    println!("Follow these steps");
    println!("net.recon on  |  net.show  | set arp.spoof.target *TARGETIP* | arp.spoof on");
    let wait = time::Duration::from_millis(300);
    thread::sleep(wait);


    Command::new("sudo")
        .args(["konsole", "--noclose", "-e", "bash", "arpspoof.sh"])
        .spawn();

}


