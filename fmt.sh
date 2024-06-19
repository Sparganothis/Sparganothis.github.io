set -ex

if [ -z "$(git status --porcelain)" ]; then 
  # Working directory clean
  
    ( cd client ; cargo fmt ; leptosfmt src ; cargo fix )
    ( cd server ;  cargo fmt ; cargo fix  )
    ( cd game  ; cargo fmt   ; cargo fix )
    git add .
    git status
    git commit -m "cargo fix $(date)"
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

