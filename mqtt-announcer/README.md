# MQTT announcer

A tool to announce upcoming events via MQTT at a specified interval before the event is due to begin.

This tool publishes on the following topics:

- `PREFIX/online`: either `true` or `false` depending on if the tool is running or exited
- `PREFIX/full`: the full event description
- `PREFIX/smol`: a minimal event description (intended for restricted resource environments, e.g. microcontrollers)
