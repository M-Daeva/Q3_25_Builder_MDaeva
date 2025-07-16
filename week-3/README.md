### Description

A custom boilerplate to create new anchor project quickly and easy


### Installation

```sh
git clone https://github.com/M-Daeva/solana-boilerplate.git app
```
```sh
cd app
```
```sh
rm -rf .git && bun install
```

### Workflow

1. Generate program ID and update it in files
```sh
clear && ./program_id.sh staking
```

2. Build programs
```sh
clear && ./build.sh
```

3. Test programs (stop local validator if required)
```sh
clear && bun test-local
```

4. Deploy programs (run local validator if required)
```sh
solana-test-validator
clear && ./deploy.sh localnet
```

5. Initialize programs
```sh
clear && bun initialize localnet
```

6. Interact with programs
```sh
clear && bun start localnet
```
