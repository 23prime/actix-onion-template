#!/usr/bin/env bash
# Seed data for local development.
set -euo pipefail

BASE_URL="${BASE_URL:-http://localhost:8080}"

create_user() {
  local name="$1" email="$2" password="$3"
  local status
  status=$(curl -s -o /dev/null -w "%{http_code}" -X POST "${BASE_URL}/users" \
    -H "Content-Type: application/json" \
    -d "{\"name\":\"${name}\",\"email\":\"${email}\",\"password\":\"${password}\"}")
  case "${status}" in
    201) echo "  created: ${email}" ;;
    409) echo "  skipped (already exists): ${email}" ;;
    *)   echo "  error (HTTP ${status}): ${email}"; exit 1 ;;
  esac
}

echo "Seeding local database..."
create_user "Admin" "admin@example.com" "admin"
echo "Done."
