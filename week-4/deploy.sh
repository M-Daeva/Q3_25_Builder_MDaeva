# The script to deploy programs

# Check if cluster parameter (localnet | devnet) is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <cluster>"
  exit 1
fi

CLUSTER="$1"

anchor deploy --provider.cluster "$CLUSTER"