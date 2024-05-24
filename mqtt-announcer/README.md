# MQTT announcer

A tool to announce upcoming events via MQTT at a specified interval before the event is due to begin.

This tool publishes on the following topics:

- `PREFIX/online`: either `true` or `false` depending on if the tool is running or exited
- `PREFIX/full`: the full event description
- `PREFIX/smol`: a minimal event description (intended for restricted resource environments, e.g. microcontrollers)

## Formats

### Full

This is similar to the official API and identical to the adapter.

```json
{
  "id": 664,
  "slug": "drop-in-lan-party-eeh",
  "start_date": "2024-05-24T11:10:00+01:00",
  "end_date": "2024-05-24T16:10:00+01:00",
  "venue": "East Essex Hackspace CIO",
  "map_link": null,
  "title": "Drop in LAN Party @ EEH",
  "speaker": "Drop in LAN Party",
  "pronouns": null,
  "description": "Pop in and play.  We have 8 laptop PCs setup to play games all day\r\n\r\nYou can play whatever whenever, however to try and get 8 players who like specific games, pop along at these times to play a game of either C&C/Quake 3/Unreal Tournament.  The laptops are hardwired _and_ we only have 8 lan ports so you're unlikely to be able to bring your own computer.  Sorry.\r\n\r\nRough timings:\r\n10am-11am Command and conquer red alert\r\n11am-12pm Dune 2000\r\n12pm-1pm Quake 3 arena\r\n1pm-2pm Unreal Tournament\r\n2pm-3pm Command and conquer tib dawn\r\n3pm-4pm Quake 3 Arena\r\n\r\nRain may stop play if we can't secure the gazebo.  The area is likely unattended, please give way if you've been on for more than an hour.\r\n\r\nPlease keep swearing to a minimum\r\n\r\nDon't steal or hack the laptops please.",
  "type": "workshop",
  "cost": "",
  "equiptment": null,
  "age_range": "Aimed at adults, but supervised kids welcome.",
  "attendees": "8",
  "may_record": null,
  "is_family_friendly": false,
  "link": "https://www.emfcamp.org/schedule/2024/664-drop-in-lan-party-eeh"
}
```

### Smol

```json
{
  "id": 664,
  "type": "workshop",
  "cost": "",
  "equiptment": null,
  "age_range": "Aimed at adults, but supervised kids welcome.",
  "attendees": "8",
  "start": "2024-05-24T11:10:00+01:00",
  "end": "2024-05-24T16:10:00+01:00",
  "venue": "East Essex Hackspace CIO",
  "title": "Drop in LAN Party @ EEH",
  "speaker": "Drop in LAN Party"
}
```
