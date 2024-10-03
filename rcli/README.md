# RCLI

~~~bash
cg run -- csv -i assets/juventus.csv --format raw

cg run -- genpass -l 4

cg run -- base64 encode --format urlsafe

cg run -- text generate -o output/key.txt

cg run -- text encrypt -k output/key.txt -i assets/juventus.csv -o output/juventus.csv.enc

cg run -- text decrypt -k output/key.txt -i output/juventus.csv.enc -o output/juventus.csv.dec

cg run -- http server

cg run -- jwt sign

# 指定 secret key 生成
cg run -- jwt sign -k <my-secret-key>

# 指定 sub, aud, exp 生成
cg run -- jwt sign --sub acme --aud device1 --exp 14d

cg run -- jwt verify -t <token-value>

# 验证指定 secret key 生成的token
cg run -- jwt verify -k <my-secret-key> -t <token-value>

# 验证指定 aud 生成的token
cg run -- jwt verify -a <audience-value> -t <token-value>
~~~