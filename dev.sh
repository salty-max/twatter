#!/usr/bin/env zsh
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd client; trunk serve --proxy-backend=http://[::1]:1337/api/v1' & \
 bash -c 'cargo watch -- cargo run --bin api')
