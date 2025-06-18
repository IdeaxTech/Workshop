## Gemini Code Assistantの確認用

- md内のプログラムの解析ができるテスト

```python
n = int(input("数字を入力>> "))
checkPrime = True
while i * i <= n:
   if n % i == 0:
	checkPrime = False
	break 

if prime_count: print(f"{n}は素数です")
else: print(f"{n}は素数ではありません")
