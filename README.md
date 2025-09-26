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

##### golang
```golang
const a int = 1;
var b string = "str";
c := false;
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

##### c++11
```c++11
const int a = 123;
int a(123);
int a{123}; // C++11 brace initialization
int a = {123};
char b[] = "str";
bool c = false;
```

##### c++23
```c++23
const int a = 123;
int a{123}; // C++23 prefers brace initialization
auto b = "str"; // C++23 auto deduction
bool c = false;
```


#### 多个变量

##### typescript
```typescript
let d: number = 1, e: number = 2;
const f: number = 1, g: number = 2;
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

##### rust
```rust
let (a, b, c) = (42, "hello", 5.0);
```

##### c++11
```c++11
int d = 1, e = 2;
const int f = 1, g = 2;

```

##### c++23
```c++23
auto [d, e] = std::make_pair(1, 2); // C++17+ structured bindings
const auto [f, g] = std::make_pair(1, 2);

```


### 注释

##### typescript
```typescript
// 单行注释
/* 多行注释 */ 
```

##### golang
```golang
// 单行注释
/* 多行注释 */ 
```

##### rust
```rust
// 单行注释
/// 三个反斜也可以
/* 多行注释 */ 
```

##### c++11
```c++11
// 单行注释
/* 多行注释 */ 
```

##### c++23
```c++23
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

##### golang
```golang
var str string = "str";
var str = "str";
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

##### c++11
```c++11
char str[4] = {"s", "t", "r", "\0"};
char str[] = "str";
// #include <string>
std::string str = "str";

```

##### c++23
```c++23
using namespace std::string_literals;
std::string str = "str"s;
auto str2 = "str"sv; // string_view

```


#### 数字

##### typescript
```typescript
let num: number = 123;
```

##### golang
```golang
var num int = 123;
```

##### rust
```rust
let num = 123; // i32
let num64: i64 = 123;
let float64 = 2.0; // f64
let float32: f32 = 3.0; // f32
```

##### c++11
```c++11
int num = 123;
```

##### c++23
```c++23
int num = 123;
auto num2 = 123; // C++23 auto
```


#### 布尔值

##### typescript
```typescript
const bf: boolean = false;
```

##### golang
```golang
var f bool = false;
f := false;
```

##### rust
```rust
let c = true;
let d: bool = false;
```

##### c++11
```c++11
bool f = false;
```

##### c++23
```c++23
bool f = false;
auto bf = false; // C++23 auto

```


### 数组

#### 定义

##### typescript
```typescript
let arr: Array<number> = [1, 2];
```

##### golang
```golang
// 数组
arr := [2]int{1, 2};
var arr = [...]int{1, 2};
// 切片
arr := []int{1, 2}
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

##### c++11
```c++11
int arr[2] = {1, 2};
int arr[] = {1, 2}; // C++11 style
```

##### c++23
```c++23
std::array<int, 2> arr{1, 2}; // C++23 prefers std::array
std::vector<int> vec{1, 2}; // C++23 modern approach
```


#### 获取长度

##### typescript
```typescript
arr.length === 2
```

##### golang
```golang
len(arr) == 2
```

##### rust
```rust
arr.len() == 2
```

##### c++11
```c++11
sizeof arr / sizeof(arr[0]) == 2;
// C++11
int length = std::end(arr) - std::begin(arr);
```

##### c++23
```c++23
arr.size() == 2;
vec.size() == 2;
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

##### golang
```golang
for index, value := range arr {
  // do something
}
```

##### rust
```rust
// 迭代值
for val in arr.iter(){
    println!("value is :{}", val);
}
// 迭代索引
for index in 0..arr.len() {
    println!("index is: {} & value is : {}",index, arr[index]);
}
// 迭代值和索引
for (index, value) in arr.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
}
// 内联
let index_value_pairs: Vec<String> = arr
        .iter()
        .enumerate()
        .map(|(index, value)| format!("Index: {}, Value: {}", index, value))
        .collect();
// 遍历 vector 数组
for i in &mut varr {
    println!("{}", i);
}
```

##### c++11
```c++11
int len = sizeof arr / sizeof arr[0];
for (int index = 0; index < len; index ++) {
  int value = arr[index];
}
// C++11 range-based for loop
for (auto value : arr) {}

```

##### c++23
```c++23
for (const auto& value : arr) {
  // do something
}
// C++20+ ranges
for (auto&& value : arr | std::views::filter([](int x) { return x > 0; })) {
  // filtered iteration
}

```


#### 截取

##### typescript
```typescript
arr.slice(0, 1)
```

##### golang
```golang
arr[0:1]
```

##### rust
```rust
&arr[0..1];
// 包含结尾元素需要加 =
&arr[0..=0];
```

##### c++11
```c++11
int a[1];
for (int index = 0, aIndex = 0; index < 1; index ++, aIndex ++) {
  a[aIndex] = arr[index];
}
```

##### c++23
```c++23
// C++23 with ranges
auto sliced = arr | std::views::take(1);
// Traditional approach
std::vector<int> a(arr.begin(), arr.begin() + 1);

```


#### 连接

##### typescript
```typescript
var strArr: string[] = ['a', 'b'];
strArr.join(',')
```

##### golang
```golang
import "strings"
var strArr = [...]string{"a", "b"};
strings.Join(strArr, ",");
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

##### c++11
```c++11
// #include <sstream>
std::stringstream ss;
int len = sizeof a / sizeof a[0];
for(int i = 0; i < len; i ++) {
   ss << a[i] << (i < len - 1 ? ".": "");
}

```

##### c++23
```c++23
// C++23 with ranges and format
std::string result = std::format("{}", fmt::join(arr, "."));
// Alternative with stringstream
std::stringstream ss;
std::copy(arr.begin(), arr.end(), std::ostream_iterator<int>(ss, "."));

```


### Map

#### 创建

##### typescript
```typescript
let map: Map<string, string> = new Map()
```

##### golang
```golang
mapa := make(map[string]string)
```

##### rust
```rust
use std::collections::HashMap;
let mut map = HashMap::new();
// 或者也可以通过 array 来初始化
let mut map2 = HashMap::from([
    ("key1", "value1"),
    ("key2", "value2"),
])
```

##### c++11
```c++11
// #include <map>
std::map<std::string, std::string> mapa;
// C++11 initializer list
std::map<std::string, std::string> mapa = {{"key1", "value1"}, {"key2", "value2"}};
```

##### c++23
```c++23
std::unordered_map<std::string, std::string> mapa{
  {"key1", "value1"}, 
  {"key2", "value2"}
}; // C++23 prefers unordered_map for performance
```


#### 设置

##### typescript
```typescript
map = map.set('a', 'b');
```

##### golang
```golang
mapa["a"] = "b";
```

##### rust
```rust
map.insert(
    "key".to_string(),
    "value"
);
```

##### c++11
```c++11
// 1. 直接设置，可覆盖
mapa["a"] = "b";
// 2. insert，当已存在key，则失败
// #include <utility>
auto result = mapa.insert(std::make_pair("a", "b"));
result.second == true;
```

##### c++23
```c++23
mapa["a"] = "b";
mapa.insert_or_assign("a", "b"); // C++17+
```


#### 长度

##### typescript
```typescript
const mapalen = map.size;
```

##### golang
```golang
mapalen := len(mapa);
```

##### rust
```rust
map.len();
```

##### c++11
```c++11
int mapalen = mapa.size();
```

##### c++23
```c++23
auto mapSize = mapa.size();
```


#### 读取

##### typescript
```typescript
const mapValue: string = map.get('a');
```

##### golang
```golang
value, existed := mapa["a"]
```

##### rust
```rust
// get 的参数类型为 &Q
if let Some(value) = map.get(&"key".to_string()) {
    // do something
}
map["key"];
map[&"key".to_string()];
```

##### c++11
```c++11
// 1. 直接读取
mapa["a"];
// 2. 通过迭代器来读取
auto ite = mapa.find("c");
if (ite != mapa.end()) {
  std::string key = ite->first;
  std::string value = ite->second;
}

```

##### c++23
```c++23
auto value = mapa["a"];
// C++23 modern approach with optional
if (auto it = mapa.find("c"); it != mapa.end()) {
  auto [key, value] = *it; // C++17+ structured bindings
}

```


#### 删除

##### typescript
```typescript
map.delete('a');
```

##### golang
```golang
delete(mapa, "a");
```

##### rust
```rust
map.remove(&"key".to_string());
```

##### c++11
```c++11
// 1. 用关键字刪除
int n = mapa.erase("a");
// 2. 用迭代器刪除
auto iter = mapa.find("c");
mapa.erase(iter);

```

##### c++23
```c++23
mapa.erase("a");
```


#### 遍历

##### typescript
```typescript
for (let [key, value] of map) {
   // do something
}
```

##### golang
```golang
for key, value := range mapa {}
```

##### rust
```rust
for (key, value) in &map {
    println!("{}: \"{}\"", key, value);
}
```

##### c++11
```c++11
for (auto iter = mapa.begin(); iter != mapa.end(); ++iter) {
  std::string key = iter->first;
  std::string value = iter->second;
}
// C++11 range-based for loop
for (const auto& pair : mapa) {
  std::string key = pair.first;
  std::string value = pair.second;
}

```

##### c++23
```c++23
for (const auto& [key, value] : mapa) { // C++17+ structured bindings
  // do something
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

##### golang
```golang
obj := struct {
  name int
}{ 123 };
```

##### rust
```rust
struct StructItem {
    str: String,
    i: u32
}
let str = String::from("123");
let s = StructItem {
    str,
    i: 123
}
let s2 = StructItem {
    i: 124,
    ..s
}
// 元祖结构体
struct T1(u32, u32)；
let t = T1(1,2);
```

##### c++11
```c++11
struct STR {
  std::string name;
  int age;
};

struct STR obj = { "test", 20 };
// C++11 uniform initialization
STR obj { "test", 20 };

```

##### c++23
```c++23
struct STR {
  std::string name;
  int age;
  
  // C++20+ spaceship operator
  auto operator<=>(const STR&) const = default;
};

STR obj{"test", 20}; // C++23 aggregate initialization

```


#### 获取属性值

##### typescript
```typescript
const objKeys: string[] = Object.keys(obj);

```

##### golang
```golang
t := reflect.TypeOf(obj);
for i := 0; i < t.NumField(); i++ {
  t.Field(i).Name;
}
```

##### rust
```rust
s2.str; // 123
t.1; // 2
```

##### c++11
```c++11
obj.name; // "test"
obj.age;  // 20

```

##### c++23
```c++23
obj.name; // "test"
obj.age;  // 20

```


### 正则表达式

#### 定义

##### typescript
```typescript
const reg: RegExp = /a(b)/i;
```

##### golang
```golang
reg, _ := regexp.Compile("(?i)a");
```

##### rust
```rust
// 需要添加依赖 regex = "1"
use regex::Regex;
let reg = Regex::new(r"(a|b)").unwrap();

```

##### c++11
```c++11
// #include <regex>
std::regex reg("(a|b)", std::regex_constants::icase);
```

##### c++23
```c++23
std::regex reg{R"((a|b))", std::regex_constants::icase};
```


#### 是否匹配

##### typescript
```typescript
const isMatch: boolean = reg.test('A');
```

##### golang
```golang
// 1. 直接匹配
match, _ := regexp.MatchString("(?i)a","A");
// 2. 先定义，后匹配
reg, _ := regexp.Compile("(?i)a");
match, _ := reg.MatchString("A");
```

##### rust
```rust
let is_match = reg.is_match("A");

```

##### c++11
```c++11
std::string s = "A";
bool match = std::regex_search(s, reg);
```

##### c++23
```c++23
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

##### rust
```rust
let text = "abcd";
if let Some(caps) = reg.captures(text) {
    let full_match = &caps[0]; // "ab"
    let capture1 = &caps[1]; // "a"
}
```

##### c++11
```c++11
std::smatch submatch;
std::regex_search(s, submatch, reg);
std::ssub_match matchRes = submatch[0];
std::string match = matchRes.str();
// match === 'A'
```

##### c++23
```c++23
std::smatch submatch;
std::regex_search(s, submatch, reg);
auto matchRes = submatch[0];
std::string match = matchRes.str();

```


### 枚举

##### rust
```rust
enum Color {
    Red,
    Green,
    Blue,
}
// 带值的枚举
enum Status {
    Success = 0,
    Error = 1,
}
let color = Color::Red;
let status = Status::Success;

```

##### c++11
```c++11
enum class Color : int {
  RED = 1,
  GREEN = 2,
  BLUE = 3
};
Color color = Color::RED;

```

##### c++23
```c++23
enum class Color : int {
  RED = 1,
  GREEN = 2,
  BLUE = 3
};
Color color = Color::RED;
// C++23 enhanced enums
using enum Color;
auto red = RED; // Can use unqualified name

```


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

##### rust
```rust
// 条件可以忽略小括号
if a > 0 {
} else if (a < 0) {
} else {
}
// 还能赋值，等同三元运算符
let b = if (a > 0) { 1 } else { 0 }
```

##### c++11
```c++11
if (true) {
} else if (false) {
} else { }

```

##### c++23
```c++23
if (true) {
} else if (false) {
} else { }
// C++17+ if with initializer
if (auto value = getValue(); value > 0) {
  // use value
}

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

##### rust
```rust
let a = [10, 20, 30, 40, 50];
for i in 0..5 {
    println!("a[{}] = {}", i, a[i]);
}
for i in a.iter() {
    println!("a[{}] = {}", i, a[i]);
}
```

##### c++11
```c++11
// 普通for循环
for(int i = 0; i < 10; i++) { }
// 仅判断
for(; rand() % 2 == 0;) {}
// 无限循环
for(;;) {}
// 遍历索引
int len = sizeof arr / sizeof arr[0];
for(int index = 0; index < len; index++) { }
// 遍历元素 (C++11)
for(auto value : arr) {}

```

##### c++23
```c++23
// Range-based for loop (preferred in C++23)
for (const auto& item : container) { }
// Traditional for loop
for (int i = 0; i < 10; i++) { }
// C++20+ ranges
for (auto i : std::views::iota(0, 10)) { }

```


#### while

##### typescript
```typescript
while(Math.random()<0.5) {}
```

##### golang
```golang
for i<10 {}
```

##### rust
```rust
let mut number = 1;
while number != 4 {
    number += 1;
}
// 无限循环

let mut i = 0;
// 支持 break 出来的值赋值
let num = loop {
    if i == 10 {
        break i;
    }
    i += 1;
};
num == 10;
```

##### c++11
```c++11
while(rand() % 2 == 0) {}

```

##### c++23
```c++23
while (condition()) {}

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

##### golang
```golang
switch value {
  case 1:  // 退出 不需要 break
  case 2:
    fallthrough; // 继续下一个case
  case 3:
}
```

##### rust
```rust
let num = 3;
match num {
    1|2=>println!("<3"),
    3=>println!("=3"),
    4..=20=> println!("4<= x <= 20"),
    _=>println!("other"),
}
```

##### c++11
```c++11
switch(value) {
  case 1:
  case 2:
    // < 3
    break;
  case 3:
    // = 3
    break;
  default:
    // other
    break;
}

```

##### c++23
```c++23
// C++17+ switch with initializer
switch (auto value = getValue(); value) {
  case 1:
  case 2:
    // < 3
    break;
  case 3:
    // = 3
    break;
  default:
    // other
    break;
}

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

##### rust
```rust
// Rust uses Result<T, E> instead of exceptions
fn might_fail() -> Result<i32, String> {
    if true {
        Ok(42)
    } else {
        Err("Something went wrong".to_string())
    }
}

match might_fail() {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}

```

##### c++11
```c++11
try {
  // code that might throw
} catch (const std::exception& e) {
  // handle exception
} catch (...) {
  // handle any other exception
}

```

##### c++23
```c++23
try {
  // code that might throw
} catch (const std::exception& e) {
  // C++23 format
  std::cout << std::format("Error: {}\n", e.what());
}

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

##### golang
```golang
func sum(a int, b int) int {
  return a + b;
}
```

##### rust
```rust
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

##### c++11
```c++11
int sum(int a, int b) {
  return a + b;
}

```

##### c++23
```c++23
auto sum(int a, int b) -> int { // C++23 trailing return type
  return a + b;
}
// C++20+ concepts
template<std::integral T>
auto sum(T a, T b) -> T {
  return a + b;
}

```


#### 匿名方法

##### typescript
```typescript
let sumFun = (a: number, b: number) => {
  return a + b;
}
```

##### golang
```golang
sum := (a int, b int) int {
  return a + b;
}
```

##### rust
```rust
// 闭包
let sum = |x: i32, y: i32| -> i32 {
    x + y
};
```

##### c++11
```c++11
// C++11 lambda
auto sumFun = [](int a, int b) -> int {
  return a + b;
};

```

##### c++23
```c++23
auto sumFun = [](int a, int b) -> int {
  return a + b;
};
// C++23 with auto parameters
auto sumFun2 = [](auto a, auto b) {
  return a + b;
};

```


#### IIFE

##### typescript
```typescript
((a: number, b: number) => {
  return a + b;
})(1, 2);

```

##### golang
```golang
(a int, b int) int {
  return a + b;
}(1, 2);

```

##### rust
```rust
// Rust doesn't have traditional IIFE, but can use closures
let result = (|a: i32, b: i32| -> i32 {
    a + b
})(1, 2);

```

##### c++11
```c++11
// C++11 lambda IIFE
int result = [](int a, int b) -> int {
  return a + b;
}(1, 2);

```

##### c++23
```c++23
auto result = [](int a, int b) -> int {
  return a + b;
}(1, 2);
// C++23 with immediate invocation
auto result2 = []<typename T>(T a, T b) {
  return a + b;
}.template operator()<int>(1, 2);

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

##### golang
```golang
type A struct {
  a number
}
func (a *A) Method {}
func (a *A) log { }
```

##### rust
```rust
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Person {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

```

##### c++11
```c++11
class A {
private:
  int value;
public:
  A(int v) : value(v) {}
  int getValue() const { return value; }
  void setValue(int v) { value = v; }
};

```

##### c++23
```c++23
class A {
private:
  int value_;
public:
  explicit A(int v) : value_(v) {}
  [[nodiscard]] auto getValue() const noexcept -> int { return value_; }
  auto setValue(int v) noexcept -> void { value_ = v; }
  
  // C++20+ spaceship operator
  auto operator<=>(const A&) const = default;
};

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

##### golang
```golang
type B struct {
  A
}
func (b *B) method() {
  b.A.method();
}
```

##### rust
```rust
// Rust doesn't have traditional inheritance, uses traits instead
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

```

##### c++11
```c++11
class B : public A {
public:
  B(int v) : A(v) {}
  void doSomething() { }
};

```

##### c++23
```c++23
class B : public A {
public:
  explicit B(int v) : A(v) {}
  auto doSomething() -> void { }
};

```


#### 实例化

##### typescript
```typescript
let instance: A = new A(123);

```

##### golang
```golang
b := B{
  A: A{
    a: 123
  }
};
```

##### rust
```rust
let person = Person::new("Alice".to_string(), 30);
let dog = Dog { name: "Buddy".to_string() };

```

##### c++11
```c++11
A instance(123);
A* instancePtr = new A(123);

```

##### c++23
```c++23
auto instance = A{123}; // C++23 preferred style
std::unique_ptr<A> instancePtr = std::make_unique<A>(123);

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

##### rust
```rust
// 需要添加依赖 serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonData {
    name: String,
    values: Vec<i32>,
    obj: JsonObject,
}

#[derive(Serialize, Deserialize)]
struct JsonObject {
    name: String,
    age: i32,
}

```

##### c++11
```c++11
// C++ doesn't have built-in JSON support
// Need external library like nlohmann/json
// Example with manual string creation:
std::string jsonStr = R"({
  "name": "example",
  "values": [1, 2, 3],
  "obj": {
    "name": "name",
    "age": 12
  }
})";

```

##### c++23
```c++23
// C++23 still requires external library like nlohmann/json
// Using raw string literal
auto jsonStr = R"({
  "name": "example",
  "values": [1, 2, 3],
  "obj": {
    "name": "name",
    "age": 12
  }
})"s;

```


#### 编码

##### typescript
```typescript
const jsonStr: string = JSON.stringify({
  name: 123
});
```

##### golang
```golang
import "encoding/json"
a := struct {
  Name int `json:name`
}{123}
jsonStr, _ := json.Marshal(a)
```

##### rust
```rust
use serde_json;
let data = JsonData {
    name: "example".to_string(),
    values: vec![1, 2, 3],
    obj: JsonObject {
        name: "name".to_string(),
        age: 12,
    },
};
let json_str = serde_json::to_string(&data).unwrap();

```

##### c++11
```c++11
// Using manual string construction
std::ostringstream oss;
oss << "{\"name\":" << 123 << "}";
std::string jsonStr = oss.str();

```

##### c++23
```c++23
// C++23 with format library
auto jsonStr = std::format(R"({{"name":{}}})", 123);

```


#### 解码

##### typescript
```typescript
const jsonObj = JSON.parse('{"name": 123}');

```

##### golang
```golang
str := `'{"name": 123}`;
strBytes := []btye(str);
var jsonStruct JStruct;
json.Unmarshal(strBytes, &jsonStruct);

```

##### rust
```rust
let json_str = r#"{"name": 123}"#;
let value: serde_json::Value = serde_json::from_str(json_str).unwrap();

```

##### c++11
```c++11
// Manual parsing required or use library
// Example would require external JSON library

```

##### c++23
```c++23
// Still requires external library
// nlohmann::json j = nlohmann::json::parse(jsonStr);

```


### 时间

#### 当前时间戳

##### typescript
```typescript
const now: number = Date.now();
```

##### golang
```golang
import "time"
time.Now().Unix();
```

##### rust
```rust
use std::time::{SystemTime, UNIX_EPOCH};
let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

```

##### c++11
```c++11
// #include <chrono>
auto now = std::chrono::system_clock::now();
auto timestamp = std::chrono::duration_cast<std::chrono::milliseconds>(
  now.time_since_epoch()).count();

```

##### c++23
```c++23
auto now = std::chrono::system_clock::now();
auto timestamp = std::chrono::duration_cast<std::chrono::milliseconds>(
  now.time_since_epoch()).count();

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

##### rust
```rust
use chrono::{DateTime, Local};
let now: DateTime<Local> = Local::now();
let year = now.year();
let month = now.month();
let day = now.day();
let hour = now.hour();
let minute = now.minute();
let second = now.second();

```

##### c++11
```c++11
// #include <ctime>
std::time_t t = std::time(nullptr);
std::tm* tm = std::localtime(&t);
int year = tm->tm_year + 1900;
int month = tm->tm_mon + 1;
int day = tm->tm_mday;
int hour = tm->tm_hour;
int minute = tm->tm_min;
int second = tm->tm_sec;

```

##### c++23
```c++23
// C++20+ chrono improvements
auto now = std::chrono::system_clock::now();
auto time_t = std::chrono::system_clock::to_time_t(now);
auto tm = *std::localtime(&time_t);
auto year = tm.tm_year + 1900;
auto month = tm.tm_mon + 1;
auto day = tm.tm_mday;
auto hour = tm.tm_hour;
auto minute = tm.tm_min;
auto second = tm.tm_sec;

```


### 系统变量

#### 当前代码文件

##### typescript
```typescript
__filename
```

##### golang
```golang
import "runtime"
_, filename, _, _ := runtime.Caller(1)
```

##### rust
```rust
file!()

```

##### c++11
```c++11
__FILE__

```

##### c++23
```c++23
std::source_location::current().file_name(); // C++20+

```


#### 当前代码目录

##### typescript
```typescript
__dirname
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

##### rust
```rust
// 需要 std::env
use std::env;
let current_dir = env::current_dir().unwrap();

```

##### c++11
```c++11
// Need to use filesystem library or platform-specific code

```

##### c++23
```c++23
std::filesystem::current_path(); // C++17+

```


#### 进程启动路径

##### typescript
```typescript
process.cwd()
```

##### golang
```golang
import "os"
dir, _ := os.Getwd() 
```

##### rust
```rust
let current_dir = env::current_dir().unwrap();

```

##### c++11
```c++11
// #include <unistd.h>
char cwd[1024];
getcwd(cwd, sizeof(cwd));

```

##### c++23
```c++23
std::filesystem::current_path();

```


#### 环境变量

##### typescript
```typescript
process.env.ENV_NAME
```

##### golang
```golang
import "os"
os.Getenv("ENV_NAME")
```

##### rust
```rust
let env_var = env::var("ENV_NAME").unwrap_or_default();

```

##### c++11
```c++11
// #include <cstdlib>
const char* env = std::getenv("ENV_NAME");

```

##### c++23
```c++23
// C++23 with expected
std::expected<std::string, int> getEnv(const std::string& name) {
  if (auto env = std::getenv(name.c_str())) {
    return std::string{env};
  }
  return std::unexpected{-1};
}

```


#### 启动时入参

##### typescript
```typescript
process.argv
```

##### golang
```golang
import "os"
os.Args
```

##### rust
```rust
use std::env;
let args: Vec<String> = env::args().collect();

```

##### c++11
```c++11
// int main(int argc, char* argv[])
// argc, argv

```

##### c++23
```c++23
// int main(int argc, char* argv[])
// std::span<char*> args{argv, static_cast<size_t>(argc)}; // C++20+

```


#### 当前进程id

##### typescript
```typescript
process.pid
```

##### golang
```golang
import "os"
os.Getpid()
```

##### rust
```rust
use std::process;
let pid = process::id();

```

##### c++11
```c++11
// #include <unistd.h>
pid_t pid = getpid();

```

##### c++23
```c++23
// Still platform-specific, no standard solution

```


#### 退出进程

##### typescript
```typescript
process.exit(signal)
```

##### golang
```golang
import "os"
os.Exit(signal)
```

##### rust
```rust
process::exit(0);

```

##### c++11
```c++11
// #include <cstdlib>
std::exit(0);

```

##### c++23
```c++23
std::exit(0);

```


#### 输出

##### typescript
```typescript
console.log
```

##### golang
```golang
import "fmt"
fmt.Println
```

##### rust
```rust
println!("value");
print!("value");
eprintln!("error message");
```

##### c++11
```c++11
// #include <iostream>
std::cout << "value" << std::endl;
```

##### c++23
```c++23
// C++23 with print
std::print("value\n"); // C++23
// Alternative
std::cout << "value\n";
```
