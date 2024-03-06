# Swaparr

Radarr and Sonarr currently lack a built-in mechanism to handle stalled torrents, this project aims to solve that, it is designed in Rust, offering a lightweight and easy to run executable.

<p align="center">
  <img src="https://i.imgur.com/clSL0YN.png?s=128">
</p>

Swaparr is inspired by a Reddit thread ["I wrote a script that replaces slow/dead torrents automatically"](https://www.reddit.com/r/radarr/comments/101q31k/i_wrote_a_script_that_replaces_slowdead_torrents/) written by [Douglas96](https://www.reddit.com/user/Douglas96/).



## Notice

Swaparr is currently in development and may undergo changes.



## Getting Started

To begin, download the executable compatible with your operating system, and utilize the provided examples to execute Swaparr.

### Arguments

> **IMPORTANT:** The quotation marks (`"`) around the args are required for the script to work.

```sh
./swaparr "<baseurl>" "<apikey>" "<platform>" "<time_threshold>" "<size_threshold>" "<check_interval>" "<strike_threshold>" "<aggressive_strikes>"
```


### Examples

> **IMPORTANT:** Adjust `baseurl`, `apikey`, and `platform` to your scenario; other values can remain as is for more advanced use-cases.

#### Radarr
```sh
./swaparr "http://127.0.0.1:7878" "1234567890abcdefghijklmnopqrstuv" "radarr" "2h" "25 GB" "10m" "3" "false"
```

#### Sonarr
```sh
./swaparr "http://127.0.0.1:8989" "1234567890abcdefghijklmnopqrstuv" "sonarr" "2h" "25 GB" "10m" "3" "false"
```



## Status-Types

| Type    | Meaning                                                                                           |
|---------|--------------------------------------------------------------------------------------------------|
| Normal  | The torrent is operating within expected parameters and is not considered to be stalled or slow. |
| Pending | The torrent is still undergoing processing and may not be actively downloading. **Can be bypassed with `aggressive_strikes` set to `true`.** |
| Striked | Torrent has been flagged as slow or stalled and may be subject to removal.                        |
| Removed | Torrent has been removed from Radarr or Sonarr.                                                   |
| Ignored | Torrent is larger than the set threshold and will not be considered for processing.              |



## Arguments

| Name             | Description                                                      | Default         | Expects      | Notes                              |
|:-----------------|:-----------------------------------------------------------------|:----------------|:-------------|:------------------------------------|
| baseurl          | The URL of a Sonarr or Radarr instance.                          | `http://127.0.0.1:7878`  | IP or FQDN   |                                    |
| apikey           | The API key of the Radarr or Sonarr instance.                    | `1234567890abcdefghijklmnopqrstuv` | string of 32 | Can be found at Settings > General > API Key |
| platform         | Defines which platform the script will run for.                  | `radarr`          | `"radarr"` or `"sonarr"` | **Has to be exact!**              |
| time_threshold   | Torrents above this time will eventually be removed.             | `2h`                | 3d, 6h, 30m, etc.. | [Supported human-like time formats](https://docs.rs/ms-converter/latest/ms_converter/#supported-time-strings) |
| size_threshold   | Torrents above this size will be ignored.                        | `25 GB`              | 1024 MB, 1 GiB, 10240 KB | [Supported human-like size formats](https://docs.rs/bytesize/latest/bytesize/#constants) |
| check_interval  | Time to wait before checking if a torrent is susceptible to a strike. | `10m`               | 3d, 6h, 30m, etc.. | [Supported human-like time formats](https://docs.rs/ms-converter/latest/ms_converter/#supported-time-strings) |
| strike_threshold| Number of strikes a torrent needs to be removed.                 | `3`                     | int-range    |                                    |
| aggressive_strikes| Enables removal of torrents stuck fetching metadata and stalled torrents. | `false`            | Boolean      |                                    |



## Need help?

If you need assistance or have suggestions for improvement, please don't hesitate to [open an issue](https://github.com/ThijmenGThN/swaparr/issues). Your feedback is valuable.



## Development

### To be added

- [ ] Request search on API after removal of torrent.
- [ ] Containerize Swaperr.

### Contributions

Feel free to [open an issue](https://github.com/ThijmenGThN/swaparr/issues) or PR if you want to contribute to this project.