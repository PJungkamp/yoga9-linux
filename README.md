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
        These are now handled in `ideapad-laptop` and on their way to mainline.
    - [0001-ACPICA-Make-address-space-handler-install-and-_REG-e.patch](kernel-patches/0001-ACPICA-Make-address-space-handler-install-and-_REG-e.patch)
      & [0002-ACPI-EC-fix-ECDT-probe-ordering-issues.patch](kernel-patches/0002-ACPI-EC-fix-ECDT-probe-ordering-issues.patch)  
        The brightness keys should be supported already but aren't due to a difference in ACPI initialization between Windows and Linux. 
        The problem is waiting for this MR https://github.com/acpica/acpica/pull/786
3. The graphical glitches seem to be related to Intel's PSR (Panel Self Refresh)
    implementation. 
    So I disabled it for now using [`/etc/modprobe.d/i915.conf`](config/etc/modprobe.d/i915.conf). 
    This may increase the idle power consumption, so I still hope for a more
    proper solution.

## Extra Features

Digging into the ACPI DSDT revealed some more features which are available via Lenovo Vantage on Windows.

- The Laptop has a USB-C quick-charge setting which is controlled by an ACPI function. 
The Bugzilla has an [issue](https://bugzilla.kernel.org/show_bug.cgi?id=216176) for this.
- The Laptop has a WMI device which reports the 'usage mode'.
The usage mode indicates whether the convertible is used as a Laptop, Tent, Tablet or Drawboard.
The [0007-Add-IdeaPad-Usage-Mode-driver.patch](kernel-patches/0007-Add-IdeaPad-Usage-Mode-driver.patch) can emit a `SW_TABLET_MODE` switch event to automatically enable the on-screen-keyboard and screen-auto-rotation in some desktop environments (e.g.
KDE Plasma & GNOME).
This should preferably be fixed in `iio-sensor-proxy` using the hinge sensor data.
- The ACPI can also control the keyboard backlight setting (Off/Low/High/Auto) but I have not written a patch for that yet.

## Bug Reports

I found a lot of useful information and patches in these threads:
- [bass speakers](https://bugzilla.kernel.org/show_bug.cgi?id=208555)  
  Many thanks to all the people sharing their knowledge and readily
  testing my patch!
- [brightness keys](https://bugzilla.kernel.org/show_bug.cgi?id=214899)  
  Many thanks to both the reporter with god-like ACPI debugging skills
  and the kernel developer spending his time on fixing the issue!

## Other Devices

The Lenovo Yoga 7 16IAP7 seems to use the same audio codec.
[0007-ALSA-hda-realtek-Add-quirk-for-Yoga-devices.patch](kernel-patches/0007-ALSA-hda-realtek-Add-quirk-for-Yoga-devices.patch)
adds a quirk to activate the fixup from
[0001-ALSA-hda-realtek-Add-quirk-for-Lenovo-Yoga9-14IAP7.patch](kernel-patches/0001-ALSA-hda-realtek-Add-quirk-for-Lenovo-Yoga9-14IAP7.patch)
on that device.

The AMD models have different audio problems than the intel models, check the Bugzilla issue mentioned above.

If you think that the fixup may also work for your device, you can apply the
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
