# 名簿管理プログラム
## はじめに
+ Rustで書く．
+ http://www.suri.cs.okayama-u.ac.jp/~niitsuma/lect/p2/

## 基本仕様
+ 標準入力から「ID,氏名,年月日,住所,備考」からなるコンマ区切り形式(CSV形式)の名簿データを受け付けて， それらをメモリ中に登録する
+ 標準入力から%で始まるコマンドを受け付けて， 登録してあるデータを表示したり整列したりする

## コマンド一覧
| コマンド | 概要 |
|:-----------|:------------|
| %Q | 終了 | This |
| %C | 登録内容のチェック |
| %P n | 先頭からn件表示 | 
| %R file | fileから読み出し | 
| %W file | fileへの書き出し |
| %F word | 検索結果を表示 |
| %S n | CSVのn番目の項目で整列 |

## 使用したライブラリ
+ lazy_static ( グローバル変数 )
++ https://wasm.fastlylabs.com/docs/rust/lazy_static/index.html
