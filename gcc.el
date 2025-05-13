package com.hello
import os

msg "hello"
function(name=add,a,b){
	i = a + b
	return i
}
file.open(c://file,dom=elium.txt){
	parm1 = input("please type me")
	parm2 = input("please type me")
	add(parm1,parm2,return=i)
	if(name == :num:){
		msg "it is number for "+{i}!
	} else {
		msg "it is number!"
	}
file.edit("input value="{i}
}
exit;