#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <stdbool.h>
#include <sys/wait.h>

int main() {
  puts("Welcome to my shell");
  char line[1024];

  while(true) {
    printf("> ");

    fgets(line, sizeof(line), stdin);
    line[strlen(line)-1] = '\0'; // strip \n

    if (strcmp(line, "exit") == 0) {
      break;
    }

    int pid = fork();

    if (pid == 0) {
      if (execlp(line, "", NULL) == -1) {
	puts("ERROR!");
	exit(1);
      }
    } else if(pid > 0) {
      int status;
      waitpid(pid, &status, 0);

      if (status != 0) {
	printf("Child exited with status %d\n", WEXITSTATUS(status));
      }
    } else {
      puts("ERROR!");
      break;
    }
  }

  return 0;
}
