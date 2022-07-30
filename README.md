# Linux on the Lenovo Yoga9 14IAP7

I've recently bought a Lenovo Yoga9 14IAP7 to run Linux on without knowing what
would work.
I looked into the various nitpicks that were bugging me.
After some messing with the kernel I'm really happy with my device.

## Problems OOTB

- The bass speakers don't work (only sound from the tinny tweeters)
- Various function keys don't emit any input event's ('Lenovo Support',
  'Lenovo Favorite', 'Virtual Background', 'Sound Profile', 'Darkmode Toggle'
  and the brightness keys)

## Solutions

- The bass speakers: [patch](kernel-patches/0001-ALSA-hda-realtek-Add-quirk-for-Lenovo-Yoga9-14IAP7.patch)
    1. The ALC287 codex has the bass speakers connected to the Pin Complex on
       NID 0x17 which reports itself as unconnected.
    2. The bass speaker amplifiers need to be enabled using an HDA verb
       sequence which has been extracted by sniffing the Windows drivers
       initialization.
    3. The HDA driver connects the HDA Pin Complex to the DAC on NID 0x06 which
       has no volume control, so the bass speakers would always be on maximum
       volume.

## Extra Features
