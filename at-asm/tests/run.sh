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

  # 期待する終了コード
  # "## TEST: Exit Code "42"" のように記述する
  # デフォルト値は0
  expected_exit_code=$(
    grep -e "^;; TEST: Exit Code" $input_file |
    sed -e 's/^;; TEST: Exit Code "\([0-9]*\)"/\1/'
  )
  if [ -z "$expected_exit_code" ]; then
    expected_exit_code="0"
  fi

  # 期待する標準出力
  # "## TEST: STDOUT "hogehoge"" のように記述する
  # デフォルト値は""
  expected_output=$(
    grep -e "^;; TEST: STDOUT" $input_file |
    sed -e 's/^;; TEST: STDOUT "\(.*\)"/\1/'
  )

  echo "======================"
  echo "  Test ${testcase}"

  # アセンブラの実行
  $asm -o $output_file $input_file

  # アセンブラの実行結果のテスト
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

for file in $(find tests -name "*.s")
do
  # file は "tests/minimum.s" 的な値
  # これを "minimum" に変換している
  run $(echo $file | sed -e "s/^tests\/\(.*\).s/\1/")
done
