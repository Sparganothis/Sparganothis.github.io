set -ex 
( cd client && leptosfmt src )
( cd server &&  cargo fmt )
( cd game  && cargo fmt )