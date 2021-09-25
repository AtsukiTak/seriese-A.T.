#!/bin/bash

# cleanup output directory
output_dir="/tmp/at-asm/tests"
[ -d $output_dir ] && rm -rf $output_dir
mkdir -p /tmp/at-asm/tests

# prepare "at-asm"
cargo build
asm="../target/debug/at-asm"

assert_exit_code() {
  testcase=$1
  input_file="tests/${testcase}.s"
  output_file="$output_dir/${testcase}.o"
  bin_file="$output_dir/${testcase}.bin"
  expected_exit_code="$2"

  echo "======================"
  echo "  Test ${testcase}"
  echo "  $asm -o $output_file $input_file"

  $asm -o $output_file $input_file

  asm_result="$?"
  if [ "$asm_result" -ne 0 ]; then
    echo "  Failed to assemble. Exit test."
    exit $asm_result
  fi

  # link and run
  ld -lSystem -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib $output_file -o $bin_file
  $bin_file
  code="$?"

  # compare
  if [ "$code" = "$expected_exit_code" ]; then
    echo "  SUCCESS!!"
    echo "======================"
  else
    echo "  FAILED"
    echo "  status code is ${code} but expected ${expected_exit_code}."
    echo "======================"
    exit 1
  fi
}

cargo build

assert_exit_code "minimum" 24
assert_exit_code "many_mov" 42
assert_exit_code "simple_hello_world" 0
