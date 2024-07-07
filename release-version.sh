#!/bin/bash
set -ex

if [ -z "$(git status --porcelain)" ]; then 
    echo "git status OK."
else 
    git status
    set +x
    echo
    echo "!!!"
    echo
    echo "WORKING DIRECTORY NOT CLEAN"
    echo "PLZ COMMIT CHANGES"
    exit 66
fi

(
    cd game
    cargo set-version --bump patch
)
export NEW_VERSION="$(cat game/Cargo.toml | grep "^version = .*$" | cut -f3 -d' ' | cut -f2 -d'"' | head -n1)"

if [ "$NEW_VERSION" == "" ]; then 
    echo "no version given!"
    exit 1
fi
echo "$NEW_VERSION" > VERSION
(
    cd client
    cargo set-version $NEW_VERSION
)
(
    cd server
    cargo set-version $NEW_VERSION
)
(
    cd sparganothis_vim
    cargo set-version $NEW_VERSION
)
git add .
git commit -m "bump version: $NEW_VERSION"
git tag -a $NEW_VERSION -m "release $NEW_VERSION"

git push
git push origin $NEW_VERSION