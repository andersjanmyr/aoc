#! /bin/bash
gawk '
/^$/ {
  print sum
  sum=0
}
{
  sum+=$1
}
END {
  print sum
}' \
  | sort -n | tail -1
