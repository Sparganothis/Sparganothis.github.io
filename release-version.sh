#!/bin/bash -ex

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

export NEW_VERSION="$1"
if [ "$NEW_VERSION" == "" ]; then 
    echo "no version given!"
    exit 1
fi

(
    cd client
    cargo set-version $NEW_VERSION
)
(
    cd server
    cargo set-version $NEW_VERSION
)
(
    cd game
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