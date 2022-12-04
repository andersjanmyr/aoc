BEGIN{
FS="[,-]"
}
{
  print $1, $2, $3, $4
  if (($1 >=$3 && $2 <= $4) || ($3 >=$1 && $4 <= $2)) sum++
}
END {
  print sum
}
