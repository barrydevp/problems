#include <iostream>
#include <vector>
using namespace std;
#define neg1 cout << -1 << endl

int main() {
  int n;
  cin >> n;
  vector<int> v(n);
  for (int i = 0; i < n; i++) {
    cin >> v[i];
  }

  int mn = INT_MAX, sm = 1e9, tot = 0;
  vector<int> res(n);
  for (int i = 0; i < n; i++) {
    int curr = (sm / v[i]) + 1;
    mn = min(mn, curr * v[i]);
    tot += curr;
    res[i] = curr;
  }

  if (sm < mn && tot < mn) {
    // print_vec(res);
    for (int i = 0; i < n; i++) {
      cout << res[i] << " ";
    }
    return 0;
  }
  neg1;

  return 0;
}
