#!/bin/bash
if [ `id -u` -ne 0 ]
  then echo Please run this program as root or using sudo for this functionality!
  exit 1
fi

# arch by default, may change out later
pacman --noconfirm -Sy neomutt workgrinder
echo ""
echo "iamb is in the rust section!"
