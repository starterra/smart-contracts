# Vesting Gateway

Vesting gateway contract is used to find address of vesting contract, where user can claim STT Tokens.

## Localterra deployment workflow
1) From artifacts directory run command to upload package with gateway to the localterra:


    terracli tx wasm store starterra_vesting_gateway.wasm --from test1 --chain-id=localterra --gas=2000000 --fees=1000000000uluna --broadcast-mode=block


2) Validate deployment:


    terracli query wasm code 50

3) Instantiate gateway contract:


    //owner: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
    terracli tx wasm instantiate 50 '{"owner":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8"}' --from test1 --chain-id=localterra --fees=10000000uluna --gas=auto --broadcast-mode=block

4) Query contract:


    terracli query wasm contract terra1vmgzvsr87h5kkh2yy8m004fd70kzynxyuzw2lh

5) Set vesting addresses, where contract will send the query, to check if the user is able to claim:


    terracli tx wasm execute terra1vmgzvsr87h5kkh2yy8m004fd70kzynxyuzw2lh '{"update_vesting_addresses":{"vesting_addresses":["terra1m7p8kpknkaxwujhp9yr2a3cs89g6zun6dgrjzk","terra1v846d3qkdy7dsr5staf499kfdgx3jqx9wfdp75"]}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block

6) Query vesting addresses:


    terracli query wasm contract-store terra1vmgzvsr87h5kkh2yy8m004fd70kzynxyuzw2lh '{"vesting_addresses":{}}'


7) Query to find vesting for the user:


    terracli query wasm contract-store terra1vmgzvsr87h5kkh2yy8m004fd70kzynxyuzw2lh '{"find_vesting_by_user":{"user_address":"terra1ygq7u54agwryd49yh0eshm2mccul8hua8xpwej"}}'

8) Add vesting address to gateway contract:


    terracli tx wasm execute terra1vmgzvsr87h5kkh2yy8m004fd70kzynxyuzw2lh '{"add_vesting_address":{"vesting_address":"terra1qgagqn77atkj59tpyxakgaszdhl4w7n9rylu52"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block

9) Remove vesting address from gateway contract:


    terracli tx wasm execute terra1vmgzvsr87h5kkh2yy8m004fd70kzynxyuzw2lh '{"remove_vesting_address":{"vesting_address":"terra1qgagqn77atkj59tpyxakgaszdhl4w7n9rylu52"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
