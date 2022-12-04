#! /bin/bash
gawk '
BEGIN{
FS="[,-]"
}
{
  print $1, $2, $3, $4
  if (!($1 >$4 || $2 < $3)) sum++
}
END {
  print sum
}
'
