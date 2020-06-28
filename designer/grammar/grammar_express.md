## 表达式
### 问题
- 表达式具有优先级问题, 如何解决优先级问题
- 存在括号的情况下如何处理
- 表达式何时可以视为结束

### lions-language表达式的语法
- 先看一下要实现的表达式的最终结果
	- 1 + 2
	- 1 + 2; a + b;
	- a + b;
	- 1 + (2 + 3) \* 5
	- 1 +
		2
- 也就是:
	- 支持在行末不需要分号也可以是一条合法的语句
	- 支持使用分号分割不同的语句
	- 支持使用 小括号 来提高优先级
	- 支持 操作符后面换行, 以至于不会时一行写太多的代码

### nup与led方法
- 在词法解析阶段, 提到过两个函数, 并将它们存入到了 Token 结构中, 之前没有提及它们的作用, 在语法分析阶段, 它们的作用就是为表达式解析而准备的
	- nup: 遇到 前缀运算符(-, !, ~, ++, --, ...) / 操作数(id, number, ...) / 小括号 ... 时, 都在 nup 方法中处理, 也就是说, 在遇到 前缀运算符/操作数/小括号的时候, 将调用 对应 token 的 nup 方法
		- nup方法处理完毕之后, 下一个 token 一定是 操作符 或者 结束符(换行 / 分号)
	- led: 遇到 操作符(+, -, \*, /, ...) 时, 调用 对应token的 led 方法
		- led方法处理完毕之后, 下一个 token 一定是 操作符 或者 结束符(换行 / 分号)

### 处理优先级
- 思路
	- 如果遇到一个比自身优先级大的, 就要让优先级大的向后查找, 直到 优先级大的 找到和它自己相同(或者更小)的为止, 举个例子: 1 + 2 \* 3 + 4
		* \* 号的优先级大于 + 号的优先级, 在遇到 1之后的+ 时, 需要继续查找下一个运算符, 如果下一个运算符是 +, 那么不需要优先计算, 所以直接计算 1 + 2, 但是在这里, 后面的运算符是 \*, 也就是说 下一个 运算符的优先级大于本次运算符的优先级, 那么就需要对 2后面的 \* 号, 做同样的逻辑(\* 的下一个运算符是 +, 优先级小于 \* 号, 那么结束判断, 对 2\*3 进行计算), 计算完毕后, 回到上一次的递归点, 也就是 1后面 + 号处, 此时, 继续查找 1后面的操作符的优先级, 这个时候的下一个操作符应该是 3后面的 + 号, 这里的 + 并不大于 1后面的+号 优先级(因为都是+, 优先级相等), 那么 1后的+结束, 计算 1+ (2\*3的结果), 最后就是 处理 3后面的+, 因为后面没有操作符, 所以结束查找, 计算 (1+ 2\*3的结果) + 4, 这样, 整个表达式的计算就结束了
	- 为了便于理解, 将错误处理部分去掉, 得到如下框架性的结构
	```rust
	51     pub fn expression(&mut self, operator_bp: &u8, express_context: &ExpressContext<T, CB>, input_token_ptr: &TokenPointer) -> TokenMethodResult {
	52         let input_token = input_token_ptr.as_ref::<T, CB>();
	53         input_token.nup(self, express_context);
	54         let mut next_tp = self.lexical_parser.lookup_next_one_ptr().unwrap();
	55         let mut next_token = next_tp.as_ref::<T, CB>();
	56         while next_token.token_attrubute().bp > operator_bp {
	57             next_token.led(self, express_context);
	58             next_tp = self.lexical_parser.lookup_next_one_ptr().unwrap();
	59             next_token = next_tp.as_ref::<T, CB>();
	60         }
	61         TokenMethodResult::End
	62     }
	```
	- 计算优先级的主要语句就是第 56 行的 while 条件, 意思就是: 只要找到优先级比输入的大的操作符就继续循环, 否则结束循环, 换一种说法就是: 只要优先级比输入的小, 或者等于输入的, 那么就跳出循环; 就拿加号来说, expression的 operator_bp 就是 加号的 优先级, 调用 expression 是为了一直遍历, 直到找到小于等于 operator_bp 的操作符为止, 如果后一个 操作符的优先级小于等于 加号的优先级, while 循环将结束, 表示的就是: 需要先计算加号, 后面的等到加号计算结束后再去计算; 如果后一个操作数是 乘号, while 循环将继续, led 方法中就会调用 expression 方法, 并将 乘号的优先级作为第一个参数传递给 expression, 这样就变成了, 乘号的 while 循环, 找到比乘号优先级小(或者等于)的为止; 总结一下: 只要遇到比自己大的, 那就不能先计算自己, 应该让比自己大的计算完成后, 再回过来继续执行自己
	- nup方法
		- 该方法中处理操作数之类的token, 从代码的 56 行可以看到, next_token 具有 bp 属性, 说明 next_token 是操作符, 也就是说 nup 结束后, next token 是操作符; nup 中不管处理了什么, 结束后需要保证next token是操作符, 或者是 结束符, 所以, nup 处理的就是 两个操作符之间的 token(或者是 开始...操作符 / 操作符...结束)
	- led方法
		- 该方法处理操作符之类的token, 从代码的 58 行可以看到, 调用 led 之后, next token 将还是操作符, 以便于在计算完比自己大的操作符后, 继续执行自身的操作符优先级比较
	- 如何让整个表达式都遍历完全呢? 从逻辑上可以看出, 只要每次循环都能比 传入的 operator_bp 大, 那么就一定可以遍历完整的表达式; 也就是说, 初始值传入 优先级的最小值就行了, 这里就使用 0, 作为优先级最小值
	```rust
	44     pub fn expression_process(&mut self, token: &TokenPointer, express_context: &ExpressContext<T, CB>) {
	45         /*
	46          * 因为 0 比任何的操作数都要小, 所以可以将整个表达式遍历完全
	47          * */
	48         self.expression(&0, express_context, token);
	49     }
	```

### 如何判断表达式到达了结尾
- lions-language支持不写分号, 用回车就可以表示表达式语句的结束, 但是同时也支持使用分号, 表示表达式语句的结束
	- 以分号结尾比较简单, 检测到分号, 如果是合法的表达式尾部, 那么就正常结束, 否则就报错
	- 但是以换行结尾有些复杂, 因为要在换行时, 判断表达式是否完整(这里就是之所以保留换行token的原因)
		- 如果换行前, 上一个token是操作数(可以正常结束)
		```rust
		1 + 1
		```
		- 如果换行前, 上一个token是操作符, 那么就需要判断是否查看下一行了
		```rust
		a &&
		b &&
		c
		```
- 另外, 在一些特殊情况下, 可能是其他的结尾符
	- 在 if 的条件中, 结尾符应该是 {
	```rust
	if a == 1 {
	}
	```
	也可以写成
	```rust
	if a
	== 1 {
	}

	if a ==
	1 {
	}
	```
	- 在函数的定义内, 结尾符可以是 }
	```go
	func f1() {
		a = 1
	}
	```
	也可以写成
	```go
	func f1() {
		a = 1}
	```
- 可以看出, 在不同的场景下, 是否结尾是不同的, 并且, 不是一个 token 就能决定的
	- 这里的解决方案就是: 对表达式求解方法传入一个回调, 该回调决定是否到达了表达式的尾部
	```rust
	64 pub type ExpressEndFunc<T, CB> = fn(&mut GrammarParser<T, CB>, &TokenVecItem<T, CB>) -> TokenMethodResult;
	65 
	66 pub struct ExpressContext<T: FnMut() -> CallbackReturnStatus, CB: Grammar> {
	67     pub end_f: ExpressEndFunc<T, CB>
	68 }
	```
	- 另外, 这里不需要捕获外部环境, 所以使用静态函数, 可以提高创建闭包带来的运行时消耗

### Grammar trait 定义的接口
- 因为语法分析阶段都是相同的, 但是具体是要先生成字节码后再运行, 还是直接动态执行, 这不是语法分析的任务, 这些属于编译器后端, 所以这里留出接口, 等到编译阶段, 再去实现这些接口
- 这样做的好处
	- 使代码的可扩展性增强
	- 使得代码层次更加清晰
	- 在 "一遍" 的情况下, 也可以使代码结构清晰(Grammar的接口就相当于是语法树的结果)

### 示例(以 含有 + 和 \* 运算符, 数值操作数的表达式进行势力解析)
- expression完整定义
```rust
 73     pub fn expression(&mut self, operator_bp: &u8, express_context: &ExpressContext<T, CB>, input_token_ptr:
 74         let input_token = input_token_ptr.as_ref::<T, CB>();
 75         match input_token.nup(self, express_context) {
 76             TokenMethodResult::None => {
 77                 /*
 78                  * 如果 nup 中遇到了 前缀运算符, 内部会进行自身调用, 如果 前缀运算符后面不是 nup 可以处理的 
 79                  * 比如说, 解析到 -1:
 80                  * 1. 首先调用 - 号的 nup 方法, - 号的 nup 方法中获取 next token. 调用 next token 的 nup 方>
 81                  * */
 82                 self.panic(&format!("expect operand, but found {:?}", &input_token.context_ref().token_type)
 83             },
 84             TokenMethodResult::StmtEnd => {
 85                 return TokenMethodResult::StmtEnd;
 86             },
 87             _ => {}
 88         }
 89         let mut next_tp = match self.lexical_parser.lookup_next_one_ptr() {
 90             Some(tp) => {
 91                 tp
 92             },
 93             None => {
 94                 /*
 95                  * 操作数之后是 EOF => 结束
 96                  * */
 97                 return TokenMethodResult::StmtEnd;
 98             }
 99         };
100         let mut next_token = next_tp.as_ref::<T, CB>();
101         /*
102          * 检测是否需要结束
103          * 一条语句的结束一定在操作数之后
104          * */
105         let cb_r = (express_context.end_f)(self, next_token);
106         match cb_r {
107             TokenMethodResult::StmtEnd
108             | TokenMethodResult::End => {
109                 /* 
110                  * 语句结束 或者 是 () 内的结束
111                  * */
112                 return cb_r;
113             },
114             _ => {
115             }
116         }
117         // println!("{}", next_token.context.token_type.format());
118         /*
119          * 如果到达这里, 说明 next_token 是操作符
120          * 比较优先级, 找到比输入的小(或者等于)的为止 (也就是说 只要大于就继续)
121          * */
122         while next_token.token_attrubute().bp > operator_bp {
123             /*
124              * 这里的 led 就是继续比对 next_token 这个操作符的 优先级, 找到比 next_token 优先级还要低(或者等
125              * */
126             // println!{"{}", next_token.context.token_type.format()};
127             match next_token.led(self, express_context) {
128                 TokenMethodResult::None => {
129                     /*
130                      * 操作符的 led 方法没有实现
131                      * */
132                     panic!(format!("operator: {} not implement", next_token.context_ref().token_type.format(
133                 },
134                 TokenMethodResult::StmtEnd => {
135                     return TokenMethodResult::StmtEnd;
136                 },
137                 _ => {}
138             }
139             next_tp = match self.lexical_parser.lookup_next_one_ptr() {
140                 Some(tp) => {
141                     tp
142                 },
143                 None => {
144                     /*
145                      * 如果到达这里, 说明 led 方法返回的不是 IoEOF, 那么这一次的 lookup next 一定不会是 None
146                      * */
147                     panic!("should not happend");
148                     return TokenMethodResult::Panic;
149                 }
150             };
151             next_token = next_tp.as_ref::<T, CB>();
152         }
153         TokenMethodResult::End
154     }
```
- 除了多了容错处理之外, 还添加了 是否是表达式结尾的判断 (105 行 ~ 116 行)
- 容错
	- 82 行: 如果 nup 处理完成之后, 返回的是 CallbackReturnStatus::None => 说明不是操作数类, 那么就是语法错误 => 提示错误
	- 132 行: 如果 led 处理完成之后, 返回的是 CallbackReturnStatus::None => 说明对应的操作符类没有实现 led 方法, 也或许说 不是 操作符类 => 语法错误
	- 147 行: 因为 led 返回的不是语句结束, 也不是None(如果是, 不会到达147行, 上面的处理要么 panic, 要么返回了, 所以 next是None的情况 是不可能发生的

- number的nup方法
```rust
15 impl NumberToken {
16     fn nup<T: FnMut() -> CallbackReturnStatus, CB: Grammar>(token: &Token<T, CB>, grammar: &mut GrammarParse    r<T, CB>, express_context: &ExpressContext<T, CB>) -> TokenMethodResult {
17         let mut token_value = TokenValue::from_token(grammar.take_next_one());
18         grammar.grammar_context().cb.express_const_number(token_value);
19         TokenMethodResult::End
20     }
21 }
```
- number token的 nup 方法就是直接转换后 调用 Grammar 的相关接口, 因为不需要做处理, 直接交予编译(执行)阶段

- 加号的led方法
```rust
 5 lazy_static!{
 6     static ref plus_token_attrubute: TokenAttrubute = TokenAttrubute{
 7         bp: &20,
 8         oper_type: &TokenOperType::Operator
 9     };
10 }
```
```rust
50     fn led<T: FnMut() -> CallbackReturnStatus, CB: Grammar>(token: &Token<T, CB>, grammar: &mut GrammarParse
51         /*
52          * 移除 + token
53          * */
54         // println!("{}", token.context.token_type.format());
55         let t = grammar.take_next_one();
56         /*
57          * 注意: 在 take_next_one() 之后, 第一个传入参数已经是无效的了
58          * 因为 take_next_one 的 token 和 传入的token是同一个对象(本次调用是由 token 发起的)
59          * 所以, 如果想利用 传入的token, 需要在之前就进行值拷贝, 或者使用 take_next_one 的结果
60          * (这就是之前 unsafe 可能导致的问题, rust 不让编译, 是有道理的, 幸运的是, 我们知道问题在哪里, 可以>
61          * */
62         /*
63          * 查找, 直到找到比 + 优先级小或者等于的为止
64          * */
65         let tp = match grammar.lookup_next_one_ptr() {
66             Some(tp) => {
67                 tp
68             },
69             None => {
70                 /*
71                  * 操作符之后没有token => 语法错误
72                  * */
73                 grammar.panic("expect one token, but arrive EOF");
74                 return TokenMethodResult::Panic;
75             }
76         };
77         // println!("{}", tp.as_ref::<T, CB>().context.token_type.format());
78         // println!("{}", token.context.token_type.format());
79         let r =  grammar.expression(t.token_attrubute().bp, express_context, &tp);
80         grammar.grammar_context().cb.operator_plus(TokenValue::from_token(t));
81         r
82     }
```
- 代码第 73 行, 如果 + 后面是 IO EOF, 说明在表达式还没有结束的时候, 就到达了 IO 尾部, 这种是语法错误, 需要给出提示
- 80行, 调用 Grammar 定义的接口

- 乘号的led方法
```rust
 8 lazy_static!{
 9     static ref multiplication_token_attrubute: TokenAttrubute = TokenAttrubute{
10         bp: &21,
11         oper_type: &TokenOperType::Operator
12     };
13 }
```
```rust
20     fn led<T: FnMut() -> CallbackReturnStatus, CB: Grammar>(token: &Token<T, CB>, grammar: &mut GrammarParse
21         /*
22          * 移除 * token
23          * */
24         let t = grammar.take_next_one();
25         /*
26          * 查找, 直到找到比 * 优先级小或者等于的为止
27          * */
28         let tp = match grammar.lookup_next_one_ptr() {
29             Some(tp) => {
30                 tp
31             },
32             None => {
33                 /*
34                  * *号 后面遇到了 EOF => 语法错误
35                  * */
36                 grammar.panic("expect operand, but arrive EOF");
37                 return TokenMethodResult::Panic;
38             }
39         };
40         let r = grammar.expression(t.token_attrubute().bp, express_context, &tp);
41         grammar.grammar_context().cb.operator_multiplication(TokenValue::from_token(t));
42         r
43     }
```
- 乘号的led和plus基本一致, 都是一样的逻辑
- 主要的不同点就是 优先级大小

### 处理表达式中含有小括号
- 思路
	- 因为 () 出现在 操作符之间(或者 开头/结尾), 那么符合 nup 的定义, 所以应该要为 左括号token编写 nup 方法
- 代码
```rust
 9     fn expression_end_right_parenthese<T: FnMut() -> CallbackReturnStatus, CB: Grammar>(grammar: &mut Gramma 10         let tp = match grammar.skip_white_space_token_with_input(TokenPointer::from_ref(token)) {
11             Some(tp) => {
12                 tp
13             },
14             None => {
15                 /*
16                  * 查找 ) 时, 遇到了 IoEOF => 语法错误
17                  * */
18                  grammar.panic("expect a ), but arrive IoEOF");
19                  return TokenMethodResult::Panic;
20             }
21         };
22         let t = tp.as_ref::<T, CB>();
23         match &t.context_ref().token_type {
24             TokenType::RightParenthese => {
25                 grammar.skip_next_one();
26                 return TokenMethodResult::End;
27             },
28             _ => {
29             } 
30         }
31         TokenMethodResult::Continue
32     }
33     
34     fn nup<T: FnMut() -> CallbackReturnStatus, CB: Grammar>(token: &Token<T, CB>, grammar: &mut GrammarParse
35         /*
36          * 表达式中遇到 ( 符号
37          * 1. 先跳过  (
38          * 2. 调用 expression (因为 小括号内的可以视为一个完整的语句)
39          * */
40         grammar.skip_next_one();
41         let tp = match grammar.lookup_next_one_ptr() {
42             Some(tp) => {
43                 tp
44             },
45             None => {
46                 /*
47                  * ( 后面是 EOF => 语法错误
48                  * */
49                 grammar.panic("expect operand after `(`, but arrive EOF");
50                 return TokenMethodResult::Panic;
51             }
52         };
53         grammar.expression(&0, &ExpressContext::new(LeftParentheseToken::expression_end_right_parenthese), &
54     }
```
- 在含有 ( 的情况下, 结束符应该是 ), 所以需要一个新的 回调函数: expression_end_right_parenthese
	- 在遇到 ) 时返回结束
- 左括号的 nup 方法就是调用 expression 方法, 因为 () 中的语句优先级是最高的

### 如何处理前缀运算符

### 如何处理后缀运算符

### 看代码一步一步分析
- 因为表达式解析部分比较难理解, 这里将用一个完整的示例解析一下
- 解析 1 + 2 \* 3 + 4 表达式

