# Bitprim powered BTC/BCH node for corporate custodian tasks

[![CircleCI](https://circleci.com/gh/bitex-la/custodian_server.svg?style=svg)](https://circleci.com/gh/bitex-la/custodian_server)

## How to use it

### Quick Start

```
git clone git@github.com:bitex-la/custodian_server.git
cd custodian_server/bin
./custodian_server_btc mainnet_btc.cfg
``` 

### Docker

#### Intro to docker
- [Getting started](https://docs.docker.com/get-started/#recap-and-cheat-sheet)
- [Docker cheat sheet](https://github.com/wsargent/docker-cheat-sheet)

#### System Requirements
- Install [docker](https://www.docker.com/community-edition#/download)
- Install [docker compose](https://docs.docker.com/compose/install/#install-compose)

#### How to install and run

```
git clone git@github.com:bitex-la/custodian_server.git
cd custodian_server
docker-compose build
docker-compose up -d
```

