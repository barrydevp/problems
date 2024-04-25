ntest = int(input())
tc = []

max_e = 0
max_f = 0
for t in range(ntest):
    [n, m] = map(int, input().split(" "))
    tc.append((n, m))
    if n > max_e:
        max_e = n
    if m > max_f:
        max_f = m

dp = [[0 for _ in range(max_f + 1)] for _ in range(max_e + 1)]
for i in range(max_f + 1):
    dp[1][i] = i
for i in range(max_e + 1):
    dp[i][1] = 1

for i in range(2, max_f + 1):
    for j in range(2, max_e + 1):
        dp[j][i] = max_f
        for k in range(1, i + 1):
            dp[j][i] = min(dp[j][i], 1 + max(dp[j][i - k], dp[j - 1][k - 1]))

for t in tc:
    print(dp[t[0]][t[1]])
