#!/bin/bash

sudo ip tuntap add mode tap name tap-rust user $USER 

sudo ip link set tap-rust up

sudo ip addr add 192.168.42.100/24 dev tap-rust

sudo iptables -t nat -A POSTROUTING -s 192.168.42.0/24 -j MASQUERADE

sudo sysctl net.ipv4.ip_forward=1

# delete tap interface
# sudo iptuntap del mode tap name tap-rust