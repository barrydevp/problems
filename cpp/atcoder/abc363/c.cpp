// https://atcoder.jp/contests/abc363/tasks/abc363_c

#include <bits/stdc++.h>
using namespace std;

#define ll long long

int main() {
  int n, k;
  cin >> n >> k;

  string s;
  cin >> s;

  vector<int> a(n);

  for (int i = 0; i < n; i++) {
    a[i] = s[i] - 'a';
  }
  sort(a.begin(), a.end());

  int ans = 0;
  bool ok, flag;
  while (true) {
    ok = true;

    for (int i = 0; i <= n - k; i++) {
      flag = true;
      for (int j = 0; j < k; j++) {
        if (a[i + j] != a[i + k - 1 - j]) {
          flag = false;
          break;
        }
      }
      if (flag) {
        ok = false;
        break;
      }
    }

    if (ok) {
      ans += 1;
    }

    if (!next_permutation(a.begin(), a.end())) {
      break;
    }
  }

  cout << ans << endl;

  return 0;
}
