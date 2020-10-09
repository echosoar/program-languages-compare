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

##### c++
```c++
int a = 123;
char b[4] = {"s", "t", "r", "\0"};
char b[] = "str";

//#include &lt;string&gt;
std::string b = "str";

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

##### golang
```golang
var str string = "str";
var str = "str";
```

##### c++
```c++

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

##### c++
```c++

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

##### c++
```c++

```


### 数组

#### 定义

##### typescript
```typescript
let arr: Array&lt;number&gt; = [1, 2];
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

##### c++
```c++

```


#### 遍历

##### typescript
```typescript
arr.forEach((item: number, i: number) =&gt; {
  // do something
});
arr.map((item: number, i: number) =&gt; {
  return item;
});
```

##### golang
```golang
for index, value := range arr {}
```

##### c++
```c++






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

##### c++
```c++

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

##### c++
```c++


```


### Map

#### 创建

##### typescript
```typescript
let map: Map&lt;string, string&gt; = new Map()
```

##### golang
```golang
mapa := make(map[string]string)
```

##### c++
```c++

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

##### c++
```c++

```


#### 读取

##### typescript
```typescript
const mapValue: string = map.get('a');
```

##### golang
```golang
mapa["a"]
```

##### c++
```c++

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

##### c++
```c++

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

##### c++
```c++



```


### Object

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

##### c++
```c++






```


#### 获取属性值

##### typescript
```typescript
const objKeys: string[] = Object.keys(obj);

```

##### golang
```golang
t := reflect.TypeOf(obj);
for i := 0; i &lt; t.NumField(); i++ {
  t.Field(i).Name;
}
```

##### c++
```c++


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

##### c++
```c++

```


#### 是否匹配

##### typescript
```typescript
const isMatch: boolean = reg.test('A');
```

##### golang
```golang
match, _ := regexp.MatchString("(?i)a","A");

reg, _ := regexp.Compile("(?i)a");
match, _ := reg.MatchString("A");
```

##### c++
```c++

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

##### c++
```c++


// match === 'ab'
// capture1 === 'b'

// matchIndex === 0

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

##### c++
```c++



```


### 循环

#### for

##### typescript
```typescript
// 普通for循环
for(let i = 0; i&lt;10; i++) { }
// 仅判断
for(; Math.random()&lt;0.5;) {}
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
for i:= 0; i&lt; 10; i++ { }
// 仅判断
for i&lt;10 {}
for ;i&lt;10; {}
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
while(Math.random()&lt;0.5) {}
```

##### golang
```golang
for i&lt;10 {}
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
let sumFun = (a: number, b: number) =&gt; {
  return a + b;
}
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
((a: number, b: number) =&gt; {
  return a + b;
})(1, 2);

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

##### golang
```golang
import "fmt"
fmt.Println
```

##### c++
```c++
// include &lt;iostream&gt;
std::cout &lt;&lt; value &lt;&lt; endl;
```
