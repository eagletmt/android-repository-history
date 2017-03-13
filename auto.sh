#!/bin/bash
set -ex

git checkout -- repository
go run update.go
git add repository
if git commit -m "Update repository $(date --rfc-3339=seconds)"; then
  git push origin master
fi
