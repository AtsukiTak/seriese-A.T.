#!/bin/bash

# cleanup output directory
output_dir="/tmp/at-asm/tests"
[ -d $output_dir ] && rm -rf $output_dir
mkdir -p /tmp/at-asm/tests

# prepare "at-asm"
cargo build
asm="../target/debug/at-asm"

# アセンブル、リンク、実行をし、
# 終了コードと標準出力をテストする
run() {
  testcase=$1
  input_file="tests/${testcase}.s"
  output_file="$output_dir/${testcase}.o"
  bin_file="$output_dir/${testcase}.bin"
  expected_exit_code="$2"
  expected_output="$3"

  echo "======================"
  echo "  Test ${testcase}"

  $asm -o $output_file $input_file

  asm_result="$?"
  if [ "$asm_result" -ne 0 ]; then
    echo "  Failed to assemble. Exit test."
    exit $asm_result
  fi

  # link
  ld -lSystem -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib $output_file -o $bin_file

  # run
  output="$($bin_file)"
  code="$?"

  # compare
  if [ "$code" != "$expected_exit_code" ]; then
    echo "  FAILED"
    echo "  status code is ${code} but expected ${expected_exit_code}."
    echo "======================"
    exit 1
  elif [ "$output" != "$expected_output" ]; then
    echo "  FAILED"
    echo "  output is ${output} but expected ${expected_output}."
    echo "======================"
    exit 1
  else
    echo "  SUCCESS!!"
    echo "======================"
  fi
}

cargo build

run "minimum" 24
run "many_mov" 42
run "simple_hello_world" 0 "Hello world!"
