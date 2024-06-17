set -ex 
( cd client && cargo fmt && leptosfmt src && cargo fix --allow-dirty )
( cd server &&  cargo fmt && cargo fix --allow-dirty )
( cd game  && cargo fmt && cargo fix --allow-dirty )