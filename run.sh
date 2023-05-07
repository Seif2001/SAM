

#!/bin/bash

while getopts 'ch:' OPTION; do
  case "$OPTION" in
    c) ./Ununtu-task-manager/target/debug/tkm2_0;;
    
    h) ./Ununtu-task-manager/target/debug/tkm2_0 "help";;
        
    ?)
      echo "To use enter -c for running the command line task manager or -g to run the gui compnent or -h for CLI task manager help" >&2
      exit 1
      ;;
  esac
done
