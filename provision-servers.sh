#!/bin/bash

# provision-servers.sh - Interactive server provisioning guide
# This script helps you provision 5 cloud servers for AXIOM mainnet

set -e

echo "╔══════════════════════════════════════════════════════════════════════════╗"
echo "║         AXIOM MAINNET - REAL CLOUD SERVER PROVISIONING GUIDE            ║"
echo "╚══════════════════════════════════════════════════════════════════════════╝"
echo ""
echo "This will guide you through provisioning 5 REAL cloud servers for mainnet."
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if user wants to continue
read -p "Ready to provision real servers? (yes/no): " CONTINUE
if [[ "$CONTINUE" != "yes" ]]; then
    echo "Exiting. Run again when ready."
    exit 0
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "STEP 1: Choose Your Cloud Provider"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Recommended providers and configurations:"
echo ""
echo "1. AWS EC2"
echo "   Instance: t3.large"
echo "   RAM: 8GB, vCPUs: 2"
echo "   Storage: 50GB SSD"
echo "   Cost: ~$60/month per server"
echo "   Regions: us-east-1, eu-west-1, ap-southeast-1, sa-east-1, ap-south-1"
echo ""
echo "2. Google Cloud Compute"
echo "   Instance: n1-standard-2"
echo "   RAM: 7.5GB, vCPUs: 2"
echo "   Storage: 50GB SSD"
echo "   Cost: ~$50/month per server"
echo "   Regions: us-east1, europe-west1, asia-southeast1, southamerica-east1, asia-south1"
echo ""
echo "3. DigitalOcean"
echo "   Droplet: 8GB/4vCPU"
echo "   RAM: 8GB, vCPUs: 4"
echo "   Storage: 50GB SSD"
echo "   Cost: $48/month per server"
echo "   Regions: NYC1, FRA1, SGP1, TOR1, BLR1"
echo ""
echo "4. Linode"
echo "   Instance: 8GB Linode"
echo "   RAM: 8GB, vCPUs: 4"
echo "   Storage: 160GB SSD"
echo "   Cost: $36/month per server"
echo "   Regions: us-east, eu-west, ap-south, ap-southeast, us-west"
echo ""
echo "5. Azure"
echo "   Instance: Standard_B2s"
echo "   RAM: 4GB, vCPUs: 2"
echo "   Storage: 50GB SSD"
echo "   Cost: ~$40/month per server"
echo "   Regions: eastus, westeurope, southeastasia, brazilsouth, centralindia"
echo ""

read -p "Which provider? (aws/gcp/do/linode/azure): " PROVIDER
PROVIDER=$(echo "$PROVIDER" | tr '[:upper:]' '[:lower:]')

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "STEP 2: Automated Provisioning"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

case "$PROVIDER" in
    aws)
        echo "AWS EC2 Provisioning..."
        echo ""
        echo "Run these AWS CLI commands to create 5 servers:"
        echo ""
        cat << 'EOF'
# Install AWS CLI if needed
# curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
# unzip awscliv2.zip
# sudo ./aws/install

# Configure AWS credentials
aws configure

# Create SSH key pair
aws ec2 create-key-pair --key-name axiom-mainnet --query 'KeyMaterial' --output text > axiom-mainnet.pem
chmod 400 axiom-mainnet.pem

# Create security group
aws ec2 create-security-group --group-name axiom-validators --description "AXIOM Validator Security Group"
SECURITY_GROUP=$(aws ec2 describe-security-groups --group-names axiom-validators --query 'SecurityGroups[0].GroupId' --output text)

# Open required ports
aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --protocol tcp --port 22 --cidr 0.0.0.0/0
aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --protocol tcp --port 8545 --cidr 0.0.0.0/0
aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --protocol tcp --port 8546 --cidr 0.0.0.0/0
aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --protocol tcp --port 9100 --cidr 0.0.0.0/0

# Launch 5 instances across regions
REGIONS=("us-east-1" "eu-west-1" "ap-southeast-1" "sa-east-1" "ap-south-1")
AMI_IDS=("ami-0c55b159cbfafe1f0" "ami-0d71ea30463e0ff8d" "ami-0c802847a7dd848c0" "ami-0a0e8b2b5e4d1c8d0" "ami-0e742cca61fb65051")

for i in {0..4}; do
    REGION=${REGIONS[$i]}
    AMI=${AMI_IDS[$i]}
    echo "Launching validator-$((i+1)) in $REGION..."
    
    aws ec2 run-instances \
        --region $REGION \
        --image-id $AMI \
        --instance-type t3.large \
        --key-name axiom-mainnet \
        --security-group-ids $SECURITY_GROUP \
        --block-device-mappings '[{"DeviceName":"/dev/sda1","Ebs":{"VolumeSize":50,"VolumeType":"gp3"}}]' \
        --tag-specifications "ResourceType=instance,Tags=[{Key=Name,Value=axiom-validator-$((i+1))}]" \
        --user-data '#!/bin/bash
apt-get update
apt-get install -y ubuntu-advantage-tools
ua enable livepatch' \
        --output json > validator-$((i+1)).json
done

# Wait for instances to start
echo "Waiting for instances to start..."
sleep 30

# Get instance IPs
echo ""
echo "Collecting server IPs..."
for i in {0..4}; do
    REGION=${REGIONS[$i]}
    IP=$(aws ec2 describe-instances \
        --region $REGION \
        --filters "Name=tag:Name,Values=axiom-validator-$((i+1))" "Name=instance-state-name,Values=running" \
        --query 'Reservations[0].Instances[0].PublicIpAddress' \
        --output text)
    echo "Validator $((i+1)): $IP (root@$IP)"
    echo "root@$IP" >> servers.txt
done

echo ""
echo "✓ All 5 AWS EC2 instances created!"
echo "✓ Server list saved to servers.txt"
echo ""
echo "Next: Wait 2-3 minutes for servers to fully boot, then run:"
echo "  ./launch-mainnet.sh"
EOF
        ;;
    
    gcp)
        echo "Google Cloud Provisioning..."
        echo ""
        echo "Run these gcloud commands to create 5 servers:"
        echo ""
        cat << 'EOF'
# Install gcloud CLI if needed
# curl https://sdk.cloud.google.com | bash
# exec -l $SHELL
# gcloud init

# Set project
gcloud config set project YOUR_PROJECT_ID

# Create firewall rules
gcloud compute firewall-rules create axiom-validators \
    --allow tcp:22,tcp:8545,tcp:8546,tcp:9100 \
    --source-ranges 0.0.0.0/0 \
    --description "AXIOM Validator Ports"

# Create SSH key
ssh-keygen -t rsa -b 2048 -f ~/.ssh/axiom-mainnet -N ""

# Launch 5 instances across regions
REGIONS=("us-east1-b" "europe-west1-b" "asia-southeast1-b" "southamerica-east1-b" "asia-south1-b")

for i in {0..4}; do
    ZONE=${REGIONS[$i]}
    echo "Launching validator-$((i+1)) in $ZONE..."
    
    gcloud compute instances create axiom-validator-$((i+1)) \
        --zone=$ZONE \
        --machine-type=n1-standard-2 \
        --image-family=ubuntu-2204-lts \
        --image-project=ubuntu-os-cloud \
        --boot-disk-size=50GB \
        --boot-disk-type=pd-ssd \
        --metadata=ssh-keys="root:$(cat ~/.ssh/axiom-mainnet.pub)" \
        --tags=axiom-validators
done

# Get instance IPs
echo ""
echo "Collecting server IPs..."
for i in {0..4}; do
    ZONE=${REGIONS[$i]}
    IP=$(gcloud compute instances describe axiom-validator-$((i+1)) \
        --zone=$ZONE \
        --format='get(networkInterfaces[0].accessConfigs[0].natIP)')
    echo "Validator $((i+1)): $IP (root@$IP)"
    echo "root@$IP" >> servers.txt
done

echo ""
echo "✓ All 5 GCP instances created!"
echo "✓ Server list saved to servers.txt"
echo ""
echo "Next: Wait 2-3 minutes for servers to fully boot, then run:"
echo "  ./launch-mainnet.sh"
EOF
        ;;
    
    do)
        echo "DigitalOcean Provisioning..."
        echo ""
        echo "Run these doctl commands to create 5 servers:"
        echo ""
        cat << 'EOF'
# Install doctl CLI
# wget https://github.com/digitalocean/doctl/releases/download/v1.94.0/doctl-1.94.0-linux-amd64.tar.gz
# tar xf doctl-1.94.0-linux-amd64.tar.gz
# sudo mv doctl /usr/local/bin

# Authenticate
doctl auth init

# Create SSH key
ssh-keygen -t rsa -b 2048 -f ~/.ssh/axiom-mainnet -N ""
SSH_KEY_ID=$(doctl compute ssh-key create axiom-mainnet --public-key "$(cat ~/.ssh/axiom-mainnet.pub)" --format ID --no-header)

# Launch 5 droplets across regions
REGIONS=("nyc1" "fra1" "sgp1" "tor1" "blr1")

for i in {0..4}; do
    REGION=${REGIONS[$i]}
    echo "Launching validator-$((i+1)) in $REGION..."
    
    doctl compute droplet create axiom-validator-$((i+1)) \
        --region $REGION \
        --size s-4vcpu-8gb \
        --image ubuntu-22-04-x64 \
        --ssh-keys $SSH_KEY_ID \
        --enable-monitoring \
        --enable-ipv6 \
        --tag-name axiom-validators \
        --wait
done

# Get droplet IPs
echo ""
echo "Collecting server IPs..."
for i in {0..4}; do
    IP=$(doctl compute droplet list axiom-validator-$((i+1)) --format PublicIPv4 --no-header)
    echo "Validator $((i+1)): $IP (root@$IP)"
    echo "root@$IP" >> servers.txt
done

echo ""
echo "✓ All 5 DigitalOcean droplets created!"
echo "✓ Server list saved to servers.txt"
echo ""
echo "Next: Run immediately:"
echo "  ./launch-mainnet.sh"
EOF
        ;;
    
    linode)
        echo "Linode Provisioning..."
        echo ""
        echo "Run these linode-cli commands to create 5 servers:"
        echo ""
        cat << 'EOF'
# Install linode-cli
# pip3 install linode-cli
# linode-cli configure

# Create SSH key
ssh-keygen -t rsa -b 2048 -f ~/.ssh/axiom-mainnet -N ""
SSH_KEY=$(cat ~/.ssh/axiom-mainnet.pub)

# Launch 5 instances across regions
REGIONS=("us-east" "eu-west" "ap-south" "ap-southeast" "us-west")

for i in {0..4}; do
    REGION=${REGIONS[$i]}
    echo "Launching validator-$((i+1)) in $REGION..."
    
    linode-cli linodes create \
        --label axiom-validator-$((i+1)) \
        --region $REGION \
        --type g6-standard-2 \
        --image linode/ubuntu22.04 \
        --root_pass "$(openssl rand -base64 32)" \
        --authorized_keys "$SSH_KEY" \
        --tags axiom-validators
done

# Wait for instances to boot
echo "Waiting for instances to start..."
sleep 45

# Get instance IPs
echo ""
echo "Collecting server IPs..."
for i in {0..4}; do
    IP=$(linode-cli linodes list --label axiom-validator-$((i+1)) --json | jq -r '.[0].ipv4[0]')
    echo "Validator $((i+1)): $IP (root@$IP)"
    echo "root@$IP" >> servers.txt
done

echo ""
echo "✓ All 5 Linode instances created!"
echo "✓ Server list saved to servers.txt"
echo ""
echo "Next: Run immediately:"
echo "  ./launch-mainnet.sh"
EOF
        ;;
    
    azure)
        echo "Azure Provisioning..."
        echo ""
        echo "Run these Azure CLI commands to create 5 servers:"
        echo ""
        cat << 'EOF'
# Install Azure CLI if needed
# curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

# Login to Azure
az login

# Create resource group
az group create --name axiom-mainnet --location eastus

# Create SSH key
ssh-keygen -t rsa -b 2048 -f ~/.ssh/axiom-mainnet -N ""

# Create network security group
az network nsg create --resource-group axiom-mainnet --name axiom-nsg

# Add security rules
az network nsg rule create --resource-group axiom-mainnet --nsg-name axiom-nsg \
    --name SSH --priority 1000 --destination-port-ranges 22 --access Allow --protocol Tcp
az network nsg rule create --resource-group axiom-mainnet --nsg-name axiom-nsg \
    --name P2P --priority 1001 --destination-port-ranges 8545 --access Allow --protocol Tcp
az network nsg rule create --resource-group axiom-mainnet --nsg-name axiom-nsg \
    --name RPC --priority 1002 --destination-port-ranges 8546 --access Allow --protocol Tcp
az network nsg rule create --resource-group axiom-mainnet --nsg-name axiom-nsg \
    --name Metrics --priority 1003 --destination-port-ranges 9100 --access Allow --protocol Tcp

# Launch 5 VMs across regions
LOCATIONS=("eastus" "westeurope" "southeastasia" "brazilsouth" "centralindia")

for i in {0..4}; do
    LOCATION=${LOCATIONS[$i]}
    echo "Launching validator-$((i+1)) in $LOCATION..."
    
    az vm create \
        --resource-group axiom-mainnet \
        --name axiom-validator-$((i+1)) \
        --location $LOCATION \
        --image Ubuntu2204 \
        --size Standard_B2s \
        --admin-username azureuser \
        --ssh-key-values ~/.ssh/axiom-mainnet.pub \
        --nsg axiom-nsg \
        --os-disk-size-gb 50 \
        --tags Project=AXIOM Role=Validator
done

# Get VM IPs
echo ""
echo "Collecting server IPs..."
for i in {0..4}; do
    IP=$(az vm list-ip-addresses --resource-group axiom-mainnet --name axiom-validator-$((i+1)) \
        --query '[0].virtualMachine.network.publicIpAddresses[0].ipAddress' --output tsv)
    echo "Validator $((i+1)): $IP (azureuser@$IP)"
    echo "azureuser@$IP" >> servers.txt
done

echo ""
echo "✓ All 5 Azure VMs created!"
echo "✓ Server list saved to servers.txt"
echo ""
echo "Next: Wait 2-3 minutes for VMs to fully boot, then run:"
echo "  ./launch-mainnet.sh"
EOF
        ;;
    
    *)
        echo "❌ Invalid provider. Choose: aws, gcp, do, linode, or azure"
        exit 1
        ;;
esac

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "STEP 3: Manual Provisioning (Alternative)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "If you prefer to create servers manually through web console:"
echo ""
echo "1. Create 5 servers with these specs:"
echo "   - OS: Ubuntu 22.04 LTS"
echo "   - RAM: 8GB minimum"
echo "   - Storage: 50GB SSD minimum"
echo "   - vCPUs: 2-4"
echo ""
echo "2. Configure firewall to allow these ports:"
echo "   - 22 (SSH)"
echo "   - 8545 (P2P)"
echo "   - 8546 (RPC)"
echo "   - 9100 (Metrics)"
echo ""
echo "3. Set up SSH key authentication"
echo ""
echo "4. Save server IPs to servers.txt (one per line):"
echo "   echo 'root@<IP1>' >> servers.txt"
echo "   echo 'root@<IP2>' >> servers.txt"
echo "   echo 'root@<IP3>' >> servers.txt"
echo "   echo 'root@<IP4>' >> servers.txt"
echo "   echo 'root@<IP5>' >> servers.txt"
echo ""
echo "5. Run: ./launch-mainnet.sh"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "💡 TIP: Test SSH connectivity before launching:"
echo "    ssh -i ~/.ssh/axiom-mainnet root@<SERVER_IP> 'echo OK'"
echo ""
echo "📊 INFRASTRUCTURE:"
echo "   Total: ~\$234/month for 5 servers"
echo "   Block rewards: 175,200 AXM/year per validator (protocol emission)"
echo "   Mathematical scarcity: Fixed 124M supply cap, provable on-chain"
echo ""
echo "✨ After provisioning, run: ./launch-mainnet.sh"
echo ""
