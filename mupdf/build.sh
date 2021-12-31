#!/bin/sh
git clone --recursive git://git.ghostscript.com/mupdf.git
git submodule update --init
sudo make HAVE_X11=no HAVE_GLUT=no prefix=/usr/local install
