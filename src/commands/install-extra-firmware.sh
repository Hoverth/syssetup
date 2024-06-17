#!/bin/bash

# arch by default, may change out later
if grep -q -F "Include = /etc/pacman.d/chaotic-mirrorlist" "/etc/pacman.conf"; then
  echo "Using chaotic-aur package..."
  if [ `id -u` -ne 0 ]
    then 
    echo Please run this program as root or using sudo for this functionality!
    exit 1
  fi
  pacman --noconfirm -Sy mkinitcpio-firmware
else
  if ! command -v yay &> /dev/null
    then
    yay -S --noconfirm mkinitcpio-firmware
  else
    echo 'Chaotic AUR not found, please use a AUR helper to install `mkinitcpio-firmware` for you, or do it manually.'
  fi
fi
