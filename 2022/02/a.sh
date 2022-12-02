#! /bin/bash
gawk '
BEGIN{
one="ABC"
two="XYZ"
}
{
  a=index(one, $1)
  b=index(two, $2)
  if (a==b) {
    round=3+b
  } else if (a-b==-1 || (a-b==2))  {
    round=6+b
  } else {
    round=b
  }
  sum+=round
  print a b round
}
END {
print sum
}
'
