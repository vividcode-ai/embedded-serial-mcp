#!/usr/bin/env bash
set -euo pipefail

dpkg --add-architecture arm64

CODENAME=$(lsb_release -cs)

for f in /etc/apt/sources.list /etc/apt/sources.list.d/*.sources /etc/apt/sources.list.d/*.list; do
    [ -f "$f" ] || continue
    if [[ "$f" == *.sources ]]; then
        grep -q "^URIs:.*ubuntu" "$f" 2>/dev/null && sed -i '/^Types: deb$/a\Architectures: amd64' "$f" || true
    else
        grep -qE "^deb https?://.*ubuntu" "$f" 2>/dev/null && sed -i 's/^deb /deb [arch=amd64] /' "$f" || true
    fi
done

tee /etc/apt/sources.list.d/arm64.list >/dev/null <<EOF
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports ${CODENAME} main restricted universe multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports ${CODENAME}-updates main restricted universe multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports ${CODENAME}-security main restricted universe multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports ${CODENAME}-backports main restricted universe multiverse
EOF

apt-get update || true
apt-get install -y --no-install-recommends \
    libudev-dev:arm64
