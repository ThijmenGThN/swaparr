# Swaparr

Radarr, Sonarr and the other Starrs currently lack a built-in mechanism to handle stalled downloads, this project aims to solve that.

> Swaparr is inspired by a Reddit thread ["I wrote a script that repl.."](https://www.reddit.com/r/radarr/comments/101q31k/i_wrote_a_script_that_replaces_slowdead_torrents/) from [Douglas96](https://www.reddit.com/user/Douglas96/).

<p align="center">
  <img src="https://i.imgur.com/7D84ooQ.png?s=128">
</p>


#### ⭐ Show Your Support for Open Source!

If Swaparr has been helpful to you and you appreciate the power of open-source software, please consider giving this repository a star. Your gesture will greatly support our efforts and help others discover Swaparr!

[![Stargazers](https://reporoster.com/stars/dark/notext/ThijmenGThN/swaparr)](https://github.com/ThijmenGThN/swaparr/stargazers)

## What is Swaparr?

Swaparr quietly operates in the background, offering full customization options and clear visibility through console logs. Its primary function is to address the issue of stalled downloads in starr instances.

### Key Features:

- **Automatic Detection:** Swaparr scans through all active downloads in your starr instances every 10 minutes (adjustable) to identify potential slowdowns.
- **Strike System:** Identified downloads are given a strike, and this evaluation cycle repeats periodically. If a download accumulates the maximum allowed strikes, Swaparr automatically removes it from your instance.
- **Customization:** Swaparr offers customization options such as time and size thresholds, strike thresholds, and the ability to toggle aggressive strike behavior.

## Getting Started

> [!WARNING]
> Swaparr is still in beta, things might change before reaching version ` 1.0.0 `

### Prerequisites 

Docker and it's compose plugin are required, below is a matrix on how to install both for your system:

|Operating System|Official Instructions|
|-|-|
|Linux|[docs.docker.com/../linux-install](https://docs.docker.com/desktop/install/linux-install/)
|MacOS|[docs.docker.com/.../mac-install](https://docs.docker.com/desktop/install/mac-install/)
|Windows|[docs.docker.com/.../windows-install](https://docs.docker.com/desktop/install/windows-install/)

### Configurations

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
      - PLATFORM=radarr               # "radarr", "sonarr".. (Optional) default: radarr
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
      - PLATFORM=sonarr               # "radarr", "sonarr".. (Optional) default: radarr
      - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
      - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
      - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
      - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
      - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 
```

### Starting Swaparr

To start Swaparr, run the following command:

```
docker compose up -d
```

### Monitor

  You can monitor Swaparr's activities and track the processing of downloads by executing the following command. Omit the ` <container_name> ` parameter to view logs for all platforms:

  ```
  docker compose logs <container_name>
  ```

<details>
  <summary>
    <strong>Instructions:</strong> Stop or Update Swaparr
  </summary>

  #### Stop

  To shutdown Swaparr, run the following command:

  ```
  docker compose down
  ```

  
  #### Update

  Updating Swaparr is a breeze, pull the latest images and restart the service:

  ```
  docker compose pull
  ```

  ```
  docker compose down
  ```

  ```
  docker compose up -d
  ```

</details>

<details>
  <summary>
    <strong>Extended support:</strong> Lidarr, Readarr and Whisparr (Experimental) 
  </summary>

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
        - PLATFORM=radarr               # "radarr", "sonarr".. (Optional) default: radarr
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
        - PLATFORM=sonarr               # "radarr", "sonarr".. (Optional) default: radarr
        - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
        - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
        - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 

    # -- (Optional)
    lidarr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-lidarr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Lidarr API Key       (Required)                
        - PLATFORM=lidarr               # "radarr", "sonarr".. (Optional) default: radarr
        - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
        - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
        - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 

    # -- (Optional)
    readarr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-readarr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Readarr API Key      (Required)                
        - PLATFORM=readarr              # "radarr", "sonarr".. (Optional) default: radarr
        - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
        - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
        - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 

    # -- (Optional)
    whisparr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-whisparr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Whisparr API Key     (Required)                
        - PLATFORM=whisparr             # "radarr", "sonarr".. (Optional) default: radarr
        - TIME_THRESHOLD=2h             # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - SIZE_THRESHOLD=25GB           # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
        - CHECK_INTERVAL=10m            # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - STRIKE_THRESHOLD=3            # Positive number      (Optional) default: 3     
        - AGGRESSIVE_STRIKES=false      # Boolean              (Optional) default: false 
  ```
</details>

## Useful Information

A brief rundown to shed light on a couple of things for you:

<details>
  <summary>
    <strong>Environment Variables</strong>
  </summary>

  | Name              | Default              |  Description                                                                                     |
  |-------------------|----------------------|  -------------------------------------------------------------------------------------------------|
  | BASEURL           | `http://127.0.0.1:7878` | The URL of a starr instance.                                                |
  | APIKEY            | `7f3a8..cbc07`         | The API key required for accessing the starr instance.                             |
  | PLATFORM          | `radarr`              | Indicates the platform with which Swaparr interacts,  either `radarr`, `sonarr`, `lidarr`, `readarr` or `whisparr`.|
  | TIME_THRESHOLD    | `2h`                  | The duration threshold for downloads to be considered  stalled; downloads exceeding this limit will be removed. |
  | SIZE_THRESHOLD    | `25GB`                | The size limit for downloads to be ignored; downloads   exceeding this limit will not be processed. |
  | CHECK_INTERVAL    | `10m`                 | The interval at which Swaparr monitors  downloads.                                               |
  | STRIKE_THRESHOLD  | `3`                   | The number of strikes a download needs to reach  before it is subject to removal.                |
  | AGGRESSIVE_STRIKES| `false`               | Enables the removal of stalled downloads and those   stuck fetching metadata.                      |
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
  | `Removed` | Removed from its starr instance. |
  | `Ignored` | Outside of threshold bounds. |
</details>

## ✨ Swaparr is for everyone!

Whether you need help, want to pitch in, or found a bug that needs fixing, just [open an issue](https://github.com/ThijmenGThN/swaparr/issues). We're all ears and ready to collaborate with you!
