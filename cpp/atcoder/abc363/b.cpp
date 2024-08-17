// https://atcoder.jp/contests/abc363/tasks/abc363_b

#include <bits/stdc++.h>
using namespace std;

int main() {
  int n, t, p;
  cin >> n >> t >> p;

  vector<int> l(n);

  for (int i = 0; i < n; i++) {
    cin >> l.at(i);
  }

  sort(l.begin(), l.end());

  if (l[n - p] < t) {
    cout << t - l[n - p] << endl;
  } else {
    cout << 0 << endl;
  }

  return 0;
}
