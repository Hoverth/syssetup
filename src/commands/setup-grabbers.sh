#!/bin/bash
if [ `id -u` -ne 0 ]
  then echo Please run this program as root or using sudo for this functionality!
  exit 1
fi

pacman --noconfirm -Sy gallery-dl pipx yt-dlp

## need to do `pipx bandcamp-dl` later
