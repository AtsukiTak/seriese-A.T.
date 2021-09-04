#!/bin/bash

assert_exit_code() {
  asmfile="$1"
  expected="$2"

  echo "Assemble ${asmfile}..."

  cat $asmfile | ../target/debug/at-asm > tmp.o
  ld -lSystem -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib tmp.o
  ./a.out
  code="$?"

  rm a.out tmp.o

  if [ "$code" = "$expected" ]; then
    echo "OK"
  else
    echo "Test failed. Status code is ${code} but expected ${expected}."
    exit 1
  fi
}

cargo build

assert_exit_code "tests/minimum.s" 42
assert_exit_code "tests/many_mov.s" 42
