#include <bits/stdc++.h>

using namespace std;

typedef long long ll;
typedef long double ld;
typedef pair<ll, ll> pll;
typedef pair<int, int> pii;

const long long kk = 1000;
const long long ml = kk * kk;
const long long mod = ml * kk + 7;
const long long inf = ml * ml * ml + 7;

void solve() {
  int n;
  cin >> n;
  vector<ll> v(n, 0);
  for (auto& i : v) {
    cin >> i;
  }

  for (int i = 1; i < n; i++) {
    if (v[i - 1] > 1 && v[i] == 1) {
      cout << "-1\n";
      return;
    }
  }

  vector<ll> ops(n, 0);

  for (int i = 1; i < n; i++) {
    ll him = v[i - 1];
    ll me = v[i];
    ll extra = 0;
    while (him != 1 && him * him <= me) extra -= 1, him *= him;
    while (me < him) extra++, me *= me;

    ops[i] = max(0ll, ops[i - 1] + extra);
  }

  ll ans = 0;
  for (auto i : ops) ans += i;
  cout << ans << "\n";
}

int main() {
  ios_base::sync_with_stdio(0);
  cin.tie(0);
  cout.tie(0);
  int t = 1;
  cin >> t;
  while (t--) solve();
}
