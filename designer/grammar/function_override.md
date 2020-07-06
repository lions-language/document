## 函数重载
### 思路
- 因为函数的声明中已经附带了参数的类型和个数, 只要组合起来不同, 但是函数名相同就可以重载

## 类型方法拓展
### 思路
- 如果解析到函数的声明, 就将其存储到树中, 然后从已经存在的声明列表中查找是否存在
	- 存在: 重写(覆盖当前模块中的声明, 不影响其他模块的)
	- 不存在: 添加

### 模块中的方法记录在哪里
- 方式
	- 方式一: 记录在每一个模块中, 查找的时候, 先从当前模块中查找, 如果找到了, 就使用当前模块的; 否则继续向前一级查找
	- 方式二: 将所有的声明都记录在对应类型的位置, 但是每一个声明都需要 标定 所属的模块(树状结构)
		- 查找时, 拿着当前的模块信息去类型的方法列表中查找
- 选择
	- 相比方式二, 方式一的查找效率太低, 并且方式二易于给出错误帮助, 方式二还可以对实际定义的位置进行地址引用, 而不是字符串拷贝

### 详细说明方式二
- 每一个类型中都存在方法查找表
	- define_map, 提供了两个查找的方法
		- 从含有模块的方法集合中获取给定的方法 (为了方便说明, 后续称为 find_mod_method)
		- 从不含模块的方法集合中获取给定的方法 (为了方便说明, 后续称为 find_method)
- 如何插入
	- 直接 调用 find_method, 检测是否存在
		- 存在 (说明已经定义了), 如果想再次定义, 那就是重写了, 所以:
			- 拿着mod信息, 调用 find_mod_method, 检测是否存在
				- 存在
					- 判断得到地址类型是否是引用类型
						- 引用类型, 说明是导入其他 mod, 里面定义的方法, 此时当前模块想要重写, 那么就写入到当前模块的方法集合中
						- 定义类型, 说明当前模块中已经重写了该方法, 那么就要报错
				- 不存在
					- 写入到 当前模块的方法集合中
		- 不存在
			- 直接写入到 不含模块的方法集合中
- 如何查找 (查找和写入恰好相反, 需要先从细粒度开始的往外找)
	- 调用 find_mod_method, 检测是否存在
		- 存在
			- 返回地址
		- 不存在
			- 调用 find_method, 检查是否存在
				- 存在
					- 返回地址
				- 不存在
					- 报错
- 导入时的操作

### 方法编译后
- 记录的字段
	- 属性
		- 类型
			- 类型方法
			- 成员方法
				- 定义
				- 重写
		- 可见性
			- 公开
			- 私有

### 原生类型的不同点
- 原生类型相比其他类型, 多出一个 primeval_set (原生集合)
	- primeval_set: PrimevalMethod
		- PrimevalMethod举例
			- U32PlusU32(u32, u32): 因为是双目的, 所以需要提供两个值
			- StrPlusStr(Str, Str)
			- U32IdPrefixPlusPlus(u32): 因为是单目运算, 所以需要提供一个值
- 原生类型的 define_mapping 中存在几个一定有的 key(原生类型的内置方法), 但是这些 key 是没有具体的定义的, 因为这些内置方法都是由宿主语言实现的
	- 比如说: 遇到了 u32 + u32, 这就是 原生类型的内置运算
		- 首先拿着当前模块信息查找 u32 类型中的 override_mapping 中是否存在 **u32_plus_u32**
			- 存在: 调用重写的方法
			- 不存在
				- 方法是否在 primeval_set 中
					- 存在
						- 运行时, 调用以下函数, 直接得到结果
						```rust
						enum PrimevalMethodResult {
						}
						fn primeval_method(value: PrimevalMethod) -> PrimevalMethodResult;
						```
						- 编译时, 调用以下函数, 获取字节码
						```rust
						fn compile_primeval_method(value: PrimevalMethod);
						```
- 优化分析
	- 拿着当前模块信息去 override_mapping 中查找的时候, 首先应该查找 override_mapping 是否存在 当前模块信息, 如果连当前模块都没有, 那么当前模块一定没有重写类型中的任何方法; 这种情况下就没必要对 函数信息对象(包含: 字段个数, 函数名, 参数类型列表(原生类型是枚举), 返回类型列表(原生类型是枚举)) 进行字符串的转换了 (转换为 函数名_参数类型_...); 所以顺序应该是先查找 当前模块, 在条件满足的情况下, 再去拼接字符串

- 原生类型查找流程
	- 外部调用方可以根据实际的语法环境, 计算出 PrimevalMethod 的值, 将该值传入查找流程, 判断是否和 PrimevalMethod 的成员匹配
		- 匹配, 说明 这个方法在原生类型中已经存在了, 如果没有找到重写的版本, 就直接调用原生版本
			- 调用 find_mod_method, 检测是否存在
				- 存在
					- 返回重写的地址
				- 不存在
					- 执行原生
		- 不匹配, 说明这个方法在原生类型中根本不存在(比如说, 想让 u32+Test, 这种的, 原生提供不了), 那就相当于是走的普通方法的判断流程
			- 调用 find_mod_method, 检测是否存在
				- 存在
					- 返回地址
				- 不存在
					- 调用 find_method, 检查是否存在
						- 存在
							- 返回地址
						- 不存在
							- 报错
- 原生类型插入流程
	- 外部根据上下文得到 PrimevalMethod, 判断是否匹配
		- 匹配, 那么这次的定义一定是重写; 但在这之前需要先查找当前模块中是否已经存在了, 也就是调用 find_mod_method, 检测是否存在
			- 存在, 判断一下地址类型(可能是导入的)
				- 引用类型: 覆盖地址
				- 定义类型: 已经定义过存在 => 报错
			- 不存在
				- 直接写入 当前模块的方法集合中
		- 不匹配, 相当于走的普通方法的判断流程
			- 直接 调用 find_method, 检测是否存在
				- 存在 (说明已经定义了), 如果想再次定义, 那就是重写了, 所以:
					- 拿着mod信息, 调用 find_mod_method, 检测是否存在
						- 存在
							- 判断得到地址类型是否是引用类型
								- 引用类型, 说明是导入其他 mod, 里面定义的方法, 此时当前模块想要重写, 那么就写入到当前模块的方法集合中
								- 定义类型, 说明当前模块中已经重写了该方法, 那么就要报错
						- 不存在
							- 写入到 当前模块的方法集合中
				- 不存在
					- 直接写入到 不含模块的方法集合中

### 原生类型相关结构
```rust
 8 pub enum PrimevalData {
 9     Uint32(Uint32Data)
10 }
11 
12 pub enum PrimevalType {
13     Uint32(Uint32Method)
14 }
15 
16 pub struct PrimevalMethodMatched {
17     pub typ: PrimevalType,
18     /*
19      * 因为原生类型提供的方法的签名是固定的, 为了降低运行时消耗, 将 func_key 定义为静态的
20     * */
21     pub func_key: FunctionKey
22 }
23 
24 pub struct PrimevalMethodRightNotMatched {
25     pub typ: PrimevalType,
26     pub func_key: FunctionKey
27 }
28 
29 pub enum PrimevalMethod {
30     /*
31      * 与原生类型的方法匹配, 可能就是原生类型方法, 也可能是原生类型方法的重写
32      * */
33     Matched(PrimevalMethodMatched),
34     /*
35      * 是原生类型的拓展方法 (属于原生类型, 但是方法不是原生类型提供的)
36      * */
37     RightNotMatched(PrimevalMethodRightNotMatched)
38 }

65 pub struct PrimevalContext<M>
66     where M: FinderMap {
67     primeval_set: PrimevalMethod,
68     define_map: M
69 }
70 
71 pub struct PrimevalControl<M>
72     where M: FinderMap {
73     uint32_method: PrimevalContext<M>
74 }
75 
76 pub enum Panic {
77     Undefine(Option<&'static str>)
78 }
79 
80 pub enum FindMethodResult<'a> {
81     Address(&'a FunctionAddress),
82     SingleOptCode(OptCode),
83     Panic(Panic)
84 }

 8 pub enum FunctionKey {
 9     /*
10      * 如果是类似原生方法的类型, function key 是不需要字符串拼接的, 在 rust编译期就可以知道
11      * */
12     Static(&'static str),
13     Dynamic(String)
14 }
```

### 原生类型代码优化
- 原生类型提供的方法(重写/直接使用原生方法)
	- 在外部调用的时候是知道PrimevalMethod的方法的, 而且原生类型提供的方法, function key 是固定的, 比如说: uint32 + uint32, 那么组合得到的 function key 一定是 uint32_+_uint32, 那么就可以将此时的类型设置为 **&'static str**, 这样的话, 就没有运行时候的内存开销和运算开销, 举个 uint32 + uint32 创建的例子
	```rust
	10 impl PrimevalMethod {
	11     pub fn new_uint32_plus_operator_uint32(right_data: Uint32Data) -> Self {
	12         PrimevalMethod::Matched(PrimevalMethodMatched{
	13             typ: PrimevalType::Uint32(Uint32Method::PlusOperatorUint32(right_data)),
	14             func_key: FunctionKey::Static("uint32_+_uint32")
	15         })
	16     }
	17 }
	```
- 对原生类型扩展方法(比如为 uint32 类型扩展 xxx 方法)
	- 这种情况下, 一定是需要根据function key查找函数的定义的, 而 function key 在查找的时候需要多次使用, 为了提高性能, 避免在使用的时候动态计算, 应该在创建 PrimevalMethod 的时候就将 function key 计算好
	- 拼接规则: 原生类型_方法名_参数类型_返回值类型
	```rust
	 91 impl PrimevalMethod {
	 92     pub fn new_primeval_type_right_not_matched(typ: PrimevalType, func_obj: FunctionObject)
	 93         /* 
	 94          * 为原生类型扩展方法
	 95          * 在 new 的时候进行 function key 的构造, 之后在使用的时候就不需要构建了
	 96          * */
	 97         -> Self {
	 98         let mut key_s = String::from(typ.to_str());
	 99         key_s.push('_');
	100         key_s.push_str(func_obj.function_string());
	101         PrimevalMethod::RightNotMatched(PrimevalMethodRightNotMatched{
	102             typ: typ,
	103             func_key: FunctionKey::Dynamic(key_s)
	104         })
	105     }
	106 }
	```

