// - 变量-单个变量
const a int = 1;
var b string = "str";
c := false;
// - 变量-多个变量
var a, b  = 1, 2;
const (
  a = 1
  b = 2
  c = 3
)
// - 注释
// 单行注释
/* 多行注释 */ 
// - 基本类型-字符串
var str string = "str";
var str = "str";
// - 基本类型-数字
var num int = 123;
// - 基本类型-浮点数
var num float32 = 1.23;
// - 基本类型-布尔值
var f bool = false;
f := false;
// - 数组-定义
// 数组
arr := [2]int{1, 2};
var arr = [...]int{1, 2};
// 切片
arr := []int{1, 2}
// - 数组-获取长度
len(arr) == 2
// - 数组-遍历
for index, value := range arr {}
// - 数组-截取
arr[0:1]
// - 数组-连接
import "strings"
var strArr = [...]string{"a", "b"};
strings.Join(strArr, ",");
// - Map-创建
mapa := make(map[string]string)
// - Map-设置
mapa["a"] = "b";
// - Map-长度
mapalen := len(mapa);
// - Map-读取
mapa["a"]
// - Map-删除
delete(mapa, "a");
// - Map-遍历
for key, value := range mapa {}
// - 结构-定义
obj := struct {
  name int
}{ 123 };
// - 结构-获取属性值
t := reflect.TypeOf(obj);
for i := 0; i < t.NumField(); i++ {
  t.Field(i).Name;
}
// - 正则表达式-定义
reg, _ := regexp.Compile("(?i)a");
// - 正则表达式-是否匹配
// 1. 直接匹配
match, _ := regexp.MatchString("(?i)a","A");
// 2. 先定义，后匹配
reg, _ := regexp.Compile("(?i)a");
match, _ := reg.MatchString("A");
// - 正则表达式-匹配结果
reg, _ := regexp.Compile("a(b)");
res, _ := reg.FindStringSubmatch("abcd");
match := res[0];
// match === 'ab'
capture1 := res[1];
// capture1 === 'b'
indexRes := reg.FindStringIndex("abcd");
index := indexRes[0]
// index === 0

// - 条件
if true {}
else if true {}
else { }
// - 循环-for
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
// - 循环-while
for i<10 {}
// - 选择-switch
switch value {
  case 1:  // 退出 不需要 break
  case 2:
    fallthrough; // 继续下一个case
  case 3:
}
// - 错误捕获
value, err := func();
if err != nil {
  // 错误处理
}

// - 方法-普通方法
func sum(a int, b int) int {
  return a + b;
}
// - 方法-匿名方法
sum := (a int, b int) int {
  return a + b;
}
// - 方法-IIFE
(a int, b int) int {
  return a + b;
}(1, 2);

// - 类-声明
type A struct {
  a number
}
func (a *A) Method {}
func (a *A) log { }
// - 类-继承
type B struct {
  A
}
func (b *B) method() {
  b.A.method();
}
// - 类-实例化
b := B{
  A: A{
    a: 123
  }
};
// - JSON-定义
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
// - JSON-编码
import "encoding/json"
a := struct {
  Name int `json:name`
}{123}
jsonStr, _ := json.Marshal(a)
// - JSON-解码
str := `'{"name": 123}`;
strBytes := []btye(str);
var jsonStruct JStruct;
json.Unmarshal(strBytes, &jsonStruct);

// - 时间-当前时间戳
import "time"
time.Now().Unix();
// - 时间-获取时间
import "time"
now := time.Now();
now.Year(); // 年
int(now.Month()); // 月
now.Day(); // 日
now.Hour(); //时
now.Minute(); // 分
int(now.Second()); // 秒，有小数

// - 系统变量-当前代码文件
import "runtime"
_, filename, _, _ := runtime.Caller(1)
// - 系统变量-当前代码目录
import (
  "runtime"
  "path"
)
_, filename, _, _ := runtime.Caller(1)
dirname := path.Dir(filename)
// - 系统变量-进程启动路径
import "os"
dir, _ := os.Getwd() 
// - 系统变量-环境变量
import "os"
os.Getenv("ENV_NAME")
// - 系统变量-启动时入参
import "os"
os.Args
// - 系统变量-当前进程id
import "os"
os.Getpid()
// - 系统变量-退出进程
import "os"
os.Exit(signal)
// - 系统变量-输出
import "fmt"
fmt.Println