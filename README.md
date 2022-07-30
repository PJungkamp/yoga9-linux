# Linux on the Lenovo Yoga9 14IAP7

I've recently bought a Lenovo Yoga9 14IAP7 to run Linux on without knowing what
would work.
I looked into the various nitpicks that were bugging me.
After some messing with the kernel I'm really happy with my device.

## Problems OOTB

There were two obvious problems when first installing Linux on this device:

1.  The bass speakers don't work (only sound from the tinny tweeters)
2.  Various function keys don't emit any input event's ('Lenovo Support',
    'Lenovo Favorite', 'Virtual Background', 'Sound Profile', 'Darkmode Toggle'
    and the brightness keys)

## Solutions

I found solutions for both of them:

1.  The bass speakers:
    - [0001-ALSA-hda-realtek-Add-quirk-for-Lenovo-Yoga9-14IAP7.patch](kernel-patches/0001-ALSA-hda-realtek-Add-quirk-for-Lenovo-Yoga9-14IAP7.patch)
        1.  The ALC287 codex has the bass speakers connected to the Pin Complex
            on NID 0x17 which reports itself as unconnected.
        2.  The bass speaker amplifiers need to be enabled using an HDA verb
            sequence which has been extracted by sniffing the Windows drivers
            initialization.
        3.  The HDA driver connects the HDA Pin Complex to the DAC on NID 0x06
            which has no volume control, so the bass speakers would always be on
            maximum volume.
2.  The function keys:
    - [0004-Add-IdeaPad-WMI-Fn-Keys-driver.patch](kernel-patches/0004-Add-IdeaPad-WMI-Fn-Keys-driver.patch)  
        The special function keys of the Yoga 9 are reported using an ACPI WMI
        event, which had no driver.
    - [0002-ACPICA-Make-address-space-handler-install-and-_REG-e.patch](kernel-patches/0002-ACPICA-Make-address-space-handler-install-and-_REG-e.patch)
      & [0003-ACPI-EC-fix-ECDT-probe-ordering-issues.patch](kernel-patches/0003-ACPI-EC-fix-ECDT-probe-ordering-issues.patch)  
        The brightness keys should be supported already but aren't due to a
        difference in ACPI initialization between Windows and Linux.

## Extra Features

Digging into the ACPI DSDT revealed some more features which are available via
Lenovo Vantage on Windows.

- The Laptop has a USB-C quick-charge setting which is controlled by an ACPI
  function. This can be exposed as a sysfs attribute using
  [0006-Add-IdeaPad-quick_charge-attribute-to-sysfs.patch](kernel-patches/0006-Add-IdeaPad-quick_charge-attribute-to-sysfs.patch).
- The Laptop has a WMI device which reports the 'usage mode'. The usage mode
  indicates whether the convertible is used as a Laptop, Tent, Tablet or
  Drawboard. The
  [0005-Add-IdeaPad-Usage-Mode-driver.patch](kernel-patches/0005-Add-IdeaPad-Usage-Mode-driver.patch)
  can emit a `SW_TABLET_MODE` switch event to automatically enable the
  on-screen-keyboard and screen-auto-rotation in some desktop environments
  (e.g. KDE Plasma & GNOME).
- The ACPI can also control the keyboard backlight setting (Off/Low/High/Auto)
  but I have not written a patch for that yet.

## Bug Reports

I found a lot of useful information and patches in these threads:
- [bass speakers](https://bugzilla.kernel.org/show_bug.cgi?id=208555)  
  Many thanks to all the people sharing their knowledge and readily
  testing my patch!
- [brightness keys](https://bugzilla.kernel.org/show_bug.cgi?id=214899)  
  Many thanks to both the reporter with god-like ACPI debugging skills
  and the kernel developer spending his time on fixing the issue!

## Submitted Patches

The bass speaker patch has been submitted to and was applied by the
SOUND subsystem maintainer. The submission process was fast and
uncomplicated!


## License

This repository contains patches, which are either derivative work targeting a
specific already licensed source, i.e. parts of the Linux kernel, or introduce
new parts to the Linux kernel. These patches fall thus, if not explicitly
stated otherwise, under the license of the source they are targeting, or if
they introduce new code, the license they explicitly specify inside of the
patch. Please refer to the specific patch and source in question for further
information. License texts can be obtained at
https://github.com/torvalds/linux/tree/master/LICENSES.