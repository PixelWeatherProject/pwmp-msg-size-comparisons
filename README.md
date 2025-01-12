# Benchmarks for PWMP Serialization Formats

This repo contains the code used to benchmark different serialization formats that are potencially viable for use in the PixelWeather Messaging Protocol.

## Formats
- [x] bincode
- [x] msgpack
- [x] postcard

## Results

### Test on PC
|                         | bincode 1.3.3 | rmp-serde 1.3.0 | postcard 1.1.1 |
| ----------------------- | ------------- | --------------- | -------------- |
| Avg. Serialized size    | 2064 bytes    | 2075 bytes      | 2056 bytes     |
| Avg. Serialization time | 2.597µs       | 5.5455µs        | 1.929µs        |
| Deserialization time    | 2.597µs       | 6.7445µs        | 0.915µs        |

Postcard and bincode seem to perform the best. Postcard serializes to a much smaller number of bytes. This isn't really visible in the table due to the large size of the `UpdatePart` message. With almost all message types, Postcard produces 40-75% less bytes on average.

#### Details
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

### Test on ESP32S3@160MHz@100Hz-RTOS
|                         | bincode 1.3.3 | rmp-serde 1.3.0 | postcard 1.1.1 |
| ----------------------- | ------------- | --------------- | -------------- |
| Avg. Serialized size    | 2064 bytes    | 2075 bytes      | 2056 bytes     |
| Avg. Serialization time | 1.042ms       | 2.918ms         | 305.917µs      |
| Deserialization time    | 1.315ms       | 1.436ms         | 58.465µs       |

**Note:** The time durations above have been rounded up to 3 decimals.

#### Details
<details>
  <summary>Detailed log here</summary>
  
  ```
  I (409) size_tests: Serialization:
  I (419) size_tests::requests: --------------------
  I (419) size_tests::requests: Message: Request(Hello { mac: Mac(1, 2, 3, 4, 5, 6) })
  I (429) size_tests::requests: Bincode: 14 bytes, took 161µs
  I (439) size_tests::requests: Msgpack: 24 bytes, took 391µs
  I (439) size_tests::requests: Postcard: 8 bytes, took 144µs
  I (449) size_tests::requests: --------------------
  I (459) size_tests::requests: --------------------
  I (459) size_tests::requests: Message: Request(PostResults { temperature: 20.44, humidity: 67, air_pressure: Some(65535) })
  I (469) size_tests::requests: Bincode: 25 bytes, took 306µs
  I (479) size_tests::requests: Msgpack: 33 bytes, took 421µs
  I (479) size_tests::requests: Postcard: 13 bytes, took 166µs
  I (489) size_tests::requests: --------------------
  I (499) size_tests::requests: --------------------
  I (499) size_tests::requests: Message: Request(PostStats { battery: 4.20, wifi_ssid: "ABCDEFGHIJKLMNOPQRSTUVXYZ0123456", wifi_rssi: -85 })
  I (509) size_tests::requests: Bincode: 61 bytes, took 198µs
  I (519) size_tests::requests: Msgpack: 62 bytes, took 360µs
  I (529) size_tests::requests: Postcard: 41 bytes, took 140µs
  I (529) size_tests::requests: --------------------
  I (539) size_tests::requests: --------------------
  I (549) size_tests::requests: Message: Request(SendNotification("The battery is too low, activating sBOP."))
  I (559) size_tests::requests: Bincode: 56 bytes, took 135µs
  I (559) size_tests::requests: Msgpack: 69 bytes, took 268µs
  I (569) size_tests::requests: Postcard: 43 bytes, took 117µs
  I (569) size_tests::requests: --------------------
  I (579) size_tests::requests: --------------------
  I (589) size_tests::requests: Message: Request(GetSettings)
  I (589) size_tests::requests: Bincode: 8 bytes, took 81µs
  I (599) size_tests::requests: Msgpack: 21 bytes, took 186µs
  I (599) size_tests::requests: Postcard: 2 bytes, took 76µs
  I (609) size_tests::requests: --------------------
  I (619) size_tests::requests: --------------------
  I (619) size_tests::requests: Message: Request(UpdateCheck(Version { major: 1, middle: 0, minor: 1 }))
  I (629) size_tests::requests: Bincode: 11 bytes, took 94µs
  I (639) size_tests::requests: Msgpack: 26 bytes, took 235µs
  I (639) size_tests::requests: Postcard: 5 bytes, took 82µs
  I (649) size_tests::requests: --------------------
  I (659) size_tests::requests: --------------------
  I (659) size_tests::requests: Message: Request(NextUpdateChunk(1024))
  I (669) size_tests::requests: Bincode: 16 bytes, took 102µs
  I (669) size_tests::requests: Msgpack: 29 bytes, took 278µs
  I (679) size_tests::requests: Postcard: 4 bytes, took 89µs
  I (689) size_tests::requests: --------------------
  I (689) size_tests::requests: --------------------
  I (699) size_tests::requests: Message: Request(ReportFirmwareUpdate(false))
  I (709) size_tests::requests: Bincode: 9 bytes, took 82µs
  I (709) size_tests::requests: Msgpack: 32 bytes, took 192µs
  I (719) size_tests::requests: Postcard: 3 bytes, took 71µs
  I (719) size_tests::requests: --------------------
  I (729) size_tests::requests: --------------------
  I (739) size_tests::requests: Message: Request(Bye)
  I (739) size_tests::requests: Bincode: 8 bytes, took 86µs
  I (749) size_tests::requests: Msgpack: 13 bytes, took 145µs
  I (749) size_tests::requests: Postcard: 2 bytes, took 63µs
  I (759) size_tests::requests: --------------------
  I (769) size_tests::requests:  SUMMARY:
  I (769) size_tests::requests: --------------------
  I (779) size_tests::requests: Avg Bincode: 23 bytes, time: 138.333µs
  I (779) size_tests::requests: Avg Msgpack: 34 bytes, time: 275.111µs
  I (789) size_tests::requests: Avg Postcard: 13 bytes, time: 105.333µs
  I (799) size_tests::requests: --------------------
  I (799) size_tests: -------------------------
  I (809) size_tests: Deserialization:
  I (809) size_tests::requests: --------------------
  I (819) size_tests::requests: Bincode: took 215µs
  I (819) size_tests::requests: Msgpack: took 538µs
  I (829) size_tests::requests: Postcard: took 77µs
  I (829) size_tests::requests: --------------------
  I (839) size_tests::requests: --------------------
  I (839) size_tests::requests: Bincode: took 194µs
  I (849) size_tests::requests: Msgpack: took 725µs
  I (859) size_tests::requests: Postcard: took 70µs
  I (859) size_tests::requests: --------------------
  I (869) size_tests::requests: --------------------
  I (869) size_tests::requests: Bincode: took 178µs
  I (879) size_tests::requests: Msgpack: took 779µs
  I (879) size_tests::requests: Postcard: took 65µs
  I (889) size_tests::requests: --------------------
  I (899) size_tests::requests: --------------------
  I (899) size_tests::requests: Bincode: took 265µs
  I (899) size_tests::requests: Msgpack: took 371µs
  I (909) size_tests::requests: Postcard: took 69µs
  I (919) size_tests::requests: --------------------
  I (919) size_tests::requests: --------------------
  I (929) size_tests::requests: Bincode: took 139µs
  I (929) size_tests::requests: Msgpack: took 252µs
  I (939) size_tests::requests: Postcard: took 77µs
  I (939) size_tests::requests: --------------------
  I (949) size_tests::requests: --------------------
  I (949) size_tests::requests: Bincode: took 175µs
  I (959) size_tests::requests: Msgpack: took 385µs
  I (959) size_tests::requests: Postcard: took 76µs
  I (969) size_tests::requests: --------------------
  I (979) size_tests::requests: --------------------
  I (979) size_tests::requests: Bincode: took 164µs
  I (989) size_tests::requests: Msgpack: took 368µs
  I (989) size_tests::requests: Postcard: took 89µs
  I (999) size_tests::requests: --------------------
  I (999) size_tests::requests: --------------------
  I (1009) size_tests::requests: Bincode: took 179µs
  I (1009) size_tests::requests: Msgpack: took 342µs
  I (1019) size_tests::requests: Postcard: took 88µs
  I (1029) size_tests::requests: --------------------
  I (1029) size_tests::requests: --------------------
  I (1039) size_tests::requests: Bincode: took 138µs
  I (1039) size_tests::requests: Msgpack: took 269µs
  I (1049) size_tests::requests: Postcard: took 69µs
  I (1049) size_tests::requests: --------------------
  I (1059) size_tests::requests: SUMMARY:
  I (1059) size_tests::requests: --------------------
  I (1069) size_tests::requests: Avg Bincode: time: 183µs
  I (1069) size_tests::requests: Avg Msgpack: time: 447.666µs
  I (1079) size_tests::requests: Avg Postcard: time: 75.555µs
  I (1089) size_tests::requests: --------------------
  I (1089) size_tests: -------------------------
  I (1099) size_tests: Serialization:
  I (1099) size_tests::responses: --------------------
  I (1109) size_tests::responses: Message: Response(Pong)
  I (1109) size_tests::responses: Bincode: 8 bytes, took 98µs
  I (1119) size_tests::responses: Msgpack: 15 bytes, took 238µs
  I (1129) size_tests::responses: Postcard: 2 bytes, took 100µs
  I (1129) size_tests::responses: --------------------
  I (1139) size_tests::responses: --------------------
  I (1139) size_tests::responses: Message: Response(Ok)
  I (1149) size_tests::responses: Bincode: 8 bytes, took 94µs
  I (1159) size_tests::responses: Msgpack: 13 bytes, took 187µs
  I (1159) size_tests::responses: Postcard: 2 bytes, took 86µs
  I (1169) size_tests::responses: --------------------
  I (1179) size_tests::responses: --------------------
  I (1179) size_tests::responses: Message: Response(Reject)
  I (1189) size_tests::responses: Bincode: 8 bytes, took 66µs
  I (1189) size_tests::responses: Msgpack: 17 bytes, took 110µs
  I (1199) size_tests::responses: Postcard: 2 bytes, took 62µs
  I (1209) size_tests::responses: --------------------
  I (1209) size_tests::responses: --------------------
  I (1219) size_tests::responses: Message: Response(FirmwareUpToDate)
  I (1219) size_tests::responses: Bincode: 8 bytes, took 67µs
  I (1229) size_tests::responses: Msgpack: 27 bytes, took 116µs
  I (1239) size_tests::responses: Postcard: 2 bytes, took 69µs
  I (1239) size_tests::responses: --------------------
  I (1249) size_tests::responses: --------------------
  I (1259) size_tests::responses: Message: Response(UpdateAvailable(Versi
  I (1259) size_tests::responses: Bincode: 11 bytes, took 82µs
  I (1269) size_tests::responses: Msgpack: 31 bytes, took 197µs
  I (1279) size_tests::responses: Postcard: 5 bytes, took 86µs
  I (1279) size_tests::responses: --------------------
  I (1349) size_tests::responses: --------------------
  I (1439) size_tests::responses: Message: Response(UpdatePart([65, 66, 6
  I (1439) size_tests::responses: Bincode: 32784 bytes, took 15.003ms
  I (1439) size_tests::responses: Msgpack: 32793 bytes, took 43.21ms
  I (1449) size_tests::responses: Postcard: 32773 bytes, took 3.472ms
  I (1449) size_tests::responses: --------------------
  I (1459) size_tests::responses: --------------------
  I (1459) size_tests::responses: Message: Response(UpdateEnd)
  I (1469) size_tests::responses: Bincode: 8 bytes, took 55µs
  I (1479) size_tests::responses: Msgpack: 20 bytes, took 190µs
  I (1479) size_tests::responses: Postcard: 2 bytes, took 71µs
  I (1489) size_tests::responses: --------------------
  I (1499) size_tests::responses: --------------------
  I (1499) size_tests::responses: Message: Response(Settings(NodeSettings
  I (1509) size_tests::responses: Bincode: 14 bytes, took 96µs
  I (1509) size_tests::responses: Msgpack: 26 bytes, took 239µs
  I (1519) size_tests::responses: Postcard: 7 bytes, took 106µs
  I (1529) size_tests::responses: --------------------
  I (1529) size_tests::responses: SUMMARY:
  I (1539) size_tests::responses: --------------------
  I (1539) size_tests::responses: Avg Bincode: 4106 bytes, time: 1.945125ms
  I (1549) size_tests::responses: Avg Msgpack: 4117 bytes, time: 5.560875ms
  I (1559) size_tests::responses: Avg Postcard: 4099 bytes, time: 506.5µs
  I (1569) size_tests::responses: --------------------
  I (1569) size_tests: -------------------------
  I (1579) size_tests: Deserialization:
  I (1579) size_tests::responses: --------------------
  I (1589) size_tests::responses: Bincode: took 156µs
  I (1589) size_tests::responses: Msgpack: took 306µs
  I (1599) size_tests::responses: Postcard: took 49µs
  I (1599) size_tests::responses: --------------------
  I (1609) size_tests::responses: --------------------
  I (1619) size_tests::responses: Bincode: took 135µs
  I (1619) size_tests::responses: Msgpack: took 204µs
  I (1629) size_tests::responses: Postcard: took 30µs
  I (1629) size_tests::responses: --------------------
  I (1639) size_tests::responses: --------------------
  I (1639) size_tests::responses: Bincode: took 143µs
  I (1649) size_tests::responses: Msgpack: took 228µs
  I (1649) size_tests::responses: Postcard: took 41µs
  I (1659) size_tests::responses: --------------------
  I (1669) size_tests::responses: --------------------
  I (1669) size_tests::responses: Bincode: took 130µs
  I (1679) size_tests::responses: Msgpack: took 212µs
  I (1679) size_tests::responses: Postcard: took 29µs
  I (1689) size_tests::responses: --------------------
  I (1699) size_tests::responses: --------------------
  I (1699) size_tests::responses: Bincode: took 179µs
  I (1709) size_tests::responses: Msgpack: took 417µs
  I (1709) size_tests::responses: Postcard: took 49µs
  I (1719) size_tests::responses: --------------------
  I (1829) size_tests::responses: --------------------
  I (1829) size_tests::responses: Bincode: took 18.488ms
  I (1829) size_tests::responses: Msgpack: took 17.306ms
  I (1829) size_tests::responses: Postcard: took 48µs
  I (1839) size_tests::responses: --------------------
  I (1849) size_tests::responses: --------------------
  I (1849) size_tests::responses: Bincode: took 136µs
  I (1859) size_tests::responses: Msgpack: took 276µs
  I (1859) size_tests::responses: Postcard: took 41µs
  I (1869) size_tests::responses: --------------------
  I (1879) size_tests::responses: --------------------
  I (1879) size_tests::responses: Bincode: took 208µs
  I (1889) size_tests::responses: Msgpack: took 441µs
  I (1889) size_tests::responses: Postcard: took 44µs
  I (1899) size_tests::responses: --------------------
  I (1899) size_tests::responses: SUMMARY:
  I (1909) size_tests::responses: --------------------
  I (1909) size_tests::responses: Avg Bincode: time: 2.446875ms
  I (1919) size_tests::responses: Avg Msgpack: time: 2.42375ms
  I (1929) size_tests::responses: Avg Postcard: time: 41.375µs
  I (1929) size_tests::responses: --------------------
  ```
</details>

### Test on ESP32S3@240Mhz@100Hz-RTOS
|                         | bincode 1.3.3 | rmp-serde 1.3.0 | postcard 1.1.1 |
| ----------------------- | ------------- | --------------- | -------------- |
| Avg. Serialized size    | 2064 bytes    | 2075 bytes      | 2056 bytes     |
| Avg. Serialization time | 697.368µs     | 1.941ms         | 228.819µs      |
| Deserialization time    | 862.486µs     | 869.653µs       | 18.278µs       |

**Note:** The time durations above have been rounded up to 3 decimals.

#### Details
<details>
  <summary>Detailed log here</summary>
  
  ```
  I (894) size_tests: Serialization:
  I (899) size_tests::requests: --------------------
  I (904) size_tests::requests: Message: Request(Hello { mac: Mac(1, 2, 3, 4, 5, 6) })
  I (912) size_tests::requests: Bincode: 14 bytes, took 51µs
  I (918) size_tests::requests: Msgpack: 24 bytes, took 130µs
  I (925) size_tests::requests: Postcard: 8 bytes, took 43µs
  I (931) size_tests::requests: --------------------
  I (937) size_tests::requests: --------------------
  I (942) size_tests::requests: Message: Request(PostResults { temperature: 20.44, humidity: 67, air_pressure: Some(65535) })
  I (954) size_tests::requests: Bincode: 25 bytes, took 93µs
  I (960) size_tests::requests: Msgpack: 33 bytes, took 153µs
  I (966) size_tests::requests: Postcard: 13 bytes, took 58µs
  I (973) size_tests::requests: --------------------
  I (978) size_tests::requests: --------------------
  I (983) size_tests::requests: Message: Request(PostStats { battery: 4.20, wifi_ssid: "ABCDEFGHIJKLMNOPQRSTUVXYZ0123456", wifi_rssi: -85 })
  I (997) size_tests::requests: Bincode: 61 bytes, took 69µs
  I (1003) size_tests::requests: Msgpack: 62 bytes, took 138µs
  I (1009) size_tests::requests: Postcard: 41 bytes, took 55µs
  I (1016) size_tests::requests: --------------------
  I (1021) size_tests::requests: --------------------
  I (1027) size_tests::requests: Message: Request(SendNotification("The battery is too low, activating sBOP."))
  I (1037) size_tests::requests: Bincode: 56 bytes, took 34µs
  I (1044) size_tests::requests: Msgpack: 69 bytes, took 78µs
  I (1050) size_tests::requests: Postcard: 43 bytes, took 30µs
  I (1057) size_tests::requests: --------------------
  I (1062) size_tests::requests: --------------------
  I (1068) size_tests::requests: Message: Request(GetSettings)
  I (1074) size_tests::requests: Bincode: 8 bytes, took 23µs
  I (1080) size_tests::requests: Msgpack: 21 bytes, took 59µs
  I (1087) size_tests::requests: Postcard: 2 bytes, took 23µs
  I (1093) size_tests::requests: --------------------
  I (1099) size_tests::requests: --------------------
  I (1104) size_tests::requests: Message: Request(UpdateCheck(Version { major: 1, middle: 0, minor: 1 }))
  I (1114) size_tests::requests: Bincode: 11 bytes, took 24µs
  I (1120) size_tests::requests: Msgpack: 26 bytes, took 78µs
  I (1127) size_tests::requests: Postcard: 5 bytes, took 29µs
  I (1133) size_tests::requests: --------------------
  I (1139) size_tests::requests: --------------------
  I (1144) size_tests::requests: Message: Request(NextUpdateChunk(1024))
  I (1151) size_tests::requests: Bincode: 16 bytes, took 35µs
  I (1158) size_tests::requests: Msgpack: 29 bytes, took 81µs
  I (1164) size_tests::requests: Postcard: 4 bytes, took 31µs
  I (1170) size_tests::requests: --------------------
  I (1176) size_tests::requests: --------------------
  I (1181) size_tests::requests: Message: Request(ReportFirmwareUpdate(false))
  I (1189) size_tests::requests: Bincode: 9 bytes, took 19µs
  I (1195) size_tests::requests: Msgpack: 32 bytes, took 61µs
  I (1202) size_tests::requests: Postcard: 3 bytes, took 22µs
  I (1208) size_tests::requests: --------------------
  I (1214) size_tests::requests: --------------------
  I (1219) size_tests::requests: Message: Request(Bye)
  I (1225) size_tests::requests: Bincode: 8 bytes, took 22µs
  I (1231) size_tests::requests: Msgpack: 13 bytes, took 46µs
  I (1237) size_tests::requests: Postcard: 2 bytes, took 23µs
  I (1244) size_tests::requests: --------------------
  I (1249) size_tests::requests: SUMMARY:
  I (1254) size_tests::requests: --------------------
  I (1260) size_tests::requests: Avg Bincode: 23 bytes, time: 41.111µs
  I (1267) size_tests::requests: Avg Msgpack: 34 bytes, time: 91.555µs
  I (1274) size_tests::requests: Avg Postcard: 13 bytes, time: 34.888µs
  I (1281) size_tests::requests: --------------------
  I (1287) size_tests: -------------------------
  I (1292) size_tests: Deserialization:
  I (1297) size_tests::requests: --------------------
  I (1302) size_tests::requests: Bincode: took 65µs
  I (1307) size_tests::requests: Msgpack: took 169µs
  I (1313) size_tests::requests: Postcard: took 23µs
  I (1318) size_tests::requests: --------------------
  I (1325) size_tests::requests: --------------------
  I (1329) size_tests::requests: Bincode: took 60µs
  I (1335) size_tests::requests: Msgpack: took 215µs
  I (1340) size_tests::requests: Postcard: took 23µs
  I (1346) size_tests::requests: --------------------
  I (1352) size_tests::requests: --------------------
  I (1357) size_tests::requests: Bincode: took 57µs
  I (1363) size_tests::requests: Msgpack: took 238µs
  I (1368) size_tests::requests: Postcard: took 21µs
  I (1374) size_tests::requests: --------------------
  I (1380) size_tests::requests: --------------------
  I (1385) size_tests::requests: Bincode: took 84µs
  I (1390) size_tests::requests: Msgpack: took 126µs
  I (1396) size_tests::requests: Postcard: took 20µs
  I (1401) size_tests::requests: --------------------
  I (1407) size_tests::requests: --------------------
  I (1412) size_tests::requests: Bincode: took 47µs
  I (1418) size_tests::requests: Msgpack: took 85µs
  I (1423) size_tests::requests: Postcard: took 26µs
  I (1429) size_tests::requests: --------------------
  I (1435) size_tests::requests: --------------------
  I (1440) size_tests::requests: Bincode: took 57µs
  I (1446) size_tests::requests: Msgpack: took 119µs
  I (1451) size_tests::requests: Postcard: took 21µs
  I (1457) size_tests::requests: --------------------
  I (1463) size_tests::requests: --------------------
  I (1468) size_tests::requests: Bincode: took 50µs
  I (1473) size_tests::requests: Msgpack: took 121µs
  I (1479) size_tests::requests: Postcard: took 29µs
  I (1484) size_tests::requests: --------------------
  I (1490) size_tests::requests: --------------------
  I (1495) size_tests::requests: Bincode: took 57µs
  I (1501) size_tests::requests: Msgpack: took 108µs
  I (1506) size_tests::requests: Postcard: took 25µs
  I (1512) size_tests::requests: --------------------
  I (1518) size_tests::requests: --------------------
  I (1523) size_tests::requests: Bincode: took 47µs
  I (1529) size_tests::requests: Msgpack: took 93µs
  I (1534) size_tests::requests: Postcard: took 24µs
  I (1540) size_tests::requests: --------------------
  I (1545) size_tests::requests: SUMMARY:
  I (1550) size_tests::requests: --------------------
  I (1555) size_tests::requests: Avg Bincode: time: 58.222µs
  I (1562) size_tests::requests: Avg Msgpack: time: 141.555µs
  I (1568) size_tests::requests: Avg Postcard: time: 23.555µs
  I (1574) size_tests::requests: --------------------
  I (1580) size_tests: -------------------------
  I (1585) size_tests: Serialization:
  I (1590) size_tests::responses: --------------------
  I (1595) size_tests::responses: Message: Response(Pong)
  I (1601) size_tests::responses: Bincode: 8 bytes, took 38µs
  I (1607) size_tests::responses: Msgpack: 15 bytes, took 85µs
  I (1614) size_tests::responses: Postcard: 2 bytes, took 28µs
  I (1620) size_tests::responses: --------------------
  I (1626) size_tests::responses: --------------------
  I (1631) size_tests::responses: Message: Response(Ok)
  I (1637) size_tests::responses: Bincode: 8 bytes, took 29µs
  I (1643) size_tests::responses: Msgpack: 13 bytes, took 74µs
  I (1650) size_tests::responses: Postcard: 2 bytes, took 28µs
  I (1656) size_tests::responses: --------------------
  I (1662) size_tests::responses: --------------------
  I (1668) size_tests::responses: Message: Response(Reject)
  I (1674) size_tests::responses: Bincode: 8 bytes, took 17µs
  I (1680) size_tests::responses: Msgpack: 17 bytes, took 58µs
  I (1686) size_tests::responses: Postcard: 2 bytes, took 21µs
  I (1693) size_tests::responses: --------------------
  I (1699) size_tests::responses: --------------------
  I (1704) size_tests::responses: Message: Response(FirmwareUpToDate)
  I (1711) size_tests::responses: Bincode: 8 bytes, took 23µs
  I (1717) size_tests::responses: Msgpack: 27 bytes, took 66µs
  I (1724) size_tests::responses: Postcard: 2 bytes, took 22µs
  I (1730) size_tests::responses: --------------------
  I (1736) size_tests::responses: --------------------
  I (1742) size_tests::responses: Message: Response(UpdateAvailable(Versi
  I (1749) size_tests::responses: Bincode: 11 bytes, took 24µs
  I (1755) size_tests::responses: Msgpack: 31 bytes, took 87µs
  I (1762) size_tests::responses: Postcard: 5 bytes, took 30µs
  I (1768) size_tests::responses: --------------------
  I (1817) size_tests::responses: --------------------
  I (1876) size_tests::responses: Message: Response(UpdatePart([65, 66, 6
  I (1876) size_tests::responses: Bincode: 32784 bytes, took 10.645ms
  I (1879) size_tests::responses: Msgpack: 32793 bytes, took 29.773ms
  I (1886) size_tests::responses: Postcard: 32773 bytes, took 3.193ms
  I (1893) size_tests::responses: --------------------
  I (1899) size_tests::responses: --------------------
  I (1905) size_tests::responses: Message: Response(UpdateEnd)
  I (1911) size_tests::responses: Bincode: 8 bytes, took 24µs
  I (1917) size_tests::responses: Msgpack: 20 bytes, took 75µs
  I (1924) size_tests::responses: Postcard: 2 bytes, took 26µs
  I (1930) size_tests::responses: --------------------
  I (1936) size_tests::responses: --------------------
  I (1941) size_tests::responses: Message: Response(Settings(NodeSettings
  I (1949) size_tests::responses: Bincode: 14 bytes, took 29µs
  I (1955) size_tests::responses: Msgpack: 26 bytes, took 103µs
  I (1962) size_tests::responses: Postcard: 7 bytes, took 34µs
  I (1968) size_tests::responses: --------------------
  I (1974) size_tests::responses: SUMMARY:
  I (1978) size_tests::responses: --------------------
  I (1984) size_tests::responses: Avg Bincode: 4106 bytes, time: 1.353625ms
  I (1992) size_tests::responses: Avg Msgpack: 4117 bytes, time: 3.790125ms
  I (1999) size_tests::responses: Avg Postcard: 4099 bytes, time: 422.75µs
  I (2006) size_tests::responses: --------------------
  I (2012) size_tests: -------------------------
  I (2017) size_tests: Deserialization:
  I (2023) size_tests::responses: --------------------
  I (2027) size_tests::responses: Bincode: took 52µs
  I (2033) size_tests::responses: Msgpack: took 96µs
  I (2038) size_tests::responses: Postcard: took 16µs
  I (2044) size_tests::responses: --------------------
  I (2050) size_tests::responses: --------------------
  I (2055) size_tests::responses: Bincode: took 46µs
  I (2061) size_tests::responses: Msgpack: took 66µs
  I (2066) size_tests::responses: Postcard: took 9µs
  I (2072) size_tests::responses: --------------------
  I (2078) size_tests::responses: --------------------
  I (2083) size_tests::responses: Bincode: took 47µs
  I (2089) size_tests::responses: Msgpack: took 73µs
  I (2094) size_tests::responses: Postcard: took 11µs
  I (2100) size_tests::responses: --------------------
  I (2106) size_tests::responses: --------------------
  I (2111) size_tests::responses: Bincode: took 45µs
  I (2117) size_tests::responses: Msgpack: took 71µs
  I (2122) size_tests::responses: Postcard: took 14µs
  I (2128) size_tests::responses: --------------------
  I (2134) size_tests::responses: --------------------
  I (2139) size_tests::responses: Bincode: took 59µs
  I (2145) size_tests::responses: Msgpack: took 129µs
  I (2151) size_tests::responses: Postcard: took 13µs
  I (2156) size_tests::responses: --------------------
  I (2236) size_tests::responses: --------------------
  I (2236) size_tests::responses: Bincode: took 12.97ms
  I (2237) size_tests::responses: Msgpack: took 12.116ms
  I (2242) size_tests::responses: Postcard: took 15µs
  I (2248) size_tests::responses: --------------------
  I (2254) size_tests::responses: --------------------
  I (2259) size_tests::responses: Bincode: took 49µs
  I (2265) size_tests::responses: Msgpack: took 90µs
  I (2270) size_tests::responses: Postcard: took 11µs
  I (2276) size_tests::responses: --------------------
  I (2282) size_tests::responses: --------------------
  I (2287) size_tests::responses: Bincode: took 66µs
  I (2293) size_tests::responses: Msgpack: took 141µs
  I (2298) size_tests::responses: Postcard: took 15µs
  I (2304) size_tests::responses: --------------------
  I (2310) size_tests::responses: SUMMARY:
  I (2314) size_tests::responses: --------------------
  I (2320) size_tests::responses: Avg Bincode: time: 1.66675ms
  I (2326) size_tests::responses: Avg Msgpack: time: 1.59775ms
  I (2333) size_tests::responses: Avg Postcard: time: 13µs
  I (2339) size_tests::responses: --------------------
  ```
</details>