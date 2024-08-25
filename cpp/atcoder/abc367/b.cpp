// https://atcoder.jp/contests/abc367/tasks/abc367_b

#include <bits/stdc++.h>

using namespace std;

// clang-format off

template<typename A, typename B> ostream& operator<<(ostream &os, const pair<A, B> &p) { return os << '(' << p.first << " " << p.second << ')'; }
template<typename T_container, typename T = typename enable_if<!is_same<T_container, string>::value, typename T_container::value_type>::type> ostream& operator<<(ostream &os, const T_container &v) { os << '{'; string sep; for (const T &x : v) os << sep << x, sep = ", "; return os << '}'; }

#define ar array
#define ll long long
#define ld long double
#define sza(x) ((int)x.size())
#define all(a) (a).begin(), (a).end()

const int MAX_N = 1e5 + 5;
const ll MOD = 1e9 + 7;
const ll INF = 1e9;
const ld EPS = 1e-9;

// clang-format on

void solve() {
  string n;
  cin >> n;

  string ans = "";
  bool flag = false;
  for (int i = n.size() - 1; i >= 0; i--) {
    if (flag) {
      ans += n[i];
    } else if (n[i] != '0') {
      flag = true;
      if (n[i] != '.') {
        i += 1;
      }
    }
  }
  // reverse the ans string
  reverse(ans.begin(), ans.end());

  cout << ans << endl;
}

int main() {
  ios_base::sync_with_stdio(0);
  cin.tie(0);
  cout.tie(0);
  int tc = 1;
  // cin >> tc;
  for (int t = 1; t <= tc; t++) {
    // cout << "Case #" << t << ": ";
    solve();
  }
}
