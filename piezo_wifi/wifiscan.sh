#! /bin/bash
echo "Please enter your Wireless Interface: "
read INT


sudo airodump-ng $INT
