#!/bin/sh
set -e

# Ensure DB exists
touch /app/db.sqlite

echo "Starting Rocket Backend..."
# Run backend in background and log outputs directly to container logs
/app/backend_bin 2>&1 &

# Wait briefly to confirm process hasn't immediately crashed
sleep 2
if ! kill -0 $! 2>/dev/null; then
    echo "ERROR: Backend binary failed to start or crashed!"
    exit 1
fi

echo "Starting Nginx..."
exec nginx -g "daemon off;"