#! /bin/bash
gawk '
BEGIN{
alpha="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
sum=0
}
{
  l=length($1)
  m=l/2
  a=substr($1, 0, m)
  b=substr($1, m+1, l)
  split(a, chars, "")
  for (i=1; i <= m; i++) {
    j=index(b, chars[i])
    if (j > 0) {
      c=chars[i]
      v=index(alpha, c)
      print j, c, v, a, b
      sum+=v
      break
    }
  }
}
END {
  print sum
}
'
