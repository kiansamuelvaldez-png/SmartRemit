## README.md

```md
# Scan2Cook

A mobile app that generates recipes from scanned ingredients and unlocks premium recipes via Stellar micropayments.

Problem
A college student in Parañaque struggles to decide what to cook with limited ingredients, leading to food waste and extra expenses.

Solution
Scan ingredients → get recipes → pay small XLM fee → unlock premium recipes instantly via Stellar.

 Timeline
- Week 1: UI + ingredient scanner
- Week 2: Recipe generator + contract integration
- Week 3: Testing + demo

Stellar Features Used
- XLM payments
- Soroban smart contracts

Vision and Purpose
Reduce food waste and make cooking easier and more affordable through smart automation and microtransactions.

Prerequisites
- Rust
- Soroban CLI

Build
```

soroban contract build

```

## Test
```

cargo test

```

## Deploy
```

soroban contract deploy --wasm target/wasm32-unknown-unknown/release/scan2cook.wasm

```

## Sample Call
```

soroban contract invoke 
--id <CONTRACT_ID> 
--fn pay_and_unlock 
--arg user=<USER_ADDRESS> 
--arg recipe_id="R1"

```

## License
MIT

https://stellar.expert/explorer/testnet/tx/1e58a84036fcdaef651f451ee82e8dd48306671c435f16f18c635df683b4a1bd