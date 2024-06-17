#!/bin/bash
if [ `id -u` -ne 0 ]
  then echo Please run this program as root or using sudo for this functionality!
  exit 1
fi

# from https://aur.chaotic.cx/docs

pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
pacman-key --lsign-key 3056513887B78AEB
pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'

echo "[chaotic-aur]\nInclude = /etc/pacman.d/chaotic-mirrorlist" >> /etc/pacman.conf
