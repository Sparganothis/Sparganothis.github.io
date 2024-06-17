set -ex 
git add . && git commit -m 'autofmt before'
( cd client && leptosfmt src )
( cd server &&  cargo fmt )
( cd game  && cargo fmt )
git add . && git commit -m 'autofmt after'