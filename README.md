# set_brightness

A terminal program to set the brightness of my laptop.
This is not guaranteed to work on any other laptop.

To get this to work you have to add your linux user to the video group,
and give the video group permission to write to the brightness file: https://wiki.archlinux.org/title/backlight#Udev_rule

You will have to download and add https://github.com/Epirius/battery-dimmer to your path

Your laptop also has to use intel_backlight
