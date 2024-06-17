#include <stdio.h>
#include <stdlib.h>

#define N 10
#define QSIZE N*N

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
				
int front, rear, queue[QSIZE], mark[N];

void bfs(int k);
int enqueue(int k);
int dequeue(int *k);

int main() {
	int s, m;
	
	printf("Give start node (1-%d) or out of range to end: ", N);
	scanf("%d", &s);
	while ((s >= 1) && (s <= N)) {
		front = -1;
		rear = -1;
		for (m = 0; m <= N-1; m++) 
			mark[m] = 0;
		bfs(s-1);
		printf("Give start node (1-%d) or out of range to end: ", N);
		scanf("%d", &s);
	}
	exit(0);
}

void bfs(int k) {
	int i, j, flag;
	flag = enqueue(k);
	mark[k] = 1;
	
	while (flag == 1) {
		flag = dequeue(&i);
		
		if (flag == 1) {
			printf("Now in %d\n", i+1);
			for (j=0; j <= N-1; j++) {
				if ((a[i][j] == 1) && (mark[j] == 0)) {
					flag = enqueue(j);
					mark[j] = 1;
				}
			}
		}
	}
}

int enqueue(int k) {
	int f = 0;
	
	if (front == -1) 
		front = 0;
	if (rear < QSIZE - 1) {
		rear = rear + 1;
		queue[rear] = k;
		f = 1;
	}
	return f;
}

int dequeue(int *k) {
	int f = 0;
	
	if (front != -1) {
		*k = queue[front];
		front = front + 1;
		f = 1;
	}
	
	if (front > rear) {
		rear = -1;
		front = -1;
	}
	return f;
}