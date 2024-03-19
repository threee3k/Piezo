#! /bin/bash
echo "Please enter your Wireless Interface: "
read INT
sudo bettercap -iface $INT
