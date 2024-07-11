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

export BRANCH_NAME="$(git rev-parse --abbrev-ref HEAD)"

if [ $BRANCH_NAME == "master" ] || [ $BRANCH_NAME == "bos" ]; then 
    echo "branch ok"
else
    echo "plz use master or bos brancch"
    exit 1
fi

for TESTFILE in test.sh test-pypi.sh; do
    if bash $TESTFILE; then
        echo $TESTFILE OK
    else
        echo $TESTFILE FAIL PLZ FIX BEFORE RELEASE VERSION
        exit 1
    fi
done

(
    cd game
    cargo set-version --bump patch
)
export NEW_VERSION="$(cat game/Cargo.toml | grep "^version = .*$" | cut -f3 -d' ' | cut -f2 -d'"' | head -n1 | tr -d '\n' | tr -d ' ')"

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