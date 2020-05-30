## 引入包
### 方式
- 使用 :: use包中的内容

### 理由
- **加快编译速度**
	- 因为对象中方法的调用将使用 . 号调用, 如果包的导入也是使用 . 号, 那么在编译的时候, 无法直接得知是要引入包中的内容, 还是要调用对象中的方法, 这种情况下, 有两种解决方案:
		- 第一种方式: 在解析到import之后, 将包的名称放在内存中, 在遇到 . 号时, 在内存中查找是否是包, 如果是包就说明用户想要引入包中的内容, 而不是调用对象中的方法; 这种方式存在一个问题: 用户将无法在作用域中定义与包名相同的对象名, 举个例子:
			```
			1. import "system:std/vector"
			2. 
			3. fn main() {
			4. 	   var vector = [1];
			5. 	   let v = vector.push(2);
			6. }
			```
			以上代码中, 第1行引入了一个包vector; 第4行定义了一个 vector 变量; 在第5行, 用户其实是想调用vector对象中的push方法, 但是由于编译器在读取到 import 时, 已经将 vector 当作包存在内存中了, 解析到第5行时, 会认为vector是包名, 那么将去 std/vector 中查找 push 的定义, 而 std/vector 中并不存在 push, 则将会报错.
		- 另外一种方式: 遇到 . 号后, 判断 . 号之前的token在当前作用域是否存在, 如果不存在, 就递归向上查找, 直到根作用域, 如果到了根作用域, 还是没有找到, 那么就认为是包导入; 这种方式很明显的一个问题就是查找效率太低, 如果嵌套很深, 查找效率是很低的.
- **代码整洁**
	- 这一点比较明显, 看代码的时候, 看到 :: 就知道是 包导入
