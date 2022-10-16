The purpose of this project is to make it easier for other developers to create ICA across different cosmos chains. In order to reduce the need to upload an [ica-host](./contracts/ica-host/) for each developer/team and prevent unnecessary use of space on the blockchain.

The process consists of deploying a contract that acts as a host in each chain and once a connection is established it deploys a [CW1](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw1-whitelist) that will be the ICA.

You can find a presentation of how implement "Simple ICA" contracts that is the base of this project. [Ethan's HackAtom Video](https://www.youtube.com/watch?v=x75UobIr4qo&t=9070s).

[cosmwasm-ica](https://crates.io/crates/cosmwasm-ica) is a library to facilitate the use of ICA in your contract for sending/receiving message to/from the host.

The code used in this project was presented at HackAtom and is available at [cw-ibc-demo](https://github.com/confio/cw-ibc-demo).

| Chain | Network | Contract Address | CW1-Code ID | CW1 Upload Tx | CW-ICA-HOST-Code ID | CW-ICA-HOST Upload Tx & Instantiation Tx |
| - | - | - | - | - | - | - |
| Osmosis | Testnet | osmo1cvqz3yysqhr980ed8uswydl2pegrp0eglen3ruyaqnve0ennq6fs3cxmdg | 2943 |  [Tx][Juno-Testnet-CW1-Upload] | 2942 | [Upload Tx][Osmosis-Testnet-ICA-Host-Upload] & [Instantiation Tx][Osmosis-Testnet-ICA-Host-Instantiation]
| Juno | Testnet | juno18fxhy3chejj655237gsa4nrzxx6sz8u3lkejtmlj78c2hgre7fcsz56z24 | 1087 | [Tx][Juno-Testnet-CW1-Upload] | 1088 | [Upload Tx][Juno-Testnet-ICA-Host-Upload] & [Instantiation Tx][Juno-Testnet-ICA-Host-Instantiation]


[Juno-Testnet-CW1-Upload]: https://testnet.mintscan.io/juno-testnet/txs/3AB898B83C55B1E5980E2112B8D3BE8AEB14DD5CAAB1D4A0E81057676AF68813
[Juno-Testnet-ICA-Host-Upload]: https://testnet.mintscan.io/juno-testnet/txs/446509DE045FA08B5C3A18784D98725633D3590CB3CF4DF87A61B105CC3DEC55
[Juno-Testnet-ICA-Host-Instantiation]: https://testnet.mintscan.io/juno-testnet/txs/7DB3608F86910F6F91BA03E7906AA1B71C3F8B8ABDB9ABA3E6D8179FA82FC387

[Osmosis-Testnet-CW1-Upload]: https://testnet.mintscan.io/osmosis-testnet/txs/C7784B5D93DF33DC4DB2DBE7B63CA94C1AAE95C54561D3DD3DFA7C310A35809F
[Osmosis-Testnet-ICA-Host-Upload]: https://testnet.mintscan.io/osmosis-testnet/txs/0E9F8B62FEBFBE5808075B9EFD1DFF9E0A474F16DF7263A0A1108D4CB9DD1ABA
[Osmosis-Testnet-ICA-Host-Instantiation]: https://testnet.mintscan.io/osmosis-testnet/txs/F779E254046520F63F8F5F5DA3470BDE1D2DD292C1DE77090A432187B96EB1D3
