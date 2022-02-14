#!/usr/bin/env bash

rm words.db

for DICT in uk en
do
  sqlite3 words.db "create table words_$DICT(word)"

  aspell --clean-affixes --clean-words -d $DICT dump master \
    | cut -d / -f 1 \
    | grep -v "[']" \
    | awk '{if(length==5) print tolower($0)}' \
    | sort | uniq \
    | sqlite3 -csv words.db ".import '|cat -' words_$DICT"
done
