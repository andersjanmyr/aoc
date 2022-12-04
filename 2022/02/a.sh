#! /bin/bash
gawk '
BEGIN{
one="ABC"
two="XYZ"
}
{
  a=index(one, $1)
  b=index(two, $2)
  c=(3+a-b)%3
  if (c==0) {
    round=3+b
  } else if (c==2)  {
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
