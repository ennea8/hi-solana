

```bash
## build
cd counter_program
cargo build-bpf

cd ..
solana program deploy ./target/deploy/counter_program.so


## deploy
solana program deploy ./target/deploy/counter_program.so

## call
node client.mjs  

##  view
https://explorer.solana.com/

```
