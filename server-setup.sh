#!/bin/bash

# Usage: ./server-setup.sh example.com 8080 example@example.com

DOMAIN=$1
PORT=$2
EMAIL=$3

if [ -z "$DOMAIN" ] || [ -z "$PORT" ]; then
  echo "Usage: $0 domain port"
  exit 1
fi

NGINX_AVAILABLE="/etc/nginx/sites-available/$DOMAIN"
NGINX_ENABLED="/etc/nginx/sites-enabled/$DOMAIN"

# Create initial nginx config (HTTP only)
cat >$NGINX_AVAILABLE <<EOF
server {
    server_name $DOMAIN www.$DOMAIN;

    location / {
        proxy_pass http://127.0.0.1:$PORT;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;

        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;

        proxy_cache_bypass \$http_upgrade;
    }

    listen 80;
}
EOF

# Enable site
ln -s $NGINX_AVAILABLE $NGINX_ENABLED 2>/dev/null

# Test nginx config
nginx -t || {
  echo "Nginx config test failed"
  exit 1
}

# Reload nginx
systemctl reload nginx

# Run certbot to automatically add SSL
certbot --nginx -d $DOMAIN -d www.$DOMAIN --non-interactive --agree-tos -m $EMAIL

# Add http2 to SSL block (right after "listen 443 ssl;")
sed -i '/listen 443 ssl;/a \    http2 on;' $NGINX_AVAILABLE

# Test again
nginx -t || {
  echo "Nginx config test failed after http2 injection"
  exit 1
}

# Reload nginx
systemctl reload nginx

echo "✅ Site $DOMAIN created, secured with SSL, and HTTP/2 enabled on port $PORT."
