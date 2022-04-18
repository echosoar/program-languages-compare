# 多语言速查表

对比多种编程语言之间的异同

Compare the similarities and differences between various programming languages

Online: [program-languages-compare](https://echosoar.github.io/program-languages-compare/)


## 基本概念

### 变量

#### 单个变量

##### typescript
```typescript
const a: number= 1;
let b: string = 'str';
var c: boolean = false;
```

##### rust
```rust
// a 为 不可变变量，即不能改变值
let a: u64 = 1;
// 不可变变量可以重新定义
// rust 支持自动类型推导
let a = 2;
// 使用 mut 关键词，使变量可变
let mut ma = 1;
// ma = "abc"; 报错，类型错误
// ma = 1.2; 报错，类型错误，精度损失
const b = "str"
let c = true;
let d: bool = false;

```

##### golang
```golang
const a int = 1;
var b string = "str";
c := false;
```

##### c++
```c++
const int a = 123;
int a(123); // C++
int a{123}; // C++ 11
int a = {123};
char b[] = "str";
bool c = false;
```


#### 多个变量

##### typescript
```typescript
let d: number = 1, e: number = 2;
const f: number = 1, g: number = 2;
```

##### rust
```rust
let (a, b, c) = (42, "hello", 5.0);
```

##### golang
```golang
var a, b  = 1, 2;
const (
  a = 1
  b = 2
  c = 3
)
```

##### c++
```c++
int d = 1, e = 2;
const int f = 1, g = 2;

```


### 注释

##### typescript
```typescript
// 单行注释
/* 多行注释 */ 
```

##### rust
```rust
// 单行注释
/// 三个反斜也可以
/* 多行注释 */ 
```

##### golang
```golang
// 单行注释
/* 多行注释 */ 
```

##### c++
```c++
// 单行注释
/* 多行注释 */ 
```


## 类型

### 基本类型

#### 字符串

##### typescript
```typescript
let str: string = 'str';
```

##### rust
```rust
// 字符
let char1: char = 'a';
let char2 = 'a';
// 字符串
let str1 = "string";
let str1: &str = "string";
let mut str2 = String::new();
str2 = str1.to_string();
// " 与 ' 都需要使用 \ 转义
let str1 = "\"test\"";
// 如果不想转义，可以使用 原始字符串
let str1 = r#"sam said: "xxx"."#
// 如果原始字符串中出现 #，两侧需要更多的 #
let str1 = r###"##"###;
```

##### golang
```golang
var str string = "str";
var str = "str";
```

##### c++
```c++
char str[4] = {"s", "t", "r", "\0"};
char str[] = "str";
// #include <string>
std::string str = "str";

```


#### 数字

##### typescript
```typescript
let num: number = 123;
```

##### rust
```rust
let num = 123; // i32
let num64: i64 = 123;
let float64 = 2.0; // f64
let float32: f32 = 3.0; // f32
```

##### golang
```golang
var num int = 123;
```

##### c++
```c++
int num = 123;
```


#### 布尔值

##### typescript
```typescript
const bf: boolean = false;
```

##### rust
```rust
let c = true;
let d: bool = false;
```

##### golang
```golang
var f bool = false;
f := false;
```

##### c++
```c++
bool f = false;
```


### 数组

#### 定义

##### typescript
```typescript
let arr: Array<number> = [1, 2];
```

##### rust
```rust
let arr = [1, 2];
// [类型;数量]
let arr: [i32;2] = [1, 2];
// 都初始化为 -1
let arr: [i32;2] = [-1;2];
// 可变长数组 vector
let varr: Vec<i32> = Vec::new();
// rust 提供了宏 vec! 根据值创建 vector
ler mut varr = vec![1,2];
```

##### golang
```golang
// 数组
arr := [2]int{1, 2};
var arr = [...]int{1, 2};
// 切片
arr := []int{1, 2}
```

##### c++
```c++
int arr[2] = {1, 2};
int arr[] {1, 2};
```


#### 获取长度

##### typescript
```typescript
arr.length === 2
```

##### rust
```rust
arr.len() == 2
```

##### golang
```golang
len(arr) == 2
```

##### c++
```c++
sizeof arr /sizeof(arr[0]) == 2;
// C++ 11
int length = std::end(arr) - std::begin(arr);
```


#### 遍历

##### typescript
```typescript
arr.forEach((item: number, i: number) => {
  // do something
});
arr.map((item: number, i: number) => {
  return item;
});
```

##### rust
```rust
// 迭代值
for val in arr.iter(){
    println!("value is :{}", val);
}
for index in 0..2 {
    println!("index is: {} & value is : {}",index, arr[index]);
}
// 遍历 vector 数组
for i in &mut varr {
    println!("{}", i);
}
```

##### golang
```golang
for index, value := range arr {
  // do something
}
```

##### c++
```c++
for (int index = 0; index < len; index ++) {
  int value = arr[i];
}
// C++ 11
for (auto value : arr) {}

```


#### 截取

##### typescript
```typescript
arr.slice(0, 1)
```

##### rust
```rust
&arr[0..1];
// 包含结尾元素需要加 =
&arr[0..=0];
```

##### golang
```golang
arr[0:1]
```

##### c++
```c++
int a[1];
for (int index = 0, aIndex = 0; index < 1; index ++, aIndex ++) {
  a[aIndex] = arr[index];
}
```


#### 连接

##### typescript
```typescript
var strArr: string[] = ['a', 'b'];
strArr.join(',')
```

##### rust
```rust
// "hello world"
["hello", "world"].join(" ");
// [1, 2, 0, 3, 4]
[[1, 2], [3, 4]].join(&0)
// [1, 2, 3, 4]
[[1, 2], [3, 4]].join(&[][..])

// Vec<String> Join(&str)
let va = vec!["1","2","3"];
// 1-2-3
va.join("-");
```

##### golang
```golang
import "strings"
var strArr = [...]string{"a", "b"};
strings.Join(strArr, ",");
```

##### c++
```c++
// #include <sstream>

std::stringstream ss;
int len = sizeof a / sizeof a[0];
for(int i = 0; i < len; i ++) {
   ss << a[i] << (i < len - 1 ? ".": "");
}

```


### Map

#### 创建

##### typescript
```typescript
let map: Map<string, string> = new Map()
```

##### rust
```rust

```

##### golang
```golang
mapa := make(map[string]string)
```

##### c++
```c++
// #include <map>
typedef std::map<std::string, std::string> STR_MAP;
STR_MAP mapa;
```


#### 设置

##### typescript
```typescript
map = map.set('a', 'b');
```

##### rust
```rust

```

##### golang
```golang
mapa["a"] = "b";
```

##### c++
```c++
// 1. 直接设置，可覆盖
mapa["a"] = "b";

// 2. insert，当已存在key，则失败
// #include <utility>
std::pair<STR_MAP::iterator, bool> Pair;
Pair result = mapa.insert(STR_MAP::value_type("a", "b"));
result.second == 1;
```


#### 长度

##### typescript
```typescript
const mapalen = map.size;
```

##### rust
```rust

```

##### golang
```golang
mapalen := len(mapa);
```

##### c++
```c++
int mapalen = mapa.size();
```


#### 读取

##### typescript
```typescript
const mapValue: string = map.get('a');
```

##### rust
```rust

```

##### golang
```golang
mapa["a"]
```

##### c++
```c++
// 1. 直接读取
mapa["a"]
// 2. 通过迭代器来读取
MAP_STR::iterator ite = mapa.find("c");
if (ite != mapa.end()) {
  std::string key = ite->first;
  std::string value = ite->second;
}

```


#### 删除

##### typescript
```typescript
map.delete('a');
```

##### rust
```rust

```

##### golang
```golang
delete(mapa, "a");
```

##### c++
```c++

// 1. 用关键字刪除
// 如果刪除了會返回1，否則返回0
int n = mapa.erase("a");

// 2. 用迭代器刪除
MAP_STR::iterator iter = mapa.find("c");
mapa.erase(iter);

```


#### 遍历

##### typescript
```typescript
for (let [key, value] of map) {
   // do something
}
```

##### rust
```rust

```

##### golang
```golang
for key, value := range mapa {}
```

##### c++
```c++
MAP_STR::iterator iter = mapa.begin();
while(iter != mapa.end()) {
  std::string key = ite->first;
  std::string value = ite->second;
  iter ++;
}

```


### 结构

#### 定义

##### typescript
```typescript
interface Obj {
  name: number;
};
let obj: Obj = {
  name: 123
};
```

##### rust
```rust

```

##### golang
```golang
obj := struct {
  name int
}{ 123 };
```

##### c++
```c++
struct STR {
  std::string name;
  int age;
};

struct STR obj = { "test", 20 };
// 也可以忽略struct 和 等号
STR obj { "test", 20 };

```


#### 获取属性值

##### typescript
```typescript
const objKeys: string[] = Object.keys(obj);

```

##### rust
```rust

```

##### golang
```golang
t := reflect.TypeOf(obj);
for i := 0; i < t.NumField(); i++ {
  t.Field(i).Name;
}
```

##### c++
```c++
// 不支持

```


### 正则表达式

#### 定义

##### typescript
```typescript
const reg: RegExp = /a(b)/i;
```

##### rust
```rust

```

##### golang
```golang
reg, _ := regexp.Compile("(?i)a");
```

##### c++
```c++
// #include <regex>
std::regex reg("a", std::regex_constants::icase);
```


#### 是否匹配

##### typescript
```typescript
const isMatch: boolean = reg.test('A');
```

##### rust
```rust

```

##### golang
```golang
// 1. 直接匹配
match, _ := regexp.MatchString("(?i)a","A");
// 2. 先定义，后匹配
reg, _ := regexp.Compile("(?i)a");
match, _ := reg.MatchString("A");
```

##### c++
```c++
std::string s = "A";
bool match = std::regex_search(s, reg);
```


#### 匹配结果

##### typescript
```typescript
let matchRes = reg.exec('abcd');
let [match, capture1] = matchRes;
// match === 'ab'
// capture1 === 'b'
let matchIndex: number = matchRes.index;
// matchIndex === 0

```

##### rust
```rust

```

##### golang
```golang
reg, _ := regexp.Compile("a(b)");
res, _ := reg.FindStringSubmatch("abcd");
match := res[0];
// match === 'ab'
capture1 := res[1];
// capture1 === 'b'
indexRes := reg.FindStringIndex("abcd");
index := indexRes[0]
// index === 0

```

##### c++
```c++
std::smatch submatch;
std::regex_match(s, submatch, reg);
std::ssub_match matchRes = submatch[0];
std::string match = matchRes.str();
// match === 'ab'
std::ssub_match capture1Res = submatch[1];
std::string capture1Res = capture1Res.str();
// capture1 === 'b'
```


### 枚举


## 语句

### 条件

##### typescript
```typescript
if (true) {
} else if (true) {
} else { }
```

##### golang
```golang
if true {}
else if true {}
else { }
```

##### c++
```c++



```


### 循环

#### for

##### typescript
```typescript
// 普通for循环
for(let i = 0; i<10; i++) { }
// 仅判断
for(; Math.random()<0.5;) {}
// 无限循环
for(;;) {}
// 遍历索引
for(let index in arr) { }
// 遍历元素
for(let value of arr) {}
```

##### rust
```rust

```

##### golang
```golang
// 普通for循环
for i:= 0; i< 10; i++ { }
// 仅判断
for i<10 {}
for ;i<10; {}
// 无限循环
for {}
// 遍历索引
for index, _ := range array {}
// 遍历元素
for _, value := range array {}
```

##### c++
```c++
// 普通for循环

// 仅判断

// 无限循环

// 遍历索引

// 遍历元素

```


#### while

##### typescript
```typescript
while(Math.random()<0.5) {}
```

##### rust
```rust

```

##### golang
```golang
for i<10 {}
```

##### c++
```c++

```


### 选择

#### switch

##### typescript
```typescript
switch(value) {
  case 1:
    break; // 退出需要break
  case 2:
  case 3:
    break;
}
```

##### rust
```rust

```

##### golang
```golang
switch value {
  case 1:  // 退出 不需要 break
  case 2:
    fallthrough; // 继续下一个case
  case 3:
}
```

##### c++
```c++







```


### 错误捕获

##### typescript
```typescript
try {
  throw new Error('msg');
} catch(err) {
  // 错误处理
}

```

##### golang
```golang
value, err := func();
if err != nil {
  // 错误处理
}

```

##### c++
```c++






```


## 方法与类

### 方法

#### 普通方法

##### typescript
```typescript
function sum(a: number, b: number) {
  return a + b;
}
```

##### rust
```rust

```

##### golang
```golang
func sum(a int, b int) int {
  return a + b;
}
```

##### c++
```c++



```


#### 匿名方法

##### typescript
```typescript
let sumFun = (a: number, b: number) => {
  return a + b;
}
```

##### rust
```rust

```

##### golang
```golang
sum := (a int, b int) int {
  return a + b;
}
```

##### c++
```c++



```


#### IIFE

##### typescript
```typescript
((a: number, b: number) => {
  return a + b;
})(1, 2);

```

##### rust
```rust

```

##### golang
```golang
(a int, b int) int {
  return a + b;
}(1, 2);

```

##### c++
```c++




```


### 类

#### 声明

##### typescript
```typescript
class A {
  private a: number;
  constructor(a: number) {
    this.a = a;
  }
  public method() {}
  private log() {}
}
```

##### rust
```rust

```

##### golang
```golang
type A struct {
  a number
}
func (a *A) Method {}
func (a *A) log { }
```

##### c++
```c++








```


#### 继承

##### typescript
```typescript
class B extends A {
  public method() {
    super.method();
  }
}
```

##### rust
```rust

```

##### golang
```golang
type B struct {
  A
}
func (b *B) method() {
  b.A.method();
}
```

##### c++
```c++





```


#### 实例化

##### typescript
```typescript
let instance: A = new A(123);

```

##### rust
```rust

```

##### golang
```golang
b := B{
  A: A{
    a: 123
  }
};
```

##### c++
```c++


```


## 数据处理

### JSON

#### 定义

##### typescript
```typescript
`{
  "number": 1,
  "boolean": false,
  "string": "str",
  "arr": [
      12,
      "abc"
  ],
  "obj": {
      "name": "name",
      "age": 12
  }
}`
```

##### rust
```rust

```

##### golang
```golang
type JStruct struct {
  Number  int   `json:"number"`
  Boolean bool  `json:"boolean"`
  String  string  `json:"string"`
  Arr []interface{} `json:"arr"`
  Obj struct {
    Name string `json:"name"`
    Age  int  `json:"age"`
  } `json:"obj"`
}
```

##### c++
```c++













```


#### 编码

##### typescript
```typescript
const jsonStr: string = JSON.stringify({
  name: 123
});
```

##### rust
```rust

```

##### golang
```golang
import "encoding/json"
a := struct {
  Name int `json:name`
}{123}
jsonStr, _ := json.Marshal(a)
```

##### c++
```c++



```


#### 解码

##### typescript
```typescript
const jsonObj = JSON.parse('{"name": 123}');

```

##### rust
```rust

```

##### golang
```golang
str := `'{"name": 123}`;
strBytes := []btye(str);
var jsonStruct JStruct;
json.Unmarshal(strBytes, &jsonStruct);

```

##### c++
```c++


```


### 时间

#### 当前时间戳

##### typescript
```typescript
const now: number = Date.now();
```

##### rust
```rust

```

##### golang
```golang
import "time"
time.Now().Unix();
```

##### c++
```c++

```


#### 获取时间

##### typescript
```typescript
const date = new Date();
date.getFullYear(); // 年
date.getMonth() + 1; // 月
date.getDate(); // 日
date.getHours(); // 时
date.getMinutes(); // 分
date.getSeconds(); // 秒

```

##### rust
```rust

```

##### golang
```golang
import "time"
now := time.Now();
now.Year(); // 年
int(now.Month()); // 月
now.Day(); // 日
now.Hour(); //时
now.Minute(); // 分
int(now.Second()); // 秒，有小数

```

##### c++
```c++








```


### 系统变量

#### 当前代码文件

##### typescript
```typescript
__filename
```

##### rust
```rust

```

##### golang
```golang
import "runtime"
_, filename, _, _ := runtime.Caller(1)
```

##### c++
```c++

```


#### 当前代码目录

##### typescript
```typescript
__dirname
```

##### rust
```rust

```

##### golang
```golang
import (
  "runtime"
  "path"
)
_, filename, _, _ := runtime.Caller(1)
dirname := path.Dir(filename)
```

##### c++
```c++

```


#### 进程启动路径

##### typescript
```typescript
process.cwd()
```

##### rust
```rust

```

##### golang
```golang
import "os"
dir, _ := os.Getwd() 
```

##### c++
```c++

```


#### 环境变量

##### typescript
```typescript
process.env.ENV_NAME
```

##### rust
```rust

```

##### golang
```golang
import "os"
os.Getenv("ENV_NAME")
```

##### c++
```c++

```


#### 启动时入参

##### typescript
```typescript
process.argv
```

##### rust
```rust

```

##### golang
```golang
import "os"
os.Args
```

##### c++
```c++

```


#### 当前进程id

##### typescript
```typescript
process.pid
```

##### rust
```rust

```

##### golang
```golang
import "os"
os.Getpid()
```

##### c++
```c++

```


#### 退出进程

##### typescript
```typescript
process.exit(signal)
```

##### rust
```rust

```

##### golang
```golang
import "os"
os.Exit(signal)
```

##### c++
```c++

```


#### 输出

##### typescript
```typescript
console.log
```

##### rust
```rust

```

##### golang
```golang
import "fmt"
fmt.Println
```

##### c++
```c++
// include <iostream>
std::cout << value << std::endl;

```
