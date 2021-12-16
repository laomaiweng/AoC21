#!/bin/bash
ffmpeg -i %d.bmp -r 10 -c:v libx264 -preset slow -crf 18 output.mp4
