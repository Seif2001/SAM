#include <sys/time.h>
#include <sys/resource.h>
#include <stdio.h>

int chPrio(int pid, int s){
    int res;
   
    res = setpriority(PRIO_PROCESS, pid, s);
   

    return res;

}

