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
    b=((1+a)%3)+1
    break
  case 2:
    b=a
    break
  case 3:
    b=(a%3)+1
    break
  }
  c=(3+a-b)%3
  if (c==0) {
    round=3+b
  } else if (c==2)  {
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
