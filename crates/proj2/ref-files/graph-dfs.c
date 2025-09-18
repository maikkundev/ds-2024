#include <stdio.h>
#include <stdlib.h>

#define N 10

int a[N][N] = { {0, 1, 1, 1, 0, 0, 0, 0, 0, 0},	// 0
				{1, 0, 0, 0, 1, 1, 0, 0, 0, 0}, // 1
				{1, 0, 0, 0, 0, 0, 1, 0, 0, 0},	// 2
				{1, 0, 0, 0, 0, 0, 0, 1, 0, 0},	// 3
				{0, 1, 0, 0, 0, 0, 0, 0, 0, 0},	// 4
				{0, 1, 0, 0, 0, 0, 1, 0, 1, 0},	// 5
				{0, 0, 1, 0, 0, 1, 0, 1, 0, 0},	// 6
				{0, 0, 0, 1, 0, 0, 1, 0, 1, 0},	// 7
				{0, 0, 0, 0, 0, 1, 0, 1, 0, 1},	// 8
				{0, 0, 0, 0, 0, 0, 0, 0, 1, 0}};
				
int mark[N];

void dfs(int k);

int main() {
	int s, m;
	
	printf("Give start node (1-%d) or out of range to end: ", N);
	scanf("%d", &s);
	while (s >= 1 && s <= N) {
		for (m = 0; m < N - 1; m++) 
			mark[m] = 0;
		dfs(s-1);
		printf("Give start node (1-%d) or out of range to end: ", N);
		scanf("%d", &s);
	}
	
	exit(0);
}

void dfs(int k) {
	int j;
	
	mark[k] = 1;
	printf("Now in %d\n", k+1);
	for (j=0; j <= N-1; j++) {
		if ((a[k][j] == 1) && (mark[j] == 0))
			dfs(j);
	}
}