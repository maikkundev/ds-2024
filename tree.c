#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct node {
	char am[10];
	char on[50];
	char ep[50];
	float b;
	int ap;
	struct node *left;
	struct node *right;
};

struct node *createNode();
void addInOrder(struct node *root, struct node *newNode);
void showInOrder(struct node *root);
int countNodes(struct node *root);
struct node *findByAM(struct node *root, char e[]);
struct node *maxAM(struct node *root);
float maxGrade(struct node *root);

int main() {
	int ch;
	struct node *temp, *root = NULL;
	char userAM[10];
	float maxiGra;
	
	do {
		printf("\n---- Menu");
		printf("\n---- 1. Add in order ");
		printf("\n---- 2. Show in order ");
		printf("\n---- 3. Find student by AM ");
		printf("\n---- 4. Show last student by AM ");
		printf("\n---- 5. Show maximum grade ");
		printf("\n---- 0. Exit ");
		printf("\nChoice: ");
		scanf("%d", &ch);
		switch(ch) {
			case 1:
				temp = createNode();
				if (temp != NULL) 
					if (root == NULL) 
						root = temp;
					else
						addInOrder(root, temp);
				else
					printf("No memory available!\n");
				break;
			case 2:
				printf("Tree contents: \n");
				showInOrder(root);
				printf("Number of nodes: %d\n", countNodes(root));
				break;
			case 3:
				printf("Give registration number (AM): ");
				scanf("%s", userAM);
				temp = findByAM(root, userAM);
				if (temp == NULL) 
					printf("Not found! \n");
				else
					printf("Found AM: %s %s  %s %.1f %d\n", temp->am,
								temp->on, temp->ep, temp->b, temp->ap);
				break;
			case 4:
				if (root == NULL) 
					printf("Empty tree\n");
				else {
					temp = maxAM(root);
					printf("Last AM: %s %s %s %.1f %d\n", temp->am,
								temp->on, temp->ep, temp->b, temp->ap);
				}
				break;
			case 5:
				maxiGra = maxGrade(root);
				if (maxiGra == -1) 
					printf("Empty tree! \n"); 
				else 
					printf("Max grade: %.1f\n", maxiGra);
				break;
			case 0:
				printf("Bye\n");
				break;
		} /* switch */
		
	} while (ch != 0);
	
	exit(0);
}

struct node *createNode() {
	struct node *r;
	
	r = (struct node *) malloc(sizeof(struct node));
	if (r != NULL) {
		printf("Give data: ");
		scanf("%s %s %s %f %d", r->am, r->on, r->ep, &r->b, &r->ap);
		r->left = NULL;
		r->right = NULL;
	}
	return r;
}

void addInOrder(struct node *root, struct node *newNode) {
	if (strcmp(newNode->am, root->am) <= 0) { /* προς τα αριστερά */
		if (root->left == NULL) { /* δεν υπάρχει αριστερό υποδένδρο */
			root->left = newNode;
		}
		else {	/* υπάρχει αριστερό υποδένδρο */
			addInOrder(root->left, newNode);
		}
	}
	else {  /* προς τα δεξιά */
		if (root->right == NULL) { /* δεν υπάρχει δεξί υποδένδρο */
			root->right = newNode;
		}
		else { /* υπάρχει δεξί υποδένδρο */
			addInOrder(root->right, newNode);
		}
	}
}

void showInOrder(struct node *root) {
	if (root != NULL) {
		showInOrder(root->left);
		printf("%s %s %s %.1f %d\n", root->am, root->on, root->ep,
					root->b, root->ap);
		showInOrder(root->right);
	}
}

int countNodes(struct node *root) {
	int result = 0;
	
	if (root != NULL) {
		result = countNodes(root->left);
		result++;
		result += countNodes(root->right);
	}
	return result;
}

struct node *findByAM(struct node *root, char e[]) {
	struct node *result = NULL;
	int comparison;
	
	if (root != NULL) {
		comparison = strcmp(e, root->am);
		if (comparison < 0) 
			result = findByAM(root->left, e);
		else if (comparison == 0) 
			result = root;
		else
			result = findByAM(root->right, e);
	}
	return result;
}

struct node *maxAM(struct node *root) {
	struct node *r;
	
	if (root->right == NULL) 
		r = root;
	else
		r = maxAM(root->right);
	return r;
}

float maxGrade(struct node *root) {
	float result = -1, resultRight;
	
	if (root != NULL) {
		result = maxGrade(root->left);
		if (result < root->b) 
			result = root->b;
		resultRight = maxGrade(root->right);
		if (result < resultRight) 
			result = resultRight;
	}
	return result;
}
