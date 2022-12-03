#! /bin/bash
gawk '
BEGIN{
alpha="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
sum=0
lines=0
}
{
  lines++
  ci=lines%3+1
  chunk[ci]=$1
  if (ci == 1) {
    split(chunk[1], chars, "")
    for (i=1; i <= length(chunk[1]); i++) {
      j=index(chunk[2], chars[i])
      k=index(chunk[3], chars[i])
      if (j > 0 && k > 0) {
        c=chars[i]
        v=index(alpha, c)
        print j, c, v, chunks[1], chunks[2], chunks[3]
        sum+=v
        break
      }
    }
  }
}
END {
  print sum
}
'
