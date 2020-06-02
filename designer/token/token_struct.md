## 概述
- 主要说明 token 结构体的内容, 字段说明
- token中必要方法的解决方案

## 结构内容
- file: 文件完整路径
	- 方便检测到错误时, 可以定位所在文件
- line: 行号
	- 记录行号为了在检测到错误时, 可以提示用户错误所在的行
- token_type: token类型

## token中涉及到的方法(nup 和 led)
- 读则这里先不要管 nup 和 led 是什么, 现在只要认为是每一个token都需要实现的两个方法; 比如说, 提取出了 + token(TokenType_Plus), 那么 TokenType_Plus 的token实例是需要实现 nup 和 led 的
- 还需要了解一点, 词法解析阶段得到的token是需要存储在一个容器中, 那么就要思考如何将不同类型的token放在同一个容器内呢
- 这里有多种做法, 下面讨论一下每一种方式的实现原理和最终的选择

### 一个Token结构, 存储了回调指针
- 类似这样(只是说明, 和实际代码不一致)

```rust
type NupCallback = fn();
type LedCallback = fn();

struct Token {
	nup: NupCallback,
	led: LedCallback
}
```

- 有的读者可能会在token对象创建的时候, 将临时的匿名函数赋值给 nulCallback / ledCallback, 如:
```rust
let t = Token{
	nup: || {
	},
	led: || {
	}
};
tokens.push(t);
```
	- 这种方式也是可以的, 但是会有性能问题, 每创建一个闭包, 编译器会创建一个对象, 这就导致每一个token对象的nup/led回调都创建一个函数对象; 那可能又有读者会问, 这和上面的全局创建的fn有什么区别呢? 词法分析是针对单词(这里的单词不是指英文单词, 这里指的是可以作为语法分析的最小单元)而言的, 而用户编码中, 必然是一种token出现过多次(非常简单的代码就不说了); 举个例子: + 号, 在用户编码过程中, 会出现过很多次
		- 全局的fn
			- 每一个 + 号token都共享同一块函数定义的地址
		- 闭包fn
			- 每一个 + 号token都会被创建函数对象
	- 计算一下如果有100个 + 号token, 全局的fn, 之消耗了一份内存, 而闭包fn将消耗 100份内存
	
### 多个Token结构, 每一个结构都实现了同样的必要函数
- 像这样(只是说明, 和实际代码不一致)

```rust
trait Token {
	fn nup(&self);
	fn led(&self);
}

// + 号 token
struct PlusToken {
}

// id token
struct IdToken {
}

impl Token for PlusToken {
	fn nup(&self) {
	}
	fn led(&self) {
	}
}

impl Token for IdToken {
	fn nup(&self) {
	}
	fn led(&self) {
	}
}
```

- 这种方式就是类似设计模式中模板方法模式(具体做什么由子类决定)
- **注意**: 这里需要重点注意一下rust中, 如何将trait放入到Vec中(因为这个用法和编译器原理有一定的关系, 需要花一点篇幅来介绍)
	- 相信有面向对象的读者第一想法就是如下实现方式
	```rust
	1 trait Token {
    2     fn nup(&self);
    3     fn led(&self);
    4 }
    5 
    6 struct PlusToken {
    7 }
    8 
    9 impl Token for PlusToken {
   10     fn nup(&self) {
   11     }
   12     fn led(&self) {
   13     }
   14 }
   15 
   16 fn main() {
   17     Vec::<dyn Token>::new();
   18 }
	```
	- 但是, 事实并没有想象中的那么顺利, 编译一下:
		![img1](images/img1.png)
		重点是这一句 *doesn't satisfy \`dyn Token: std::marker::Sized\`*, 意思是说, Vec的模板参数需要满足 std::marker::Sized 这个 trait, *std::marker::Sized* 标记的类型, 需要在编译期就要知道大小, 然而 trait 不是实体对象, 只要实现该trait的类型, 在这里都可以被指定, 那么trait很显然是无法在编译期知道大小的, 所以编译器会报错, 这个是Vec实现者强制要求的条件. 实现者强制要求这个条件, 是因为Vec是动态数组, 可以动态扩容, 如果不知道每一个元素的大小, 无法分配连续的内存
	- 那如何解决呢? 仔细想想, 既然需要固定大小, 那么就让它固定大小就好了, 什么类型一定是固定大小的呢, 那就是指针, 所以只需要在外层套一个Box智能指针就好了
	```rust
	1 trait Token { 
	2     fn nup(&self);
	3     fn led(&self);
	4 }
	5 
	6 struct PlusToken { 
	7 }
	8 
	9 impl Token for PlusToken { 
	10     fn nup(&self) { 
	11     } 
	12     fn led(&self) { 
	13     } 
	14 }
	15 
	16 fn main() { 
	17     Vec::<Box<dyn Token>>::new();
	18 }
	```

### token中涉及到的方法的最终实现方式

