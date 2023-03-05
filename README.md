# purple_exporter

[![MIT Licensed][license-badge]][license-url]
[![GitHub Super-Linter][linter-badge]][linter-url]

[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: LICENSE
[linter-badge]: https://github.com/viridianforge/purple_exporter/workflows/Lint%20Code%20Base/badge.svg
[linter-url]: https://github.com/marketplace/actions/super-linter

Exporting metrics from your local PurpleAir Sensor.

## Table of Contents

1. [Description](#description)
2. [Technical Details](#technical-details)
    1. [Exposed Metrics](#exposed-metrics)
        1. [Environmental Data](#environmental-data)
        2. [Estimated Mass Concentrations](#estimated-mass-concentrations)
        3. [Particle Concentrations](#particle-concentrations)
3. [Building Purple Exporter](#building-purple-exporter)
    1. [Getting the IP address](#getting-the-ip-address)
    2. [Building from Source](#building-from-source)
    3. [Building Docker Image](#building-docker-image)
4. [Required Configuration](#required-configuration)
    1. [Sensor Index](#sensor-index)
    2. [Request Rate](#request-rate)
    3. [Network Port](#network-port)
    4. [Environmental Adjustment](#environmental-adjustment)
    5. [Log Level](log-level)
5. [Running Purple Exporter](#running-purple-exporter)
    1. [Running from Source](#running-from-source)
    2. [Running from Executable](#running-from-executable)
    3. [Running from Docker](#running-from-docker)
    4. [Running from Docker-compose](#running-from-docker-compose)
6. [Grafana](#grafana)
7. [Testing](#testing)
8. [License](#license)
9. [References](#references)

## Description

purple_exporter is a Prometheus exporter that exports a subset of available
information from a single
[PAII Purple Air Sensor](https://www2.purpleair.com/products/purpleair-pa-ii).

This project was developed by [ViridianForge](https://github.com/ViridianForge/purple_exporter] could)
to pipe the information from his own PAII sensor to a grafana chart.
[mikeki](https://github.com/mikeki/purple_exporter) forked the project in 2023 to fetch directly from the local device.

## Technical Details

### Exposed Metrics

Metrics are collected from all sensors for which they are available.

#### Environmental Data

- Humidity: Relative humidity in the sensor chamber, on average, 4% lower than ambient humidity
- Temperature: Temperature in the sensor chamber in Farenheit, on average 8F higher than ambient temperature
- Air Pressure: Current pressure in millibars
- AQI: Calculated Air Quality Index provided by the PAII sensor.

#### Estimated Mass Concentrations

- pm1.0: Estimated mass concentration of particles less than 1 uM in diameter. (ug/m^3)
- pm2.5: Estimated mass concentration of particles less than 2.5 uM in diameter. (ug/m^3)
- pm10.0: Estimated mass concentration of particles less than 10.0 uM in diameter. (ug/m^3)

#### Partical Concentrations

- 0.3um: Count of particles greater than 0.3 uM in diameter. (particles/100mL)
- 0.5um: Count of particles greater than 0.5 uM in diameter. (particles/100mL)
- 1.0um: Count of particles greater than 1.0 uM in diameter. (particles/100mL)
- 2.5um: Count of particles greater than 2.5 uM in diameter. (particles/100mL)
- 5.0um: Count of particles greater than 5.0 uM in diameter. (particles/100mL)
- 10.0um: Count of particles greater than 10.0 uM in diameter. (particles/100mL)

## Building Purple Exporter

### Getting the IP address

In order to use the exporter, one will need to have network access from the machine running the exporter
to the PAII sensor, and know the IP address of the sensor.

### Building From Source

`cargo build --release`

The resulting executable will be placed in `<path_to_repo>/target/release/`.

### Building Docker Image

If running from the repository root:
`docker build -t <tag> .`

This will result in a trimmed down alpine linux image with a statically compiled executable.

## Required Configuration

The exporter is configurable using the values below, specified via environmental
variable, or command line argument.

### Sensor Index

Set with the `SENSOR_IP` environment variable, or the `-s` flag.
This value is the IP address assigned by your Router to the PurpleAir sensor that readings
will be drawn from.

### Request Rate

Set with the `REQUEST_RATE` environment variable, or the `-r` flag.
This is how often to query the purple air API, in seconds.

### Network Port

Set with the `PORT` environment variable, or the `-p` flag.
This is the network port the exporter will serve its data from.

### Environmental Adjustment

Set with the `ADJUST` environment variable, or the `-a` flag.
This flag is used to indicate whether humidity and temperature readings
should be adjusted to reflect likely ambient readings rather than readings in
the sensor housing. Sensor readings are, on average, 8F hotter and 4% drier.
Valid values are `true` or `false`, and will default to `false`.

### Log Level

Set with `RUST_LOG` to indicate which level of messages to log when running
purple_exporter. Options include: `info` or `trace`.

## Running the Exporter

### Running from Source

`cargo run -- -r rate -s sensor_index -x API_read_key -p port -a adjust`

### Running from Executable

`./purple_exporter -r rate -s sensor_index -x API_read_key -p port -a adjust`

or by setting environment variables.

### Running from Docker

`docker run --env-file .env <image>`

An example environment variable file is provided in `.env.example`.

### Running from Docker-compose

```
version: "3.9":
services:
  purple-exporter:
    image: miguelcervera/purple_exporter:latest
    restart: always
    ports:
      - "9420:9420"
    environment:
      - REQUEST_RATE=15
      - PORT=9420
      - SENSOR_IP=<IP_ADDRESS_OF_YOUR_PAII_SENSOR>
      - ADJUST=true
```

## Grafana

A Grafana dashboard for use with this exporter is available from the [Grafana
community library](https://grafana.com/grafana/dashboards/14723).

For direct importing into a Grafana instance, please use the packaged
[dashboard.json](./dashboards/dashboard.json).

## Testing

Work in Progress

## License

[LICENSE](./LICENSE)

## References

- [PurpleAir API docs](https://api.purpleair.com/#api-welcome)
- Purple_Exporter makes use of seanmonstar's [pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger) to
provide activity logging.
