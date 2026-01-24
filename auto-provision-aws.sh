#!/bin/bash

# auto-provision-aws.sh - Automated AWS EC2 provisioning for AXIOM mainnet
# This script automatically creates 5 EC2 instances across global regions

set -e

echo "╔══════════════════════════════════════════════════════════════════════════╗"
echo "║         AXIOM MAINNET - AUTOMATED AWS EC2 PROVISIONING                  ║"
echo "╚══════════════════════════════════════════════════════════════════════════╝"
echo ""

# Check if AWS CLI is installed
if ! command -v aws &> /dev/null; then
    echo "❌ AWS CLI not found. Installing..."
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
    unzip -q awscliv2.zip
    sudo ./aws/install
    rm -rf aws awscliv2.zip
    echo "✓ AWS CLI installed"
fi

# Check if AWS is configured
if ! aws sts get-caller-identity &> /dev/null; then
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "⚙️  AWS Configuration Required"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "You need to configure AWS credentials. You'll need:"
    echo "  1. AWS Access Key ID"
    echo "  2. AWS Secret Access Key"
    echo "  3. Default region (use: us-east-1)"
    echo ""
    echo "Get these from: https://console.aws.amazon.com/iam/home#/security_credentials"
    echo ""
    aws configure
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 1: Creating SSH Key Pair"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if key already exists
if [ -f "axiom-mainnet.pem" ]; then
    echo "⚠️  axiom-mainnet.pem already exists"
    read -p "Delete and recreate? (yes/no): " RECREATE
    if [[ "$RECREATE" == "yes" ]]; then
        aws ec2 delete-key-pair --key-name axiom-mainnet --region us-east-1 2>/dev/null || true
        rm -f axiom-mainnet.pem
    else
        echo "Using existing key pair"
    fi
fi

if [ ! -f "axiom-mainnet.pem" ]; then
    echo "Creating SSH key pair..."
    aws ec2 create-key-pair --key-name axiom-mainnet --region us-east-1 \
        --query 'KeyMaterial' --output text > axiom-mainnet.pem
    chmod 400 axiom-mainnet.pem
    echo "✓ SSH key created: axiom-mainnet.pem"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 2: Creating Security Group"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Create security group in us-east-1
SECURITY_GROUP_EXISTS=$(aws ec2 describe-security-groups --group-names axiom-validators --region us-east-1 2>/dev/null || echo "")

if [ -z "$SECURITY_GROUP_EXISTS" ]; then
    echo "Creating security group..."
    aws ec2 create-security-group \
        --group-name axiom-validators \
        --description "AXIOM Validator Security Group" \
        --region us-east-1
    
    SECURITY_GROUP=$(aws ec2 describe-security-groups \
        --group-names axiom-validators \
        --region us-east-1 \
        --query 'SecurityGroups[0].GroupId' \
        --output text)
    
    echo "Opening ports..."
    aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --region us-east-1 \
        --protocol tcp --port 22 --cidr 0.0.0.0/0 2>/dev/null || true
    aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --region us-east-1 \
        --protocol tcp --port 8545 --cidr 0.0.0.0/0 2>/dev/null || true
    aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --region us-east-1 \
        --protocol tcp --port 8546 --cidr 0.0.0.0/0 2>/dev/null || true
    aws ec2 authorize-security-group-ingress --group-id $SECURITY_GROUP --region us-east-1 \
        --protocol tcp --port 9100 --cidr 0.0.0.0/0 2>/dev/null || true
    
    echo "✓ Security group created: $SECURITY_GROUP"
else
    SECURITY_GROUP=$(aws ec2 describe-security-groups \
        --group-names axiom-validators \
        --region us-east-1 \
        --query 'SecurityGroups[0].GroupId' \
        --output text)
    echo "✓ Using existing security group: $SECURITY_GROUP"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 3: Launching 5 EC2 Instances Across Global Regions"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Define regions and their AMIs (Ubuntu 22.04 LTS)
declare -A REGION_AMIS
REGION_AMIS["us-east-1"]="ami-0866a3c8686eaeeba"      # US East (N. Virginia)
REGION_AMIS["eu-west-1"]="ami-0d64bb532e0502c46"      # EU (Ireland)
REGION_AMIS["ap-southeast-1"]="ami-0497a974f8d5dcef8" # Asia Pacific (Singapore)
REGION_AMIS["sa-east-1"]="ami-0c820c196a818d66a"      # South America (São Paulo)
REGION_AMIS["ap-south-1"]="ami-053b12d3152c0cc71"     # Asia Pacific (Mumbai)

REGIONS=("us-east-1" "eu-west-1" "ap-southeast-1" "sa-east-1" "ap-south-1")
INSTANCE_IDS=()

# Clean up old servers.txt
rm -f servers.txt

for i in {0..4}; do
    REGION=${REGIONS[$i]}
    AMI=${REGION_AMIS[$REGION]}
    
    echo "[$((i+1))/5] Launching validator-$((i+1)) in $REGION..."
    
    # Create security group in this region if needed
    if [ "$REGION" != "us-east-1" ]; then
        aws ec2 describe-security-groups --group-names axiom-validators --region $REGION &>/dev/null || {
            echo "  Creating security group in $REGION..."
            aws ec2 create-security-group \
                --group-name axiom-validators \
                --description "AXIOM Validator Security Group" \
                --region $REGION &>/dev/null
            
            REGIONAL_SG=$(aws ec2 describe-security-groups \
                --group-names axiom-validators \
                --region $REGION \
                --query 'SecurityGroups[0].GroupId' \
                --output text)
            
            aws ec2 authorize-security-group-ingress --group-id $REGIONAL_SG --region $REGION \
                --protocol tcp --port 22 --cidr 0.0.0.0/0 &>/dev/null || true
            aws ec2 authorize-security-group-ingress --group-id $REGIONAL_SG --region $REGION \
                --protocol tcp --port 8545 --cidr 0.0.0.0/0 &>/dev/null || true
            aws ec2 authorize-security-group-ingress --group-id $REGIONAL_SG --region $REGION \
                --protocol tcp --port 8546 --cidr 0.0.0.0/0 &>/dev/null || true
            aws ec2 authorize-security-group-ingress --group-id $REGIONAL_SG --region $REGION \
                --protocol tcp --port 9100 --cidr 0.0.0.0/0 &>/dev/null || true
        }
    fi
    
    # Copy SSH key to this region
    aws ec2 import-key-pair --key-name axiom-mainnet --region $REGION \
        --public-key-material fileb://<(ssh-keygen -y -f axiom-mainnet.pem) &>/dev/null || true
    
    # Launch instance
    INSTANCE_JSON=$(aws ec2 run-instances \
        --region $REGION \
        --image-id $AMI \
        --instance-type t3.large \
        --key-name axiom-mainnet \
        --security-groups axiom-validators \
        --block-device-mappings '[{"DeviceName":"/dev/sda1","Ebs":{"VolumeSize":50,"VolumeType":"gp3"}}]' \
        --tag-specifications "ResourceType=instance,Tags=[{Key=Name,Value=axiom-validator-$((i+1))},{Key=Project,Value=AXIOM},{Key=Role,Value=Validator}]" \
        --output json)
    
    INSTANCE_ID=$(echo "$INSTANCE_JSON" | jq -r '.Instances[0].InstanceId')
    INSTANCE_IDS+=("$INSTANCE_ID:$REGION")
    
    echo "  ✓ Instance created: $INSTANCE_ID"
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 4: Waiting for Instances to Start"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Waiting 60 seconds for instances to initialize..."
sleep 60

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 5: Collecting Instance IPs"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

counter=1
for instance_info in "${INSTANCE_IDS[@]}"; do
    INSTANCE_ID=$(echo "$instance_info" | cut -d: -f1)
    REGION=$(echo "$instance_info" | cut -d: -f2)
    
    echo "[$counter/5] Getting IP for $INSTANCE_ID in $REGION..."
    
    # Wait for instance to be running
    aws ec2 wait instance-running --instance-ids $INSTANCE_ID --region $REGION
    
    # Get public IP
    IP=$(aws ec2 describe-instances \
        --instance-ids $INSTANCE_ID \
        --region $REGION \
        --query 'Reservations[0].Instances[0].PublicIpAddress' \
        --output text)
    
    echo "  ✓ Validator $counter: $IP (ubuntu@$IP)"
    echo "ubuntu@$IP" >> servers.txt
    
    counter=$((counter+1))
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ AWS EC2 PROVISIONING COMPLETE!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Server Information:"
echo ""
cat servers.txt
echo ""
echo "📁 Files Created:"
echo "  ✓ axiom-mainnet.pem - SSH private key"
echo "  ✓ servers.txt - List of server IPs"
echo ""
echo "🔑 SSH Key Location: ./axiom-mainnet.pem"
echo ""
echo "✅ Test SSH Connection:"
echo "   ssh -i axiom-mainnet.pem ubuntu@\$(head -1 servers.txt | cut -d@ -f2) 'echo OK'"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 NEXT STEP: Launch AXIOM Mainnet"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Wait 2-3 minutes for servers to fully boot, then run:"
echo ""
echo "  ./launch-mainnet.sh"
echo ""
echo "This will:"
echo "  1. SSH to each server"
echo "  2. Install dependencies (Rust, build tools)"
echo "  3. Build AXIOM validator binary"
echo "  4. Generate genesis block"
echo "  5. Start all validators"
echo "  6. Launch the mainnet!"
echo ""
echo "💰 Monthly Cost: ~\$300 (5 × \$60/month)"
echo "📊 Protocol Emission: 875,000 AXM/year (fixed block rewards)"
echo "🔒 Verifiable Economics: Provable supply cap, transparent emission schedule"
echo ""
