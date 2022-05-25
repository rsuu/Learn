# -*- coding: UTF-8 -*-

from bs4 import BeautifulSoup
import requests, sys


fo = open("./q.log", "r")
#print ("文件名: ", fo.name)
#link = fo.read()
#.splitlines()
lines = fo.readlines()  # 读取所有行
link = lines[0].replace("\n", "")  # 取第一行  并且忽略换行符
#link = 'https://stackoverflow.com/questions/12330522/how-to-read-a-file-without-newlines'


#print ("", link)

#link = str(sys.argv[1])
hearders = {'headers':'Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:51.0) Gecko/20100101 Firefox/51.0'}
web_code = requests.get(link, headers=hearders)
al = web_code.text
title = al[al.find('<title>') + 7 : al.find('</title>')]
print(title)
#fo.close()
