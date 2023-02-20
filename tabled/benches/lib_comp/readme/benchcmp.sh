#!/bin/sh

critcmp | tail +3 | awk '{print $1, $3 }' | awk -F "/" '{print $0, $1, $2, $3}'  | awk '{print $1, $3, $4, $5, $6}'
