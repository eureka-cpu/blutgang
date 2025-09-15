# Blutgang - the wd40 of ethereum load balancers
![blutgang_gm](https://github.com/rainshowerLabs/blutgang/assets/55022497/ec668c7a-5f56-4b26-8386-f112c2f176ce)

Join the discussion on our [discord](https://discord.gg/92TfQWdjEh), [telegram](https://t.me/rainshower), or [matrix!](https://matrix.to/#/%23rainshower:matrix.org)

Blutgang is a blazing fast, caching, minimalistic load balancer designed with Ethereum's JSON-RPC in mind. Historical RPC queries are cached in a local database, bypassing the need for slow, repeating calls to your node.

For more info about blutgang and how to use it, please check out the [wiki](https://github.com/rainshowerLabs/blutgang/wiki).

## How to run 

For detailed instructions on how to use blutgang, please read the [wiki](https://github.com/rainshowerLabs/blutgang/wiki).

### Using cargo

To install blutgang via cargo, run the following command:

```bash
cargo install blutgang
```
Once done, grab the `example_config.toml` from this repository, modify it to your liking, and start blutgang with it.

### From source

Clone the repository, and find the `example_config.toml` file. Edit it to your liking, and run `cargo run --release -- -c example_config.toml`.   

If you want to use command line arguments instead, please run `cargo run --release -- --help` for more info. Keep in mind that the recommended way to run blutgang is via a config file.

### Max performance

If you need the absolute maximum performance from blutgang, compile it using the command below:

```bash
RUSTFLAGS='-C target-cpu=native' cargo build --profile maxperf
```

### Docker

The official docker image is available on [dockerhub](https://hub.docker.com/r/makemake1337/blutgang).  
You must provide a config file to the docker container, as well as expose the port specified. Example:   
```bash
docker run -v /full/path/to/config.toml:/app/config.toml --network host makemake1337/blutgang
```

### Nix

Using Flakes and the [nix-community/ethereum.nix](https://github.com/nix-community/ethereum.nix) overlay:

```bash
nix run github:nix-community/ethereum.nix#blutgang -- --help
```

## Benchmarks
*Benchmarks were performed with a Ryzen 7 2700X, NVME SSD, and default Ubuntu 23.04 kernel. Same RPC endpoints were used*

```bash
time sothis --source_rpc http://localhost:3000 --mode call_track --contract_address 0x1c479675ad559DC151F6Ec7ed3FbF8ceE79582B6 --origin_block 17885300 --terminal_block 17892269 --calldata 0x06f13056 --query_interval 20
```
![Figure_1](https://github.com/rainshowerLabs/blutgang/assets/55022497/8ce9a690-d2eb-4910-9a5d-807c2bdd4649)
![Figure_2](https://github.com/rainshowerLabs/blutgang/assets/55022497/50d78e5f-2209-488d-82fc-8018388a82e7)

## Acknowledgements

- [dshackle](https://github.com/emeraldpay/dshackle)
- [proxyd](https://github.com/ethereum-optimism/optimism/tree/develop/proxyd)
- [web3-proxy](https://github.com/llamanodes/web3-proxy)

Blutgang is standing on the shoulders of giants. Thank you to all the contributors of the projects above!
