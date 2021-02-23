#!/bin/bash

# Script to convert all png images within render to render/render.gif

FPS=20

ffmpeg -pattern_type glob -i 'render/*.png' -vf "scale=400x400,fps=$FPS" render/render.gif
