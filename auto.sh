#!/bin/bash
set -ex

git clone git@github.com:eagletmt/android-repository-history /android-repository-history
cd /android-repository-history
/root/update
git add repository
if git commit -m "Update repository $(date --rfc-3339=seconds)"; then
  git push origin master
fi
