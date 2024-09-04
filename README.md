
# Swaparr

Radarr, Sonarr and the other Starrs currently lack a built-in mechanism to handle stalled downloads, this project aims to solve that.

> Swaparr is inspired by a Reddit thread ["I wrote a script that repl.."](https://www.reddit.com/r/radarr/comments/101q31k/i_wrote_a_script_that_replaces_slowdead_torrents/) from [Douglas96](https://www.reddit.com/user/Douglas96/).

<p align="center">
  <img src="https://i.imgur.com/HMCDGbO.png?s=128">
</p>



#### ⭐ Show Your Support for Open Source!

If Swaparr has been helpful to you and you appreciate the power of open-source software, please consider giving this repository a star. Your gesture will greatly support our efforts and help others discover Swaparr!

[![Stargazers](https://reporoster.com/stars/dark/notext/ThijmenGThN/swaparr)](https://github.com/ThijmenGThN/swaparr/stargazers)



## What is Swaparr?

Swaparr quietly operates in the background, offering full customization options and clear visibility through console logs. Its primary function is to address the issue of stalled downloads in starr instances.



### Key Features:

- **Automatic Detection:** Swaparr scans through all active downloads in your starr instances every 10 minutes (adjustable) to identify potential slowdowns.
- **Strike System:** Identified downloads are given a strike, and this evaluation cycle repeats periodically. If a download accumulates the maximum allowed strikes, Swaparr automatically removes it from your instance.
- **Customization:** Swaparr offers customization options to fine-tune striking behaviour.



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
      - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
      - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
      - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
      - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  

  # -- (Optional)
  sonarr: 
    image: ghcr.io/thijmengthn/swaparr:latest
    container_name: swaparr-sonarr
    restart: unless-stopped
    environment:
      - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
      - APIKEY=7f3a8..cbc07           # Sonarr API Key       (Required)                
      - PLATFORM=sonarr               # "radarr", "sonarr".. (Optional) default: radarr
      - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
      - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
      - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
      - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
```

<details>
  <summary>
    <strong>🚩 Extended experimental support:</strong> Lidarr, Readarr and Whisparr
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
        - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
        - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  

    # -- (Optional)
    sonarr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-sonarr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Sonarr API Key       (Required)                
        - PLATFORM=sonarr               # "radarr", "sonarr".. (Optional) default: radarr
        - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
        - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  

    # -- (Optional)
    lidarr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-lidarr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Lidarr API Key       (Required)                
        - PLATFORM=lidarr               # "radarr", "sonarr".. (Optional) default: radarr
        - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
        - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  

    # -- (Optional)
    readarr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-readarr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Readarr API Key      (Required)                
        - PLATFORM=readarr              # "radarr", "sonarr".. (Optional) default: radarr
        - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
        - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  

    # -- (Optional)
    whisparr: 
      image: ghcr.io/thijmengthn/swaparr:latest
      container_name: swaparr-whisparr
      restart: unless-stopped
      environment:
        - BASEURL=http://127.0.0.1:8989 # IP or FQDN           (Required)
        - APIKEY=7f3a8..cbc07           # Whisparr API Key     (Required)                
        - PLATFORM=whisparr             # "radarr", "sonarr".. (Optional) default: radarr
        - MAX_STRIKES=3                 # Positive number      (Optional) default: 3     
        - SCAN_INTERVAL=10m             # 1d, 6h, 30m, etc..   (Optional) default: 10m   
        - MAX_DOWNLOAD_TIME=2h          # 1d, 6h, 30m, etc..   (Optional) default: 2h    
        - IGNORE_ABOVE_SIZE=25GB        # 1TB, 1GB, 1MB, etc.. (Optional) default: 25GB  
  ```
</details>



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



## Useful Information

A brief rundown to shed light on a couple of things for you:

<details>
  <summary>
    <strong>Environment Variables</strong>
  </summary>

  | Name              | Default                 | Description                                                                                         |
  |-------------------|-------------------------|-----------------------------------------------------------------------------------------------------|
  | BASEURL           | `http://127.0.0.1:7878` | The URL of a radarr, sonarr or other starr instance.                                                |
  | APIKEY            | `7f3a8..cbc07`          | The API key of a radarr, sonarr or other starr instance.                                            |
  | PLATFORM          | `radarr`                | Indicates the type of starr platform, either `radarr`, `sonarr`, `lidarr`, `readarr` or `whisparr`. |
  | MAX_DOWNLOAD_TIME | `2h`                    | Maximum allowed download time before it's considered stalled.                                       |
  | IGNORE_ABOVE_SIZE | `25GB`                  | Files larger than this size will be ignored and not monitored.                                      |
  | SCAN_INTERVAL     | `10m`                   | How often Swaparr checks for stalled downloads.                                                     |
  | MAX_STRIKES       | `3`                     | Maximum number of strikes a download can accumulate before it is removed.                           |
</details>

<details>
  <summary>
    <strong>Status-Types Explained</strong>
  </summary>

  | **Status** | **Description**                                                                 |
  |------------|---------------------------------------------------------------------------------|
  | `Normal`   | Download is proceeding as expected; no issues detected.                         |
  | `Striked`  | Download flagged as slow or stalled; may be removed if it continues to accumulate   strikes. |
  | `Removed`  | Download has been attempted to be removed from the starr instance.       |
  | `Ignored`  | Download is not monitored because it falls outside the set thresholds (e.g., size or time limits). |
</details>



## ✨ Swaparr is for everyone!

Whether you need help, want to pitch in, or found a bug that needs fixing, just [open an issue](https://github.com/ThijmenGThN/swaparr/issues). We're all ears and ready to collaborate with you!
