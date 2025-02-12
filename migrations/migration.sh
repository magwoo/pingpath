#!/bin/sh

RQLITE_URL="http://localhost:4001"
MIGRATION_FILE="migration.sql"

docker-entrypoint.sh run &

until curl --silent --fail --output /dev/null "$RQLITE_URL/readyz"; do
    echo "Wait until rqlite setup..."
    sleep 1
done
echo "rqlite started!"

echo "Migration..."
curl -XPOST 'localhost:4001/db/execute' \
    -H "Content-Type: text/plain" \
    --data-binary @"$MIGRATION_FILE" \
    --output /dev/null
echo "Migration success!"

wait
