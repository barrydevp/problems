#include <iostream>
#include <numeric>
#include <vector>
using namespace std;

using ll = long long;

int main() {
  cin.tie(nullptr)->sync_with_stdio(false);

  int tests;
  cin >> tests;
  while (tests--) {
    int n;
    ll x;
    cin >> n >> x;
    vector<ll> a(n + 1);
    for (int i = 1; i <= n; ++i) cin >> a[i];
    partial_sum(a.begin() + 1, a.end(), a.begin() + 1);
    // for (int i = 0; i <= n; ++i) cout << a[i] << ' ';
    // cout << endl;

    vector<int> dp(n + 2);
    for (int i = n; i > 0; --i) {
      auto q = upper_bound(a.begin(), a.end(), a[i - 1] + x) - a.start();
      dp[i] = dp[q + 1] + q - i;
    }
    cout << accumulate(dp.begin(), dp.end(), 0ll) << '\n';
  }
}
