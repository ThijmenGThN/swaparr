# Swaparr

Radarr and Sonarr currently lack a built-in mechanism to handle stalled torrents, this project aims to solve that.

> Swaparr is inspired by a Reddit thread ["I wrote a script that replaces slow/dead torrents automatically"](https://www.reddit.com/r/radarr/comments/101q31k/i_wrote_a_script_that_replaces_slowdead_torrents/) from [Douglas96](https://www.reddit.com/user/Douglas96/).

<p align="center">
  <img src="https://i.imgur.com/7D84ooQ.png?s=128">
</p>


## Disclaimer

As Swaparr is not yet at its 1.0.0 stage, expect significant changes and occasional unpredictability until then.

#### Supported Platforms

- [X] Radarr
- [X] Sonarr
- [ ] Lidarr
- [ ] Readarr
- [ ] Arm64 binaries for Raspberry Pi

## What is Swaparr?

Swaparr quietly operates in the background, offering full customization options and clear visibility through console logs. Its primary function is to address the issue of stalled torrents in Radarr and Sonarr instances.

### Key Features:

- **Automatic Detection:** Swaparr scans through all active torrents in your Radarr or Sonarr instances every 10 minutes (adjustable) to identify potential slowdowns indicating stalled torrents.
- **Strike System:** Identified torrents are given a strike, and this evaluation cycle repeats periodically. If a torrent accumulates the maximum allowed strikes, Swaparr automatically removes it from your instance.
- **Customization:** Swaparr offers customization options such as time and size thresholds, strike thresholds, and the ability to toggle aggressive strike behavior.

## Getting Started

In this section, we'll deploy Swaparr using Docker and its compose plugin.

> Docker is not mandatory; you can also [run the binaries](#prerequisites) or compile Swaparr yourself. However, note that more advanced steps are required for these methods.

Start with the provided compose file as a foundation, and customize it by excluding or adjusting any services.

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


<details>
  <summary>
    <strong>Starting Swaparr</strong>
  </summary>

To start Swaparr, run the following command:

```sh
docker compose up -d
```

</details>


<details>
  <summary>
    <strong>Observing Swaparr</strong>
  </summary>

The following example will output logs of Swaparr's Radarr service.

```sh
docker compose logs radarr
```

_[It should output something similar to this.](#swaparr)_

</details>


<details>
  <summary>
    <strong>Stopping Swaparr</strong>
  </summary>

To stop Swaparr, run the following command:

```sh
docker compose down
```

</details>


## Useful Information

<details>
  <summary>
    <strong>Environment Variables</strong>
  </summary>

| Name              | Default              | Description                                                                                     |
|-------------------|----------------------|-------------------------------------------------------------------------------------------------|
| BASEURL           | `http://127.0.0.1:7878` | The URL of either a Sonarr or Radarr instance.                                                |
| APIKEY            | `7f3a8..cbc07`         | The API key required for accessing the Radarr or Sonarr instance.                             |
| PLATFORM          | `radarr`              | Indicates the platform with which Swaparr interacts, either `radarr` or `sonarr`.              |
| TIME_THRESHOLD    | `2h`                  | The duration threshold for torrents to be considered stalled; torrents exceeding this limit will be removed. |
| SIZE_THRESHOLD    | `25GB`                | The size limit for torrents to be ignored; torrents exceeding this limit will not be processed. |
| CHECK_INTERVAL    | `10m`                 | The interval at which Swaparr monitors torrents.                                               |
| STRIKE_THRESHOLD  | `3`                   | The number of strikes a torrent needs to reach before it is subject to removal.                |
| AGGRESSIVE_STRIKES| `false`               | Enables the removal of stalled torrents and those stuck fetching metadata.                      |

</details>



<details>
  <summary>
    <strong>Status-Types Explained</strong>
  </summary>

| Type | Meaning |
| --- | --- |
| `Normal`  | Not stalled or slow, will not be striked. |
| `Pending` | Fetching metadata or stalled (can be bypassed with `aggressive_strikes`). |
| `Striked` | Flagged as slow or stalled, pending removal. |
| `Removed` | Removed from Radarr / Sonarr. |
| `Ignored` | Outside of threshold bounds. |

</details>



<details>
  <summary>
    <strong>Getting Started</strong> (without Docker)
  </summary>

#### Prerequisites

To begin, [download the executable](https://github.com/ThijmenGThN/swaparr/releases) compatible with your operating system.

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

#### Run Swaparr

You should now be able to run Swaparr directly from the binary file.

</details>



## Need Help?

If you need assistance or have suggestions for improvement, please don't hesitate to [open an issue](https://github.com/ThijmenGThN/swaparr/issues). Your feedback is valuable.



## Feature Requests, Bug Reports

Your feedback, bug reports and suggestions are very welcome, you can do so by [opening an issue](https://github.com/ThijmenGThN/swaparr/issues).



## Contributions

Feel free to [open an issue](https://github.com/ThijmenGThN/swaparr/issues) or PR if you want to contribute to this project.
