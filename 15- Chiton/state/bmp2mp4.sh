#!/bin/bash
ffmpeg -r 60 -i %d.bmp -vf tpad=stop_mode=clone:stop_duration=3 -c:v libx264 -preset slow -crf 18 output.mp4
