#!/bin/bash
docker rm -f bg-eraser
docker build . -t bg-eraser 
docker run -it -p 8080:8080 bg-eraser

