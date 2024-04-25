# https://oj.vnoi.info/problem/cowgirl

mask = 3


def check_fn(a, b, n):
    for k in range(n - 1):
        if ((a >> (k)) & mask) | (((b >> (k)) & mask) << 2) in [0, 15]:
            return False
    # for i in range(n-1):
    #     if (a&(1<<i))!=0 and (a&(1<<(i+1)))!=0 and (b&(1<<i))!=0 and (b&(1<<(i+1)))!=0:return False
    #     if (a&(1<<i))==0 and (a&(1<<(i+1)))==0 and (b&(1<<i))==0 and (b&(1<<(i+1)))==0:return False

    return True


if __name__ == "__main__":
    tc = int(input())

    for t in range(tc):
        [m, n] = map(int, input().split(" "))
        if m < n:
            m, n = n, m
        check = [[0 for _ in range(1 << n)] for _ in range(1 << n)]
        f = [[0 for _ in range(1 << n)] for _ in range(m)]

        for i in range(1 << n):
            for j in range(1 << n):
                check[i][j] = check_fn(i, j, n)

        s = 0
        for i in range(m):
            for j in range(1 << n):
                if i == 0:
                    f[i][j] = 1
                else:
                    f[i][j] = sum([f[i - 1][k] for k in range(1 << n) if check[j][k]])
                if i == m - 1:
                    s += f[i][j]

        # print(sum(f[m-1][k] for k in range(1<<n)))
        print(s)
