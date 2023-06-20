#!/usr/bin/env bash

yum install -y epel-release
yum install -y --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-8.noarch.rpm
yum install -y --nogpgcheck https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-8.noarch.rpm
yum install -y ffmpeg ffmpeg-devel
