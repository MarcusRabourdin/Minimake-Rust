#!/bin/sh

if [ $(cargo build; echo $?) -ne 0 ]; then 
  echo "Cargo build failed"
  exit 1
fi

binaryName="Minimake"
targetPath="../target/debug/$binaryName"
#echo -e $targetPath
filePath="./lexer/"

for file in lexer/*; do
 # echo -e "./$targetPath $filepath$file"
 result=$(./$targetPath $filepath$file)
 return_code=$(echo $?)
  
 if [ $return_code -ne 0 ]; then
    echo -e "$file failed"
  fi
  echo -ne $result
done

