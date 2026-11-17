// Main file for project in C
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#ifdef _MSC_VER
#define strcasecmp _stricmp
#define strdup _strdup
#else
#include <strings.h>
#endif

typedef struct {
    int to; // index of neighbor
    double weight;
} Edge;

typedef struct { 
    char *name; // copy of name
    Edge *edges; // dynamic array
    int edgeCount;
    int edgeCapacity;
} Node;

typedef struct {
    Node *nodes;
    int count;
    int capacity;
} Graph;

void graphInit(Graph *g){
    g->nodes = NULL;
    g->count = 0;
    g->capacity = 0;
}

void freeGraph(Graph *g){
    for(int i = 0; i < g->count; i++){
        free(g->nodes[i].name);
        free(g->nodes[i].edges);
    }
    free(g->nodes);
    g->nodes = NULL;
    g->count = 0;
    g->capacity = 0;
}

static char *trim(char *str){
    while (*str == ' ' || *str == '\t') str++;
    char *end = str + strlen(str);
    while (end > str && (end[-1] == ' ' || end[-1] == '\t' || end[-1] == '\n' || end[-1] == '\r')) end--;
    *end = '\0';
    return str;
}



int findOrAddNode(Graph *g, const char *name){
    for(int i = 0; i < g->count; i++){
        if(strcmp(g->nodes[i].name, name) == 0){
            return i;
        }
    }
    if(g->count == g->capacity){
        int new_capacity = (g->capacity == 0) ? 4 : g->capacity * 2;
        Node *tmp = realloc(g->nodes, new_capacity * sizeof(Node));
        if(tmp == NULL) {
            perror("realloc");
            exit(1);
        }
        g->nodes = tmp;
        g->capacity = new_capacity;
    }
    g->nodes[g->count].name = strdup(name);
    g->nodes[g->count].edges = NULL;
    g->nodes[g->count].edgeCount = 0;
    g->nodes[g->count].edgeCapacity = 0;
    return g->count++;
}

// Add edge to list of edges (paths and costs)
void addEdge(Graph *g, int from, int to, double weight){ 
    Node *n = &g->nodes[from];
    if(n->edgeCount == n->edgeCapacity){ // list of edges must double if full
        int new_capacity = (n->edgeCapacity == 0) ? 2 : n->edgeCapacity * 2;
        Edge *tmp = realloc(n->edges, new_capacity * sizeof(Edge));
        if(tmp == NULL) {
            // Handle memory allocation failure
            perror("realloc");
            exit(1);
        }
        n->edges = tmp;
        n->edgeCapacity = new_capacity;
    }
    n->edges[n->edgeCount].to = to;
    n->edges[n->edgeCount].weight = weight;
    n->edgeCount++;
}

void readGraph(Graph *g, const char *filename){
    FILE *file = fopen(filename, "r");
    if(file == NULL){
        perror("fopen");
        exit(1);
    }
    char line[1024];
    int lineNum = 0;

    while(fgets(line, sizeof(line), file) != NULL){
        lineNum++;
        if (lineNum == 1) continue;
        char *text = trim(line);
        if (text[0] == '\0') continue;
        char from[256], to[256];
        double weight;
        if (sscanf(text, "%255[^,],%255[^,],%lf", from, to, &weight) != 3) {
            printf("Skipping line %d: expected 3 fields\n", lineNum);
            continue;
        }
        int fromIndex = findOrAddNode(g, trim(from));
        int toIndex = findOrAddNode(g, trim(to));
        addEdge(g, fromIndex, toIndex, weight);
    }
    fclose(file);
}

int findNodeIgnoreCase(Graph *g, const char *name){
    for(int i = 0; i < g->count; i++){
        if(strcasecmp(g->nodes[i].name, name) == 0){
            return i;
        }
    }
    return -1;
}

static int compareNodePtrs(const void *a, const void *b){
    Node *na = *(Node * const *)a;
    Node *nb = *(Node * const *)b;
    return strcmp(na->name, nb->name);
}

int shortestPath(Graph *g, int start, int end, double *costOut, int *path, int *pathLen){
  
    int n = g->count;
    double *dist = malloc(n * sizeof(double));
    int *prev = malloc(n * sizeof(int));
    int *visited = malloc(n * sizeof(int));

    for(int i = 0; i < n; i++){
        dist[i] = INFINITY;
        prev[i] = -1;
        visited[i] = 0;
    }
    dist[start] = 0.0;

    while(1){
        int current = -1;
        double best = INFINITY;
        for(int i = 0; i < n; i++){
            if(!visited[i] && dist[i] < best){
                best = dist[i];
                current = i;
            }
        }
        if(current == -1) break;
        if(current == end) break;
        visited[current] = 1;
        Node *node = &g->nodes[current];
        for(int j = 0; j < node->edgeCount; j++){
            Edge *e = &node->edges[j];
            double newDist = dist[current] + e->weight;
            if (newDist < dist[e->to]){
                dist[e->to] = newDist;
                prev[e->to] = current;
            }
        }

    }

    int found = (dist[end] != INFINITY);
    if(found){
        *costOut = dist[end];
        int len = 0;
        for (int v = end; v != -1; v = prev[v]){
            path[len++] = v;
        }
        *pathLen = len;
    }

    free(dist);
    free(prev);
    free(visited);
    return found; // 1 if path found, 0 otherwise


}

void printGraph(Graph *g) {
    if (g->count == 0) {
        printf("Graph is empty.\n");
        return;
    }

    Node **sorted = malloc(g->count * sizeof(Node *));
    for (int i = 0; i < g->count; i++) {
        sorted[i] = &g->nodes[i];
    }
    qsort(sorted, g->count, sizeof(Node *), compareNodePtrs);
    for (int i = 0; i < g->count; i++) {
        Node *node = sorted[i];
        printf("Node %s:\n", node->name);
        for (int j = 0; j < node->edgeCount; j++) {
            Edge *e = &node->edges[j];
            printf("  -> %s (weight: %.2f)\n", g->nodes[e->to].name, e->weight);
        }
    }
    free(sorted);
}
// Keeps asking until the user types a place that exists. Returns its index.
int promptForPlace(Graph *g, const char *prompt){
    char input[256];
    while(1){
        printf("%s", prompt);
        fflush(stdout);
        if(fgets(input, sizeof(input), stdin) == NULL){
            exit(1);   // input was closed
        }
        int idx = findNodeIgnoreCase(g, trim(input));
        if(idx != -1) return idx;
        printf("That place is not in the graph.\n");
    }
}

int main(int argc, char *argv[]) {
    const char *filePath = (argc > 1) ? argv[1] : "C:\\Users\\jackm\\COSC450\\final_project\\Samples\\city_roads.csv";

    Graph g;
    graphInit(&g);
    readGraph(&g, filePath);

    if(g.count == 0){
        printf("The graph is empty.\n");
        freeGraph(&g);
        return 1;
    }

    printGraph(&g);

    int start = promptForPlace(&g, "Where are you starting from? ");
    int end = promptForPlace(&g, "Where are you ending at? ");

    int *route = malloc(g.count * sizeof(int));
    int routeLen = 0;
    double cost = 0.0;

    if(!shortestPath(&g, start, end, &cost, route, &routeLen)){
        printf("Path is not possible\n");
    } else {
        printf("The total cost for starting at %s and ending at %s is: %.1f\n\n",
               g.nodes[start].name, g.nodes[end].name, cost);

        // route is backward (end first), so print from the last index down
        for(int i = routeLen - 1; i >= 0; i--){
            const char *place = g.nodes[route[i]].name;
            if(i == routeLen - 1)  printf("Start: %s\n", place);
            else if(i == 0)        printf("End:   %s\n", place);
            else                   printf("Go to: %s\n", place);
        }
    }

    free(route);
    freeGraph(&g);
    return 0;
}