# VIPER
Dynamic `dumphfdl` wrapper that changes listening frequencies based off activity.

### Modes
#### `single`
Only stay within a single change and never change. This is the same as running `dumphfdl` normally. The only advantage this offers is the automatic grouping of frequencies within a 256-384 KHz "bands".
```
--chooser single:band=13
```

#### `rotate`
Change bands once inactivity timeout invokes.

Valid `type`s:
* `inc` - on timeout, move on to the next highest band; if already on the highest band, move to the lowest in the list
* `dec` - on timeout, move on to the next lowest band; if already on the lowest band, move to the highest in the list
* `random` - on timeout, choose a random band that we haven't visited in the last 6 sessions
```
--chooser rotate:type=random,start=21
```
#### `tracker`
Track messages to/from a specific ground station. Move on to a new band if inactivity timeout occurs or we haven't heard a message to/from target for `timeout` seconds.
```
--chooser tracker:target=Agana,timeout=600
```
### Output
Use the `--output` flag to add an additional output method. For example:
```
--output decoded:json:udp:address=127.0.0.1,port=8000
```