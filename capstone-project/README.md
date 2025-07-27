### Description

### Workflows

a) User has 11$ as SOL
1. Client calls Registry::create_account(1$ SOL)
2. Client wraps 10$ SOL to WSOL
3. Client calls DEXAdapter::swap_and_forward(10$ WSOL → USDC, Registry)
4. DEX Adapter swaps and forwards USDC to Registry
5. Registry automatically activates account upon USDC receipt

b) User has 11$ as WBTC
1. User calls DEXAdapter::multi_swap(11$ WBTC → 1$ WSOL + 10$ USDC)
2. DEX Adapter performs two swaps:
    1/11 * WBTC → WSOL
    10/11 * WBTC → USDC
3. DEX Adapter sends 10$ USDC to user
4. DEX Adapter unwraps 1$ WSOL to SOL sending it to user directly
5. Client calls Registry::create_account(1$ SOL)
6. Client calls Registry::activate_account(10$ USDC)

