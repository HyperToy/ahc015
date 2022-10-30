#include <bits/stdc++.h>
using namespace std;
using ll = long long;
#define rep(i,n) for (int i = 0; i < (n); ++i)
#define per(i,n) for (int i = (n) - 1; i >= 0; --i)

random_device seed_gen;
mt19937 engine{seed_gen()};
uniform_int_distribution<> dist{1, 1000000000};

const double TIME_LIMIT = 1.9;
struct Timer {
    clock_t start;
    Timer() { reset(); }
    void reset() { start = clock(); }
    double get() { return (double)(clock() - start) / CLOCKS_PER_SEC; }
};
Timer timer;

string ds = "RFLB";
const int dx[] = {0, -1, 0, 1};
const int dy[] = {1, 0, -1, 0};
bool inside(int h, int w, int H, int W) {
    return 0 <= h && h < H && 0 <= w && w < W;
}

struct State {
    int L;
    int N;
    int next;
    vector<int> f;
    vector<vector<int>> box;

    State (int L, vector<int> f) : f(f), L(L), N(L * L) {
        next = 0;
        box = vector(N, vector<int>(N, 0));
    }
    State (int L, vector<int> f, int next, vector<vector<int>> box): L(L), N(L * L), f(f),next(next), box(box) {}

    void put(int p) {
        int cnt = 0;
        rep(z,N) {
            int x = z / L;
            int y = z % L;
            if (box[x][y] == 0) cnt++;
            if (cnt == p) box[x][y] = f[next];
        }
        next++;
    }
    State move(int dir) {
        vector new_box(L, vector<int>(L, 0));
        if (dx[dir] != 0) {
            rep(i,L) rep(j,i) swap(new_box[i][j], new_box[j][i]);
            rep(i,L) rep(j,i) swap(box[i][j], box[j][i]);
        }
        rep(i,L) {
            if (dx[dir] + dy[dir] > 0) { // R
                rep(x,L/2) swap(box[i][x], box[i][L - 1 - x]);
                rep(x,L/2) swap(new_box[i][x], new_box[i][L - 1 - x]);
                int pos = 0;
                rep(j,L) {
                    if (box[i][j] == 0) continue;
                    new_box[i][pos] = box[i][j];
                    pos++;
                }
                rep(x,L/2) swap(new_box[i][x], new_box[i][L - 1 - x]);
                rep(x,L/2) swap(box[i][x], box[i][L - 1 - x]);
                
                // int pos = L - 1;
                // per(j,L) {
                //     if (box[i][j] == 0) continue;
                //     new_box[i][pos] = box[i][j];
                //     pos--;
                // }
            } else { // L
                int pos = 0;
                rep(j,L) {
                    if (box[i][j] == 0) continue;
                    new_box[i][pos] = box[i][j];
                    pos++;
                }
            }
        }
        if (dx[dir] != 0) {
            rep(i,L) rep(j,i) swap(box[i][j], box[j][i]);
            rep(i,L) rep(j,i) swap(new_box[i][j], new_box[j][i]);
        }
        return State{L, f, next, new_box};
    }
    int get_score() {
        int res = 0;
        vector seen(L, vector<bool>(L, false));
        queue<pair<int, int>> q;

        rep(i,L) rep(j,L) {
            if (box[i][j] == 0) seen[i][j] = true;
            if (seen[i][j]) continue;

            int cnt = 1;
            seen[i][j] = true;
            q.push({i, j});

            while (!q.empty()) {
                int x = q.front().first;
                int y = q.front().second;
                q.pop();

                rep(dir,4) {
                    int nx = x + dx[dir];
                    int ny = y + dy[dir];
                    if (!inside(nx, ny, L, L)) continue;
                    if (seen[nx][ny]) continue;
                    if (box[nx][ny] != box[i][j]) continue;
                    cnt++;
                    seen[nx][ny] = true;
                    q.push({nx, ny});
                }
            }

            res += cnt * cnt;
        }
        return res;
    }
};

void solve() {
    int L = 10;
    int N = L * L;

    vector<int> f(N);
    rep(i,N) cin >> f[i];

    State state{L, f};

    int score = 0;
    rep(i,N) {
        int p;
        cin >> p;
        state.put(p);

        int best_dir = -1;
        int best_score = -1;
        State best_state{L, f};

        rep(dir,2) {
            State now_state = state.move(dir);
            int now_score = now_state.get_score();
            if (now_score > best_score) {
                best_state = now_state;
                best_score = now_score;
                best_dir = dir;
            }
        }
        state = best_state;
        score = best_score;
        // int best_dir = dist(engine);
        cout << ds[best_dir] << endl;
    }

    cerr << "score : " << score << endl;
    // timer.reset();
    // while (timer.get() < TIME_LIMIT) {
    // }
}

int main(){
    solve();
} 