set -ex

if [ -z "$(git status --porcelain)" ]; then 
  # Working directory clean
  
    ( cd client ;  cargo fmt ; leptosfmt src  )
    ( cd server ;  cargo fmt  )
    ( cd game  ;   cargo fmt   )
    git add .
    git status
    git commit -m "cargo fmt $(date)" || true
    git push

    ( cd client ;  cargo fix || true )
    ( cd server ;   cargo fix || true   )
    ( cd game  ;   cargo fix || true   )
    
    git add .
    git status
    git commit -m "cargo fix $(date)" || true
    git push

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

