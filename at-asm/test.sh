#!/bin/bash

assert_exit_code() {
  asmfile="$1"
  expected="$2"

  echo "Assemble ${asmfile}..."

  cat $asmfile | ../target/debug/at-asm > /tmp/at-asm-test.o
  asm_exit="$?"
  if [ "$asm_exit" -ne 0 ]; then
    echo "Failed to assemble. Exit test."
    exit $asm_exit
  fi

  ld -lSystem -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib /tmp/at-asm-test.o -o /tmp/at-asm-test.out
  /tmp/at-asm-test.out
  code="$?"

  if [ "$code" = "$expected" ]; then
    echo "OK"
  else
    echo "Test failed. Status code is ${code} but expected ${expected}."
    exit 1
  fi
}

cargo build

assert_exit_code "tests/minimum.s" 24
assert_exit_code "tests/many_mov.s" 42
