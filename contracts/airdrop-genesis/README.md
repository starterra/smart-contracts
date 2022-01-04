# Genesis StarTerra Token Airdrop

The Genesis Airdrop contract is for airdropping Starterra Tokens to Luna stakers.
The StarTerra team will register Merkle Root with addresses from Luna stakers
snapshot. Luna stakers can use Merkle proofs to take airdropped StarTerra Tokens.


## Localterra deployment workflow
1) Requirement: StarTerra Token already instantiated on localterra and we have compiled contracts.

2) From artifacts directory run command to upload package to the localterra:


    terracli tx wasm store starterra_airdrop_genesis.wasm --from test1 --chain-id=localterra --gas=2000000 --fees=1000000000uluna --broadcast-mode=block

3) Validate deployment:


    terracli query wasm code 2

4) Instantiate Genesis Airdrop contract:


    //owner: terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8
    //starterra_token: terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5
    terracli tx wasm instantiate 2 '{"owner":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8","starterra_token":"terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5"}' --from test1 --chain-id=localterra --fees=10000000uluna --gas=auto --broadcast-mode=block

5) Query contract:


    terracli query wasm contract terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4

6) Set merkle_root:


    terracli tx wasm execute terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4 '{"register_merkle_root":{"merkle_root":"23b3a57e29ff443405f0785a48c4c2849ea94dd29ffc7a56b31c21b3dd1f8e80"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block


7) Query merkle_root:


    terracli query wasm contract-store terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4 '{"merkle_root":{}}'


8) Send tokens to Genesis Airdrop address:


    terracli tx wasm execute terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"transfer":{"amount":"1000","recipient":"terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4"}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block

9) Query balance:


    terracli query wasm contract-store terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"balance":{"address":"terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4"}}'

10) Add to terracli second address:


    terracli keys add test2 --recover

    //use mnemonic:
    symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb


11) Claim StarTerra tokens


    terracli tx wasm execute terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4 '{"claim":{"amount":"100","proof":["89c3ed50ec60bbdd53179f493c11206da35fc5fd12efded36285b5cdbcd72007","4639a30e22bcf7e0fcf2730cbba5fcdf3a741b07d437880be4bc4e3a7221649f","5da28caf6b7c08dc909554dcb1b5a30fcebeda611ddf318d52b880680b74c067","ed538386b429072307c1f32b96637503e0cee5eda88cade123d89c3d3a865f01"]}}' --from test2 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block


12) Query balance:


    //Genesis airdrop:
    terracli query wasm contract-store terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"balance":{"address":"terra10pyejy66429refv3g35g2t7am0was7ya7kz2a4"}}'
    //User balance:
    terracli query wasm contract-store terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"balance":{"address":"terra1757tkx08n0cqrw7p86ny9lnxsqeth0wgp0em95"}}'

13) End genesis airdrop:


    terracli tx wasm execute terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"end_airdrop_genesis":{}}' --from test1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
