#!/bin/bash

# Script to convert all png images within render to render/render.gif

FPS=20

ffmpeg -pattern_type glob -i 'render/*.png' -r $FPS render/render.gif
