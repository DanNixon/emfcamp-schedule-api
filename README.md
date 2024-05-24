# EMFcamp schedule API tools

Various things related to ingesting, manipulating and otherwise using the [EMFcamp schedule API](https://developer.emfcamp.org/schedule).

Includes:

- [A Rust client library](./client)
- [A CLI](./cli)
- [An adapter that sits between the official API, adding some nice development features and fixing the fucky timestamps](./adapter)
- [A tool to send event announcements via MQTT](./mqtt-announcer)
