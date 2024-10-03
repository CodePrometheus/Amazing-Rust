# RCLI

~~~bash
cg run -- csv -i assets/juventus.csv --format raw

cg run -- genpass -l 4

cg run -- base64 encode --format urlsafe

cg run -- text generate -o output/key.txt

cg run -- text encrypt -k output/key.txt -i assets/juventus.csv -o output/juventus.csv.enc

cg run -- text decrypt -k output/key.txt -i output/juventus.csv.enc -o output/juventus.csv.dec

cg run -- http server
~~~