# API adapter

The adapter is a small shim that sits between applications and the [official schedule API](https://developer.emfcamp.org/schedule).

It adds a few nice features on top of the official API:

- RFC 3339 timestamps (not some RFC 3339-like but actually not nonsense)
- Filtering by multiple venues
- Filtering by timestamps
- A now and next API that is not dependant on being part way through the event to develop for
- Listing venues

The format of the data returned by the adapter is effectively identical to what the official EMF API is (with the expection of correctly formatted timestamps).

Note that if you make use of `fake_epoch` that this only changes the start and end timestamps (`start_date` and `end_date`) it does not affect the times (`start_time` and `end_time`, why these fields are there when the former fields are actually a timestamp I have no bloody idea).

## Examples

It is likely useful to stuff the output of these into your JSON visualiser of choice (e.g. pipe to `| jq | less`).

- List the entire schedule: `curl "localhost:8000/schedule"`
- List events for a specific venue: `curl "localhost:8000/schedule?venue=Stage+A"`
- List events for a set of venues: `curl "localhost:8000/schedule?venue=Stage+A&venue=Stage+B"`
- List events starting after a certain time (i.e. events that are in progress and in the future as of the given time): `curl "localhost:8000/schedule?starting_after=2022-06-05T12:00:00%2b01:00"`
- List events ending after a certain time (i.e. events that are in the future/yet to start as of the given time): `curl "localhost:8000/schedule?ending_after=2022-06-05T12:00:00%2b01:00"`
- List the entire schedule, using a fake start time for the first event and offsetting the rest of the schedule accordingly (useful for development): `curl "localhost:8000/schedule?fake_epoch=2024-04-01T17:00:00%2b01:00"`
- Now and next, for all venues, at the time of the request: `curl "localhost:8000/now-and-next"`
- Now and next, for "Stage A" and "Blacksmiths" venues, for a specific point in time, with a fake epoch: `curl "localrost:8000/now-and-next?fake_epoch=2024-04-01T17:00:00%2b01:00&now=2024-04-02T17:15:00%2b01:00&venue=Stage+A&venue=Blacksmiths"`
- List all venues: `curl "localhost:8000/venues"`
