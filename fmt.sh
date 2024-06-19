set -ex 
( cd client && cargo fmt && leptosfmt src )
( cd server &&  cargo fmt  )
( cd game  && cargo fmt  )