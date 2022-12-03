#! /bin/bash
gawk '
BEGIN{
one="ABC"
two="XYZ"
}
{
  a=index(one, $1)
  c=index(two, $2)
  switch(c) {
  case 1:
    b=(a-1)
    if (b<1) b=3
    break
  case 2:
    b=a
    break
  case 3:
    b=(a+1)
    if (b>3) b=1
    break
  }
  if (a==b) {
    round=3+b
  } else if (a-b==-1 || (a-b==2))  {
    round=6+b
  } else {
    round=b
  }
  sum+=round
  print a c b round
}
END {
print sum
}
'
