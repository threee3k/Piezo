#! /bin/bash

echo "Please enter your Wireless Interface: "

read int

sudo airodump-ng $int
