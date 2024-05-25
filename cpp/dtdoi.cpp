#include <iostream>
#define maxV 10000
using namespace std;
long long s, t, n, a[101], i, vt, d, c, g, res, j, f[20000];

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  cout.tie(0);
  cin >> n >> s;
  for (i = 1; i <= n; i++) cin >> a[i];
  sort(a + 1, a + 1 + n);
  if (s > maxV) {
    t = (s - maxV) / a[n];
    s = s - t * a[n];
  }
  cout << s << endl;
  f[0] = 0;
  for (i = 1; i <= s; i++) {
    f[i] = f[i - 1] + 1;
    for (j = 1; j <= n; j++)
      if (a[j] <= i) f[i] = min(f[i], f[i - a[j]] + 1);
  }
  cout << f[s] + t;
  return 0;
}
