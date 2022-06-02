#include<stack>
#include<string>

using namespace std;


class Solution {
public:

    char table[9][9] = {   //算符优先级表
    //           +   -   *   /   ^   !   (   )   #    // 当前运算符
        /* + */    '>','>','<','<','<','<','<','>','>',
        /* - */    '>','>','<','<','<','<','<','>','>',
        /* * */    '>','>','>','>','<','<','<','>','>',
        /* / */    '>','>','>','>','<','<','<','>','>',
        /* ^ */    '>','>','>','>','>','<','<','>','>',
        /* ! */    '>','>','>','>','>','>',' ','>','>',
        /* ( */    '<','<','<','<','<','<','<','=',' ',
        /* ) */    ' ',' ',' ',' ',' ',' ',' ',' ',' ',
        /* # */    '<','<','<','<','<','<','<',' ','='
        // 栈顶运算符
    };

    int calculate(string s) {
        s += '#';
        int flag = 0;
        stack<char> OPTR;//操作符栈
        stack<int> OPND;//操作数栈

        OPTR.push('#');


        for (int i = 0; i < s.size(); i++) {
            char c = s[i];
            if (c == ' ')
                continue;
            if (isdigit(c)) {
                if (flag) {   //连续数字
                    int t = OPND.top();
                    OPND.pop();
                    t = t * 10 - '0'+ c ;//防止int溢出
                    OPND.push(t);
                }
                else {
                    OPND.push(c - '0');
                    flag = 1;
                }
            }
            else {
                flag = 0;
                char opt = OPTR.top();
                switch (orderBetween(opt, c)) {

                case '<'://栈顶运算符优先级更低，则继续进栈
                    OPTR.push(c); break;
                case '=':
                    OPTR.pop(); break;
                case '>': {
                    i--;  //此时栈顶元素的优先级高于当前元素，此时不再进行解析，而是处理已经入栈的值(a的光标拉回到解析这个符号之前)
                    OPTR.pop();

                    if ('!' == opt) { //一元运算符
                        int num1 = OPND.top();  //第一个运算数
                        OPND.pop();
                        num1 = evaluate1(num1, opt);
                        OPND.push(num1);
                    }
                    else {//二元运算符
                        int num1 = OPND.top();
                        OPND.pop();
                        int num2 = OPND.top();
                        OPND.pop();

                        num1 = evaluate2(num1, opt, num2);
                        OPND.push(num1);
                    }
                    break;
                }
                }
            }
        }
        return OPND.top();
    }

    int factorial(int a) {//求阶乘
        if (a == 0 || a == 1) {
            return 1;
        }
        if (a < 0) {
            printf("ERROR,Negative number cannot calculate factorial");
            return 0;
        }
        int i, val = 1;
        for (i = a; i > 1; i--) {
            val *= i;
        }
        return val;
    }
    int evaluate1(int a, char opt) {
        if (opt == '!') {
            return factorial(a);
        }
        else {
            printf("ERROR 001");
            return 0;
        }
    }
    int evaluate2(int a, char opt, int b) {
        switch (opt) {
            case '+':
                return a + b;
            case '-':
                return b - a;
            case '*':
                return a * b;
            case '/': {
                if (a == 0) {
                    printf("ERROE zero cannot be div");
                    return 0;
                }
                return b / a;
            }
            case '^':
                return (int)pow(b, a);
        }
        return -1;
    }

    int getorder(char a) {
        switch (a) {
            case '+':
                return 0;
            case '-':
                return 1;
            case '*':
                return 2;
            case '/':
                return 3;
            case '^':
                return 4;
            case '!':
                return 5;
            case '(':
                return 6;
            case ')':
                return 7;
            case '#':
                return 8;
        }
        return -1;
    }
    char orderBetween(char a, char b) {
        int i, j;
        i = getorder(a);
        j = getorder(b);
        return table[i][j];
    }

};

int main()
{
    Solution s;
    //vector<int> input = { 0,2,3,4,6,8,9 };
    auto a = s.calculate("(1+(4+5+2)-3)+(6+8)");

    //for (auto t : a) {
    //    for (auto tt : t) {
    //        cout << tt << " , ";
    //    }
    //    cout << endl;
    //}

    //for (auto t : a) {
    //    cout << t << " , ";
    //}

    cout << a << endl;

    return 0;
}
