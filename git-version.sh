BRANCH_NAME="$(git rev-parse --abbrev-ref HEAD)"
COMMIT_HASH="$(git rev-parse --short HEAD)"

UNTRACKED="$(git ls-files --exclude-standard --others)"
DIFF_INDEX="$(git diff-index  HEAD) $(git diff-index --cached HEAD)"
UNTRACKED_MD5="$(echo "$DIFF_INDEX" "$UNTRACKED" | md5sum | cut -d' ' -f1 )"
UNTRACKED_LEN="$(echo "$DIFF_INDEX" "$UNTRACKED" | wc -c)"

echo "$BRANCH_NAME-$COMMIT_HASH-$UNTRACKED_MD5-$UNTRACKED_LEN"