# Linux on the Lenovo Yoga9 14IAP7

I've recently bought a Lenovo Yoga9 14IAP7 to run Linux on without knowing what would work.
I looked into the various nitpicks that were bugging me.
After some messing with the kernel I'm really happy with my device.

## Problems OOTB

There were some obvious problems when first installing Linux on this device:

1. The bass speakers don't work (only sound from the tinny tweeters)
2. Various function keys don't emit any input event's ('Lenovo Support', 'Lenovo Favorite', 'Virtual Background', 'Sound Profile', 'Darkmode Toggle' and the brightness keys)
3. Wayland Desktop Environments experience show graphical artifacts on some
    screen updates.
4. Hibernating the Laptop breaks the sound after resume.

## Solutions

I found solutions for most of them:

1. The bass speakers fix is mainlined (5.19 I think)
2. The function keys:
    - [0008-platform-x86-ideapad-laptop-support-for-more-special.patch](kernel-patches/0008-platform-x86-ideapad-laptop-support-for-more-special.patch)  
        The special function keys of the Yoga 9 are reported using an ACPI WMI event, which had no driver. 
        These are now handled in `ideapad-laptop` in linux mainline.
    - [0001-ACPICA-Make-address-space-handler-install-and-_REG-e.patch](kernel-patches/0001-ACPICA-Make-address-space-handler-install-and-_REG-e.patch)
      & [0002-ACPI-EC-fix-ECDT-probe-ordering-issues.patch](kernel-patches/0002-ACPI-EC-fix-ECDT-probe-ordering-issues.patch)  
        The brightness keys should be supported already but aren't due to a difference in ACPI initialization between Windows and Linux.
        These are now also in mainline linux.
3. The graphical glitches seem to be related to Intel's PSR (Panel Self Refresh)
    implementation. 
    So I disabled it for now using [`/etc/modprobe.d/i915.conf`](config/etc/modprobe.d/i915.conf). 
    This may increase the idle power consumption, so I still hope for a more
    proper solution.

## Extra Features

Digging into the ACPI DSDT and Intel ISH custom sensors revealed some more features which are sometimes only available via Lenovo Vantage on Windows.

- The Laptop has a USB-C quick-charge setting which is controlled by an ACPI function. 
The Bugzilla has an [issue](https://bugzilla.kernel.org/show_bug.cgi?id=216176) for this.
- The Laptop has a WMI device which reports the 'usage mode'.
The usage mode indicates whether the convertible is used as a Laptop, Tent, Tablet or Drawboard.
The [0007-Add-IdeaPad-Usage-Mode-driver.patch](kernel-patches/0007-Add-IdeaPad-Usage-Mode-driver.patch) can emit a `SW_TABLET_MODE` switch event to automatically enable the on-screen-keyboard and screen-auto-rotation in some desktop environments (e.g.
KDE Plasma & GNOME).
This should preferably be fixed in `iio-sensor-proxy` using the hinge sensor data.
There is now an equivalent upstream driver for the tablet mode switch in the linux kernel.
- There are a lot of special sensors built in next to the camera. These can be used to get the proximity from the screen and ambient light brightness.
Lenovo calls this collection of sensors 'Lenovo Intelligent Sensing' and exposes them as 'custom' Intel ISH HID sensors to the system.
The ambient light sensor above the keyboard is also responsible fot the automatic keyboard backlight setting.
It does not seem to do anything in Windows with regard to automatic screen brightness.
I introduced some patches to make the HPD (Human Presence Detection) and ALS work.
  - infrastructure for custom sensors: [0003-HID-hid-sensor-custom-Allow-more-custom-iio-sensors.patch](kernel-patches/0003-HID-hid-sensor-custom-Allow-more-custom-iio-sensors.patch)
  - ambient light: [0005-IIO-hid-sensor-als-Use-generic-usage.patch](kernel-patches/0005-IIO-hid-sensor-als-Use-generic-usage.patch)
  - proximity/HPD: [0006-IIO-hid-sensor-prox-Use-generic-usage.patch](kernel-patches/0006-IIO-hid-sensor-prox-Use-generic-usage.patch)
These are in mainline since Linux 6.3, but seem to be broken due to a Intel ISH/hid-sensor-hub/hid-sensor-custom regression in Linux 6.5.
- The ACPI can also control the keyboard backlight setting (Off/Low/High/Auto) but I have not written a patch for that yet.
Initial support has been merged into Linux 6.6, Linux 6.7 or 6.8 should contain a working driver for the keyboard backlight control from the operating system.

## Bug Reports

I found a lot of useful information and patches in these threads:
- [bass speakers](https://bugzilla.kernel.org/show_bug.cgi?id=208555)  
  Many thanks to all the people sharing their knowledge and readily
  testing my patch!
- [brightness keys](https://bugzilla.kernel.org/show_bug.cgi?id=214899)  
  Many thanks to both the reporter with god-like ACPI debugging skills
  and the kernel developer spending his time on fixing the issue!

## Other Devices

The AMD models have different audio problems than the intel models, check the Bugzilla issue mentioned above.

If you think that the audio fixup may also work for your device, you can apply the
patches yourself and add something like
[`/etc/modprobe.d/snd.conf`](config/etc/modprobe.d/snd.conf)
(without the comments) to the force the fixup on your hardware.

## License

This repository contains patches, which are either derivative work targeting a specific already licensed source, i.e. 
parts of the Linux kernel, or introduce new parts to the Linux kernel. 
These patches fall thus, if not explicitly stated otherwise, under the license of the source they are targeting, or if they introduce new code, the license they explicitly specify inside of the patch. 
Please refer to the specific patch and source in question for further information. 
License texts can be obtained at https://github.com/torvalds/linux/tree/master/LICENSES.

Any other contents of this repository created by me shall be licensed under the MIT license included herein.
