# BunnyCDN lib 🐰

Thin Rust library around BunnyCDN's web API's 🐇

* [ ] [BunnyCDN](https://docs.bunny.net/reference/bunnynet-api-overview)
* [x] [BunnyCDN Storage API](https://docs.bunny.net/reference/storagezonepublic_index)
* [ ] [Logfiles](https://support.bunnycdn.com/hc/en-us/articles/360018952591-How-do-I-download-my-logs-via-the-API-)

Note: storage API is the only one implemented so far, others coming soon!

## Terminal CLI

[bunnycli](https://github.com/publicarray/bunnycli)

## Requirements

 1. Have a BunnyCDN account
 2. An API Key. You can find this in our Dashboard in the My Account section.
 3. A Storage API key, You can find this in our Storage Zone, the FTP password is the API Key.

## Usage in Cargo.toml

```toml
[dependencies]
bunnycdn = "0.1"
```

Official C lib is here: https://github.com/cp6/BunnyCDN-API
