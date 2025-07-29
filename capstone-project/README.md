### Description

A decentralized password manager that combines self-custody security with blockchain-based data persistence, where users maintain complete control over their encrypted data while optionally leveraging on-chain storage for enhanced reliability and accessibility


### Diagrams

#### Architecture Diagram
![Architecture Diagram](./diagrams/Password_Manager_Architecture_Diagram.drawio.svg)

#### Create and Activate Account with SOL Flowchart
![Create and Activate Account with SOL Flowchart](./diagrams/Password_Manager_Flow_Create_Account_SOL.drawio.svg)

#### Create and Activate Account with WBTC Flowchart
![Create and Activate Account with WBTC Flowchart](./diagrams/Password_Manager_Flow_Create_Account_WBTC.drawio.svg)

#### Update Data in User Account Flowchart
![Update Data in User Account Flowchart](./diagrams/Password_Manager_Flow_Update_Data.drawio.svg)

#### Rotate Account Flowchart
![Rotate Account Flowchart](./diagrams/Password_Manager_Flow_Rotate_Account.drawio.svg)


### Workflow

1. Generate program ID and update it in files
```sh
clear && ./program_id.sh registry
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