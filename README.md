# Swaparr

Radarr and Sonarr currently lack a built-in mechanism to handle stalled torrents, this project aims to solve that.

> Swaparr is inspired by a Reddit thread ["I wrote a script that replaces slow/dead torrents automatically"](https://www.reddit.com/r/radarr/comments/101q31k/i_wrote_a_script_that_replaces_slowdead_torrents/) from [Douglas96](https://www.reddit.com/user/Douglas96/).

<p align="center">
  <img src="https://i.imgur.com/7D84ooQ.png?s=128">
</p>


## Disclaimer

As Swaparr is not yet at its 1.0.0 stage, expect significant changes and occasional unpredictability until then.


## Getting Started

In this section, we'll deploy Swaparr using Docker and its compose plugin.

> Docker is not mandatory; you can also [run the binaries](#getting-started-without-docker) or compile Swaparr yourself. However, note that more advanced steps are required for these methods.

Start with the provided compose file as a foundation, and customize it by excluding or adjusting the services.

```yml
version: '3'
services:

  radarr:
    image: ghcr.io/thijmengthn/swaparr:latest
    container_name: swaparr-radarr
    restart: unless-stopped
    environment:
      - BASEURL=http://127.0.0.1:7878 # IP or FQDN           (Required)
      - APIKEY=7f3a8..cbc07           # Radarr API Key       (Required)                
      - PLATFORM=radarr               # "radarr" or "sonarr" (Optional) default: radarr
      - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
      - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
      - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
      - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
      - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 

  # -- (Optional)
  sonarr: 
    image: ghcr.io/thijmengthn/swaparr:latest
    container_name: swaparr-sonarr
    restart: unless-stopped
    environment:
      - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
      - APIKEY=7f3a8..cbc07           # Sonarr API Key       (Required)                
      - PLATFORM=sonarr               # "radarr" or "sonarr" (Optional) default: radarr
      - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
      - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
      - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
      - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
      - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 
```

### Starting Swaparr

```sh
docker compose up -d
```

#### Observing Swaparr

#### Example for Radarr - (_[It should output something similar to this.](#swaparr)_)

```sh
docker compose logs radarr
```

### Stopping Swaparr

```sh
docker compose down
```

## Environment Variables

| Name | Default | Description |
|-|-|-|
| BASEURL | `http://127.0.0.1:7878` | URL of a Sonarr or Radarr instance. |
| APIKEY | `7f3a8..cbc07` | API key for accessing the Radarr or Sonarr instance. |
| PLATFORM | `radarr` | Indicates the platform Swaparr interacts with, either `radarr` or `sonarr`. |
| TIME_THRESHOLD | `2h` | Duration threshold for torrents to be considered stalled. |
| SIZE_THRESHOLD | `25GB` | Size limit for torrents to be ignored. |
| CHECK_INTERVAL | `10m` | Interval for monitoring torrents. |
| STRIKE_THRESHOLD | `3` | Number of strikes before a torrent is subject to removal. |
| AGGRESSIVE_STRIKES | `false` | Enables removal of stalled torrents and those stuck fetching metadata. |


## Status Types

| Type | Meaning |
| --- | --- |
| `Normal`  | Not stalled or slow, will not be striked. |
| `Pending` | Fetching metadata or stalled (can be bypassed with `aggressive_strikes`). |
| `Striked` | Flagged as slow or stalled, pending removal. |
| `Removed` | Removed from Radarr / Sonarr. |
| `Ignored` | Outside of threshold bounds. |


## Getting Started (without Docker)

To begin, download the executable compatible with your operating system.

Before running Swaparr, manually set the required [environment variables](#environment-variables).

> Note: You do not need to define every environment variable; only the ones you need and those that are required.

#### Powershell

```
$Env:<variable>="<value>"
```

#### Shell

```
export <variable>="<value>"
```

You should now be able to run Swaparr.


## Need Help?

If you need assistance or have suggestions for improvement, please don't hesitate to [open an issue](https://github.com/ThijmenGThN/swaparr/issues). Your feedback is valuable.


## Contributions

Feel free to [open an issue](https://github.com/ThijmenGThN/swaparr/issues) or PR if you want to contribute to this project.
