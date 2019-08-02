#
#	You need to have an SSH key on the app. Or password
# 	TEST APP PORT 15928
# 	PRODUCTION APP PORT 15928
#

BASEDIR=$(shell pwd)
DESTINATIONMACHINE=node-11.rosti.cz
DESTINATIONPORT=15928
DESTINATIONDIR=/srv/app/
DESTINATIONUSER=app
NAME=lunchtime
VERSION=0.3

.PHONY: all build upload clean

build:
	cargo build --release
# 	upx --brute ${BASEDIR}/test

upload: build
	mkdir -p ${BASEDIR}/rsync_dir/assets
	cp ${BASEDIR}/target/release/${NAME} ${BASEDIR}/rsync_dir
	cp -r ${BASEDIR}/target/release/assets ${BASEDIR}/rsync_dir/assets
	rsync -avz --exclude=".*" --delete -e "ssh -p ${DESTINATIONPORT}" ${BASEDIR}/rsync_dir ${DESTINATIONUSER}@${DESTINATIONMACHINE}:${DESTINATIONDIR}
	rm -rf ${BASEDIR}/rsync_dir

all: build upload

