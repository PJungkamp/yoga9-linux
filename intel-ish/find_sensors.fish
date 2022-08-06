
set -a properties property-friendly-name:'Friendly Name'
set -a properties property-sensor-model:'Sensor Model'
set -a properties property-sensor-manufacturer:'Manufacturer'
set -a properties property-sensor-serial-number:'Serial Number'

function get_value -a feature
    for letter in (string split -n ' ' < $feature/*-value)
        printf \\(printf %o $letter)
    end
end

function check_property -a property feature
    if string match -q (string split -f1 ':' $property) -- (cat $feature/*-name)
        echo \t(string split -f2 ':' $property):\t (get_value $feature)
    end
end

for sensor in /sys/bus/ishtp/devices/\{33AECD58-B679-4E54-9BD9-A04D34F0C226\}/001F:8087:0AC2.000?/HID-SENSOR-*
    echo Sensor: $sensor
    for feature in $sensor/feature-*
        for property in $properties
            check_property $property $feature
        end
    end
    echo
end
