# DAB <==> RDK Adapter #

This software is a RUST application that enables compatibility with [DAB 2.0 specification](https://getdab.org/) to devices based on [Reference Design Kit (RDK)](https://rdkcentral.com/).
The DAB <-> RDK adapter can be executed both on the RDK device or using an external PC.

## Building ##

Since this software uses Cargo package manager, the building process is straightforward:

```
$ cargo build
```

The output binary will be located at `./target/debug/dab-adapter`.

## Usage ##

```
dab-adapter --help

USAGE:
    dab-adapter [OPTIONS]

OPTIONS:
    -b, --broker <MQTT_HOST>    The MQTT broker host name or IP (default: localhost)
    -d, --device <DEVICE>       The device host name or IP (default: localhost)
    -h, --help                  Print help information
    -p, --port <MQTT_PORT>      The MQTT broker port (default: 1883)
    -v, --version               Print the version information
```

## Device ID ##

In this implementation for RDK, the Device ID as specified by DAB is given by the `org.rdk.System.getDeviceInfo`` method of [RDK plugin](https://rdkcentral.github.io/rdkservices/#/api/SystemPlugin).

## Implementations ##

This adapter supports the three full protocol implementation types:

### Option 1: "On Device" Implementation ###

![Option 1: "On Device" Implementation](doc/Option1.png)

```
$ dab-adapter
```

### Option 2: Remote Broker Implementation ###

![Option 2: Remote Broker Implementation](doc/Option2.png)

Let's suppose `192.168.0.100` as the MQTT Broker IP address:

```
$ dab-adapter -b 192.168.0.100
```

### Option 3: "Bridge" Implementation ###

![Option 3: "Bridge" Implementation](doc/Option3.png)

Let's suppose `192.168.0.200` as the RDK Device (Device Under Test) IP address:

```
$ dab-adapter -d 192.168.0.200
```

## DAB Operations Currently Supported ##

This version currently supports the following DAB operations:

### Applications ###

| Request Topic                    | Supported |
|----------------------------------|-----------|
| applications/list                |    Yes    |
| applications/launch              |    Yes    |
| applications/launch-with-content |    Yes    |
| applications/get-state           |    Yes    |
| applications/exit                |    Yes    |
| device/info                      |    Yes    |
| system/restart                   |    Yes    |
| system/settings/list             |     -     |
| system/settings/get              |     -     |
| system/settings/set              |     -     |
| input/key/list                   |    Yes    |
| input/key-press                  |    Yes    |
| input/long-key-press             |    Yes    |
| output/image                     |    Yes    |
| device-telemetry/start           |     -     |
| device-telemetry/stop            |     -     |
| app-telemetry/start              |     -     |
| app-telemetry/stop               |     -     |
| health-check/get                 |    Yes    |
| voice/list                       |    Yes    |
| voice/send-audio                 |    Yes    |
| voice/send-text                  |    Yes    |
