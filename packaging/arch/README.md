
# Arch Linux Kernel

I run Arch myself, so I might as well share the [`PKGBUILD`](linux-yoga/PKGBUILD)
I use for a patched kernel.

## Kernel `config`

I have reduced my `config`. The included kernel [`config`](linux-yoga/config)
does only include the kernel modules I use myself.

If you already run a reduced `config` you can use that one and add
```
CONFIG_IDEAPAD_WMI_FN_KEYS=m
CONFIG_IDEAPAD_WMI_USAGE_MODE=m
```
to enable the two WMI drivers in the patches.

If you want to try this package on another device you may need to use the default arch `config`.

## Building `linux-yoga`

`cd` into the `linux-yoga` directory and run something like:
```bash
MAKEFLAGS="-j$(nproc)" makepkg -si
```
The default config where pretty much all drivers are enabled takes hours to
compile on a single thread.
The `MAKEFLAGS` enable parallel compilation to significantly improve the
compile time.

You should also consider trimming down your config to further improve
compilation speed.

## Relevant Arch Wiki Pages

[Kernel/Arch Build System](https://wiki.archlinux.org/title/Kernel/Arch_Build_System)
[Makepkg#Parallel_compilation](https://wiki.archlinux.org/title/Makepkg#Parallel_compilation)
[Modprobed-db](https://wiki.archlinux.org/title/Modprobed-db)
