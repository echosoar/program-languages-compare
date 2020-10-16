// - 变量-单个变量
const int a = 123;
int a(123); // C++
int a{123}; // C++ 11
int a = {123};
char b[] = "str";
bool c = false;
// - 变量-多个变量
int d = 1, e = 2;
const int f = 1, g = 2;

// - 注释
// 单行注释
/* 多行注释 */ 
// - 基本类型-字符串
char str[4] = {"s", "t", "r", "\0"};
char str[] = "str";
// #include <string>
std::string str = "str";

// - 基本类型-数字
int num = 123;
// - 基本类型-浮点数
float num = 1.49f;
double num = 1.49;
// - 基本类型-布尔值
bool f = false;
// - 数组-定义
int arr[2] = {1, 2};
int arr[] {1, 2};
// - 数组-获取长度
sizeof arr /sizeof(arr[0]) == 2;
// C++ 11
int length = std::end(arr) - std::begin(arr);
// - 数组-遍历
for (int index = 0; index < len; index ++) {
  int value = arr[i];
}
// C++ 11
for (auto value : arr) {}

// - 数组-截取
int a[1];
for (int index = 0, aIndex = 0; index < 1; index ++, aIndex ++) {
  a[aIndex] = arr[index];
}
// - 数组-连接
// #include <sstream>

std::stringstream ss;
int len = sizeof a / sizeof a[0];
for(int i = 0; i < len; i ++) {
   ss << a[i] << (i < len - 1 ? ".": "");
}

// - Map-创建

// - Map-设置

// - Map-读取

// - Map-删除

// - Map-遍历



// - Object-定义






// - Object-获取属性值


// - 正则表达式-定义

// - 正则表达式-是否匹配

// - 正则表达式-匹配结果


// match === 'ab'
// capture1 === 'b'

// matchIndex === 0

// - 条件



// - 循环-for
// 普通for循环

// 仅判断

// 无限循环

// 遍历索引

// 遍历元素

// - 循环-while

// - 选择-switch







// - 错误捕获






// - 方法-普通方法



// - 方法-匿名方法



// - 方法-IIFE




// - 类-声明








// - 类-继承





// - 类-实例化


// - JSON-定义













// - JSON-编码



// - JSON-解码


// - 时间-当前时间戳

// - 时间-获取时间








// - 系统变量-当前代码文件

// - 系统变量-当前代码目录

// - 系统变量-进程启动路径

// - 系统变量-环境变量

// - 系统变量-启动时入参

// - 系统变量-当前进程id

// - 系统变量-退出进程

// - 系统变量-输出
// include <iostream>
std::cout << value << endl;
