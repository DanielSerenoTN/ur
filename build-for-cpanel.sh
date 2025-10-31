#!/bin/bash

echo "Building URVIC for cPanel deployment..."

# Create deployment folder
echo "Creating deployment package..."
rm -rf cpanel-deploy
mkdir -p cpanel-deploy/public_html
mkdir -p cpanel-deploy/api

# Build frontend
if [ -d "frontend_urvic" ]; then
    echo "Building React frontend..."
    cd frontend_urvic
    npm ci
    npm run build
    cp -r build/* ../cpanel-deploy/public_html/
    cd ..
else
    echo "Warning: frontend_urvic directory not found, skipping frontend build"
fi

# Build backend for Linux
echo "Building Rust backend for Linux..."
if [ -f "Cargo.toml" ]; then
    # Install cross-compiler if not installed
    rustup target add x86_64-unknown-linux-gnu
    
    # Build for Linux
    cargo build --release --target x86_64-unknown-linux-gnu
    
    # Copy backend binary
    if [ -f "target/x86_64-unknown-linux-gnu/release/urvic-backend" ]; then
        mkdir -p cpanel-deploy/api
        cp target/x86_64-unknown-linux-gnu/release/urvic-backend cpanel-deploy/api/
    else
        echo "Error: Failed to build Rust backend"
        exit 1
    fi
else
    echo "Error: Cargo.toml not found. Are you in the correct directory?"
    exit 1
fi

# Copy configuration files
if [ -f ".env.example" ]; then
    cp .env.example cpanel-deploy/api/.env
    echo "Copied .env.example to cpanel-deploy/api/.env"
else
    echo "Warning: .env.example not found, you'll need to create .env manually"
fi

if [ -f "diesel.toml" ]; then
    cp diesel.toml cpanel-deploy/api/
    echo "Copied diesel.toml to cpanel-deploy/api/"
else
    echo "Warning: diesel.toml not found"
fi

# Create a simple .htaccess for cPanel
echo "Creating .htaccess for cPanel..."
cat > cpanel-deploy/public_html/.htaccess << 'EOL'
<IfModule mod_rewrite.c>
  RewriteEngine On
  RewriteBase /
  RewriteRule ^index\.html$ - [L]
  RewriteCond %{REQUEST_FILENAME} !-f
  RewriteCond %{REQUEST_FILENAME} !-d
  RewriteRule . /index.html [L]
</IfModule>
EOL

echo ""
echo "Deployment package ready in cpanel-deploy/"
echo "1. Upload cpanel-deploy/public_html/* to your cPanel public_html"
echo "2. Upload cpanel-deploy/api/* to a private folder in cPanel (outside public_html)"
echo "3. Make sure to set proper permissions on the API binary: chmod +x /path/to/urvic-backend"
