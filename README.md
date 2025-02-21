

## counter_program

```bash
## build
cd counter_program
cargo build-sbf

cd ..
solana program deploy ./target/deploy/counter_program.so


## deploy
solana program deploy ./target/deploy/counter_program.so

## call
pnpx esrun ./scripts/counter.ts

##  view
https://explorer.solana.com/

```


## movie-review

```sh
pnpx esrun ./scripts/movie-review.ts

```