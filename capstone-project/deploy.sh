# The script to deploy programs
# clear && ./deploy.sh devnet counter
# clear && ./deploy.sh devnet

# Check if cluster parameter (localnet | devnet) is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <cluster> [program_name]"
  echo "  cluster: localnet | devnet | mainnet"
  echo "  program_name (optional): specific program to deploy"
  exit 1
fi

CLUSTER="$1"
PROGRAM="$2"

# Build first
echo "Building programs..."
anchor build

# Deploy specific program or all programs
if [ -n "$PROGRAM" ]; then
  echo "Deploying $PROGRAM to $CLUSTER..."
  anchor deploy --provider.cluster "$CLUSTER" --program-name "$PROGRAM"
else
  echo "Deploying all programs to $CLUSTER..."
  anchor deploy --provider.cluster "$CLUSTER"
fi