# Benchmarks for PWMP Serialization Formats

This repo contains the code used to benchmark different serialization formats that are potencially viable for use in the PixelWeather Messaging Protocol.

## Formats
- [x] bincode
- [x] msgpack
- [x] postcard

## Results

### Summary
|                         | bincode 1.3.3 | rmp-serde 1.3.0 | postcard 1.1.1 |
| ----------------------- | ------------- | --------------- | -------------- |
| Avg. Serialized size    | 2064 bytes    | 2075 bytes      | 2056 bytes     |
| Avg. Serialization time | 2.597µs       | 5.5455µs        | 1.929µs        |
| Deserialization time    | 2.597µs       | 6.7445µs        | 0.915µs        |

Postcard and bincode seem to perform the best. Postcard serializes to a much smaller number of bytes. This isn't really visible in the table due to the large size of the `UpdatePart` message. With almost all message types, Postcard produces 40-75% less bytes on average.

### Details
<details>
  <summary>Detailed log here</summary>
  
  ```
  Serialization:
  --------------------
  Message: Request(Hello { mac: Mac(1, 2, 3, 4, 5, 6) })
  Bincode: 14 bytes, took 2.375µs
  Msgpack: 24 bytes, took 2.375µs
  Postcard: 8 bytes, took 1.188µs
  --------------------
  --------------------
  Message: Request(PostResults { temperature: 20.44, humidity: 67, air_pressure: Some(65535) })
  Bincode: 25 bytes, took 1.188µs
  Msgpack: 33 bytes, took 1.187µs
  Postcard: 13 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Request(PostStats { battery: 4.20, wifi_ssid: "ABCDEFGHIJKLMNOPQRSTUVXYZ0123456", wifi_rssi: -85 })
  Bincode: 61 bytes, took 1.187µs
  Msgpack: 62 bytes, took 1.187µs
  Postcard: 41 bytes, took 1.188µs
  --------------------
  --------------------
  Message: Request(SendNotification("The battery is too low, activating sBOP."))
  Bincode: 56 bytes, took 1.187µs
  Msgpack: 69 bytes, took 1.187µs
  Postcard: 43 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Request(GetSettings)
  Bincode: 8 bytes, took 1.187µs
  Msgpack: 21 bytes, took 1.187µs
  Postcard: 2 bytes, took 1.188µs
  --------------------
  --------------------
  Message: Request(UpdateCheck(Version { major: 1, middle: 0, minor: 1 }))
  Bincode: 11 bytes, took 0ns
  Msgpack: 26 bytes, took 2.375µs
  Postcard: 5 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Request(NextUpdateChunk(1024))
  Bincode: 16 bytes, took 1.187µs
  Msgpack: 29 bytes, took 1.187µs
  Postcard: 4 bytes, took 1.188µs
  --------------------
  --------------------
  Message: Request(ReportFirmwareUpdate(false))
  Bincode: 9 bytes, took 1.188µs
  Msgpack: 32 bytes, took 1.187µs
  Postcard: 3 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Request(Bye)
  Bincode: 8 bytes, took 1.188µs
  Msgpack: 13 bytes, took 1.187µs
  Postcard: 2 bytes, took 1.187µs
  --------------------
  
  SUMMARY:
  --------------------
  Avg Bincode: 23 bytes, time: 1.187µs
  Avg Msgpack: 34 bytes, time: 1.451µs
  Avg Postcard: 13 bytes, time: 1.187µs
  --------------------
  -------------------------
  
  Deserialization:
  --------------------
  Bincode: took 1.188µs
  Msgpack: took 1.187µs
  Postcard: took 1.187µs
  --------------------
  --------------------
  Bincode: took 1.188µs
  Msgpack: took 2.375µs
  Postcard: took 0ns
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 1.187µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 1.187µs
  --------------------
  --------------------
  Bincode: took 1.188µs
  Msgpack: took 1.187µs
  Postcard: took 0ns
  --------------------
  --------------------
  Bincode: took 1.188µs
  Msgpack: took 1.187µs
  Postcard: took 1.188µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.187µs
  Postcard: took 0ns
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 1.187µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 1.187µs
  --------------------
  
  SUMMARY:
  --------------------
  Avg Bincode: time: 1.187µs
  Avg Msgpack: time: 1.319µs
  Avg Postcard: time: 791ns
  --------------------
  -------------------------
  Serialization:
  --------------------
  Message: Response(Pong)
  Bincode: 8 bytes, took 1.187µs
  Msgpack: 15 bytes, took 1.188µs
  Postcard: 2 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Response(Ok)
  Bincode: 8 bytes, took 1.187µs
  Msgpack: 13 bytes, took 1.187µs
  Postcard: 2 bytes, took 0ns
  --------------------
  --------------------
  Message: Response(Reject)
  Bincode: 8 bytes, took 1.188µs
  Msgpack: 17 bytes, took 1.187µs
  Postcard: 2 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Response(FirmwareUpToDate)
  Bincode: 8 bytes, took 1.188µs
  Msgpack: 27 bytes, took 1.187µs
  Postcard: 2 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Response(UpdateAvailable(Versi
  Bincode: 11 bytes, took 1.188µs
  Msgpack: 31 bytes, took 1.187µs
  Postcard: 5 bytes, took 0ns
  --------------------
  --------------------
  Message: Response(UpdatePart([65, 66, 6
  Bincode: 32784 bytes, took 23.746µs
  Msgpack: 32793 bytes, took 68.864µs
  Postcard: 32773 bytes, took 15.435µs
  --------------------
  --------------------
  Message: Response(UpdateEnd)
  Bincode: 8 bytes, took 1.187µs
  Msgpack: 20 bytes, took 1.187µs
  Postcard: 2 bytes, took 1.187µs
  --------------------
  --------------------
  Message: Response(Settings(NodeSettings
  Bincode: 14 bytes, took 1.187µs
  Msgpack: 26 bytes, took 1.188µs
  Postcard: 7 bytes, took 1.187µs
  --------------------
  
  SUMMARY:
  --------------------
  Avg Bincode: 4106 bytes, time: 4.007µs
  Avg Msgpack: 4117 bytes, time: 9.646µs
  Avg Postcard: 4099 bytes, time: 2.671µs
  --------------------
  -------------------------
  
  Deserialization:
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.187µs
  Postcard: took 2.375µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 0ns
  --------------------
  --------------------
  Bincode: took 1.188µs
  Msgpack: took 1.187µs
  Postcard: took 1.188µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.187µs
  Postcard: took 1.188µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 1.187µs
  --------------------
  --------------------
  Bincode: took 23.746µs
  Msgpack: took 89.048µs
  Postcard: took 1.188µs
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.187µs
  Postcard: took 0ns
  --------------------
  --------------------
  Bincode: took 1.187µs
  Msgpack: took 1.188µs
  Postcard: took 1.187µs
  --------------------
  
  SUMMARY:
  --------------------
  Avg Bincode: time: 4.007µs
  Avg Msgpack: time: 12.17µs
  Avg Postcard: time: 1.039µs
  --------------------
  ```
</details>