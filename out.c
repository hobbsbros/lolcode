#include <stdio.h>

int main(void) {

int VAR;
VAR = 0;
while (1) {
VAR = VAR + 1;
if (VAR > 10) {
break;
};
}
printf("%d\n", VAR);

}
