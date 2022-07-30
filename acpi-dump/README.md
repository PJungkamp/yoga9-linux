
# Information gathered from the ACPI tables (mainly the DSDT)

- dumping the DSDT: https://wiki.archlinux.org/title/DSDT
- parsing the WMI data in `_WDG` methods: https://github.com/iksaif/wmidump
- decode the WMI MOF data: https://github.com/pali/bmfdec.git

## ACPI functions

`\_SB.PC00.LPCB.EC0.VPC0.KBLC` - keyboard backlight control

-   Use the `acpicall` script with the `acpicall` kernel module to set the backlight:
    ```sh
    ./acpicall \\_SB.PC00.LPCB.EC0.VPC0.KBLC 0x?0033
    ```
    Substitute the '?' with 0,1,2,3 to control the keyboard backlight.
    | argument | meaning  |
    | -------: | :------- |
    |  0x00033 | set off  |
    |  0x10033 | set low  |
    |  0x20033 | set high |
    |  0x30033 | set auto |
    |  0x00032 | query    |

 -  Query the backlight level with:
    ```sh
    ./acpicall \\_SB.PC00.LPCB.EC0.VPC0.KBLC 0x00032
    ```
    The lower bits indicate the current setting.
    | query result | meaning |
    | -----------: | :------ |
    |      0x10007 | auto    |
    |      0x10005 | high    |
    |      0x10003 | low     |
    |      0x10001 | off     |

## WMI devices in the ACPI table

`WMI4 (_UID 0x04)` - Battery information
- Expensive String 0xAD (WQAD)
    - Query battery information string
- Object 0xBD (WQBD)

`WMIY (_UID "YMC")` - Yoga Mode change event -> emit tablet tablet mode switch
- Method 0xAB (WMAB)
    - Calls EC0.YGAM
- Event 0xD (_WED)
    - Always returns 1
- Object 0xBD (WQBD)

`WMIU (_UID "LSK20")` - Fn/Special-Key reporting
- Method 0xSK (WMSK)
    - Returns different numbers based on Arg2 if Arg1 == One
- Event 0xD (_WED)
    - checks FnLock (HKDB) for return value
    - LSKD seems to indicate the key which was pressed
- Object 0xDA (WQDA)

`WMIS (_UID "LSR")` - Lenovo SUPER Resolution ????
- Method 0xSR (WMSR)
    - Can toggle EC0.LESR between 0 and 1
    - Queries GFX0.VIDG and GFX0.DIDG
- Event 0xD (_WED)
    - Always returns 1
- Object 0xDF (WQDF)

`LFCD (_UID "LCFC")` - Fan Speed & Power Limit Control + Temperature readings
- Method 0xA0 (WMA0)
    - Temperature Query and Fan Control
- Object 0xDE (WQDE)

`WFDE (_UID "DSarDev")` - Monitors WiFi SAR... does this do anything iwlwifi can't?
- Method 0xDE (WMDE)
    - Checks if DSSI and sets PDAT and DSSI
- Event 0xD (_WED)
    - Checks PDAT
- Object 0xCC (WQCC)

`WFTE (_UID "TestDev")` - Test events for WiFi SAR monitor
- Method 0xTE (WMTE)
    - Sets PDAT and notifies the WFDE WMI device
- Object 0xCC (WQCC)
