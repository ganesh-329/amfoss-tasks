#include <stdio.h>

int main() {

    int t,n,i,x,j;
    int mana;
    int a[100];
    scanf("%d",&t);
    for(x=0;x<t;x++)
    {
        scanf("%d",&n);
        for(i=0;i<n;i++)
        {
            scanf("%d",&a[i]);
        }
    int flag = 1;
    for(i=0;i<n;i++)
    {
        for(j=i+1;j<n;j++)
        {
            if(a[i]==a[j])
            {
                flag = 0;
                break;
            }
        }
    }
    int flag0 = 0;
    for(i=0;i<n;i++)
    {
        if(a[i] == 0)
        {
            flag0++;
        }
        
    }
    if (flag == 1 && flag0 == 0) {
            mana = n + 1;
        }
        else if (flag0 != 0) {
            mana = n - flag0;
        }
        else {
            mana = n;
        }
        printf("%d\n", mana);
    }
    
    return 0;
}
