#!/usr/bin/env bash

dnf install -y epel-release
dnf install -y --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-8.noarch.rpm
dnf install -y --nogpgcheck https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-8.noarch.rpm
dnf install -y ffmpeg ffmpeg-devel
