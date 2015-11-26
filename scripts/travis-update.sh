#!/bin/sh

# Exit if anything fails
set -e

[ "$TRAVIS_BRANCH" = master ]

[ "$TRAVIS_PULL_REQUEST" = false ]

eval SSH_KEY_TRAVIS_ID=a2e63a976778
eval key=\$encrypted_${SSH_KEY_TRAVIS_ID}_key
eval iv=\$encrypted_${SSH_KEY_TRAVIS_ID}_iv

mkdir -p ~/.ssh
openssl aes-256-cbc -K $key -iv $iv -in scripts/travis-float-free-libcore.enc -out ~/.ssh/id_rsa -d
chmod 600 ~/.ssh/id_rsa

git clone --depth=1 https://github.com/rust-lang/rust.git

cd rust
commit_hash=$(git rev-parse HEAD)
cd ..

git clone git@github.com:phil-opp/float-free-libcore.git

cd float-free-libcore
rm -r src
cp -r ../rust/src/libcore libcore
cp -r libcore libcore_orig
# Make floats optional
patch -p0 < ../libcore_nofp.patch

rm -r libcore_orig
mv libcore src
# Remove stability attributes
find src/ -name "*.rs" | xargs perl -0pe 's/#(!)?\[((un)?stable|rustc_deprecated)\([\S\s]*?(?<=\))\]//g' -i

# try to build it
cargo build
cargo build --features="disable_float"

git config user.name "travis-update-bot"
git config user.email "nobody@example.com"
git config --global push.default simple

git add src
git commit -m "Update to $commit_hash"
git push

cd ../
rm -rf rust
rm -rf float-free-libcore
rm libcore_nofp.patch