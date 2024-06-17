#!/bin/bash
if [ `id -u` -ne 0 ]
  then echo Please run this program as root or using sudo for this functionality!
  exit 1
fi

# arch by default, may change out later
pacman --noconfirm -Sy akregator antimicrox ark audacity blender calibre digikam filelight gimp goldendict gwenview inkscape kate kdeconnect sshfs xdg-desktop-portal kdenlive keepassxc kicad krita konsole libreoffice-fresh librewolf lutris mpv okular onboard qalculate-gtk qbittorrent qpwgraph steam texstudio thunderbird

# if chaotic aur
if grep -q -F "Include = /etc/pacman.d/chaotic-mirrorlist" "/etc/pacman.conf"; then
  pacman --noconfirm -Sy czkawka-gui
else
  echo 'Chaotic AUR not found, no czkawka-gui for you.'
fi
