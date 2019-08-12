#
#	You need to have an SSH key on the app. Or password
# 	TEST APP PORT 15928
# 	PRODUCTION APP PORT 15928
#

BASEDIR=$(shell pwd)
DESTINATIONDIR=/srv/app/
NAME=lunchtime
VERSION=0.3

.PHONY: all build restart-services

build:
	cargo build --release
# 	upx --brute ${BASEDIR}/test

upload: build
	mkdir -p ${BASEDIR}/rsync_dir/assets
	cp ${BASEDIR}/target/release/${NAME} ${BASEDIR}/rsync_dir
	cp -r ${BASEDIR}/assets/. ${BASEDIR}/rsync_dir/assets
	rsync -avz --exclude=".*" --delete -e "ssh" \
		${BASEDIR}/rsync_dir/ lunchtime:${DESTINATIONDIR}
	rm -rf ${BASEDIR}/rsync_dir

restart-services:
	ssh lunchtime \
		'supervisorctl restart all'

all: upload restart-services
