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

### 原生类型
- 原生类型存在固定的方法名
	- 如: uint32 + uint32, 那么方法名就是 uint32_+_uint32
- 如何存储固定的方法名
	- crates中存在一个 phf 库, 该库用的hash算法是完美哈系, 保证键不重复, 另外, 该库还提供了 静态匹配, 如:
	```rust
	static MY_MAP: phf::Map<&'static str, u32> = phf_map! {
		"hello" => 1,
		"world" => 2,
	};
	```
	在这种情况下, 运行时的消耗的均衡的, 效率较高
	- phf很适合存储原生类型信息
	```rust
	139 pub struct PrimevalMethodBindValue {
	140     single_optcode: OptCode
	141 }

	147 static PRIMEVAL_METHOD_MAP: phf::Map<&'static str, PrimevalMethodBindValue> = phf_map! {
	148     "uint32_+_uint32" => PrimevalMethodBindValue{
	149         single_optcode: OptCode::Uint32PlusOperatorUint32
	150     }
	151 };
	```
- 原生类型查找流程
	- 外部调用方可以根据实际的语法环境, 计算出 FunctionKey 的值, 将 function key 放在 PRIMEVAL_METHOD_MAP 中进行查找
    - **为了优化, 应该先 match type, 然后再从类型的 map 中查找是否是原生方法**
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
	- 外部根据上下文得到 FunctionKey 对象, 判断是否匹配
		- 匹配, 那么这次的定义一定是重写; 但在这之前需要先查找当前模块中是否已经存在了, 也就是调用 find_mod_method, 检测是否存在
			- 存在, 判断一下地址类型(可能是导入的)
				- 引用类型: 覆盖地址
				- 定义类型: 已经定义过了 => 报错
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
					
### 结构类型

### 原生类型与结构类型的区别
- 原生类型, 不需要通过名称查找实例, 因为原生类型在编译期(rust编译期)就知道实例是什么(固定代码); 而结构类型是用户自定义的, 在rust代码中是无法确定的(动态代码), 所以结构类型需要通过名称(模块+结构组合而成的名称)获取实例对象
- 所以, 如果是原生类型, 可以通过匹配 Type 得到实例对象, 并且可以在代码中固定构建 Type; 而结构对象需要先获取到结构的实例对象, 然后再将该实例放入Type中, 构建出 Type 类型

### 原生类型相关结构
```rust
 9 pub enum PrimevalType {
10     Uint32(Uint32Method)
11 }

38 pub struct PrimevalMethod {
39     pub typ: PrimevalType,
40     pub func_key: FunctionKey
41 }

77 pub struct PrimevalContext<M>
78     where M: FinderMap {
79     define_map: M
80 }
81 
82 pub struct PrimevalControl<M>
83     where M: FinderMap {
84     uint32_method: PrimevalContext<M>
85 }
86 
87 pub enum Panic {
88     Undefine(Option<&'static str>),
89     AlreadyDefine
90 }
91 
92 pub enum FindMethodResult<'a> {
93     Address(&'a FunctionAddress),
94     SingleOptCode(&'static OptCode),
95     Panic(Panic)
96 }

18 #[derive(Eq, PartialEq, Hash)]
19 pub struct FunctionKey(String);
20 
21 impl FunctionKey {
22     pub fn key_ref(&self) -> &str {
23         &self.0
24     }
25 
26     pub fn new(v: String) -> Self {
27         Self(v)
28     }
29 }

 1 pub struct FunctionObject {
 2     function_str: String,
 3     type_function_str: String
 4 }
 5 
 6 impl FunctionObject {
 7     /*
 8      * 函数 new 的时候应该 to string(存储到成员), 而不是在获取时再拼接, 加快访问效率
 9      * */
10     pub fn function_string(&self) -> &str {
11         /*
12          * 函数名_参数类型_返回值类型
13          * */
14         return &self.function_str
15     }
16 
17     pub fn type_function_string(&self) -> &str {
18         /*
19          * 类型_函数名_参数类型_返回值类型
20          * */
21         return &self.type_function_str
22     }
23 }
```

### 原生类型代码优化
- 为了避免在使用时临时创建和计算 function key, 应该在 FunctionKey 对象被创建的时候, 存储在成员中, 读取的时候直接返回引用
```rust
127 impl PrimevalMethod {
128     pub fn new(typ: PrimevalType, func_obj: FunctionObject) -> Self {
129         let mut key_s = String::from(typ.to_str());
130         key_s.push('_');
131         key_s.push_str(func_obj.function_string());
132         Self {
133             typ: typ,
134             func_key: FunctionKey::new(key_s)
135         }
136     }
137 }
```

### 原生类型的查找逻辑
```rust
11     pub fn find_method(&self, method: &PrimevalMethod, module_key: &ModuleKey)
12         -> FindMethodResult {
13         match primeval_method(&method.func_key) {
14             Some(v) => {
15                 /*
16                  * 可能是:
17                  * 1. 重写的原生方法
18                  * 2. 原生方法
19                  * */
20                 return self.find_method_from_override(method, module_key, v);
21             },
22             None => {
23                 /*
24                  * 不属于任何原生方法
25                  *  => 一定不在 override_map 中, 可能在 define_map 中
26                  * */
27                 return self.find_method_from_define(method, module_key);
28             }
29         }
30     }
31 
32     fn find_method_from_define(&self, method: &PrimevalMethod
33         , module_key: &ModuleKey)
34         -> FindMethodResult {
35         let r = self.context(method).define_map.find_module_method(module_key, method.function_key());
36         match r {
37             Some(addr) => {
38                 /*
39                  * 模块中找到了 => 返回找到的地址, 提供给后续处理
40                  * */
41                 FindMethodResult::Address(addr)
42             },
43             None => {
44                 /*
45                  * 在模块方法中没找到 => 没有重写, 可能在没有模块信息的方法集合中
46                  * 检测是否在 不含有模块的方法集合中
47                  * */
48                 match self.context(method).define_map.find_method(
49                     method.function_key()) {
50                     Some(addr) => {
51                         FindMethodResult::Address(addr)
52                     },
53                     None => {
54                         FindMethodResult::Panic(Panic::Undefine(None))
55                     }
56                 }
57             }
58         }
59     }
60 
61     fn find_method_from_override(&self, method: &PrimevalMethod
62         , module_key: &ModuleKey, value: &'static PrimevalMethodBindValue)
63         -> FindMethodResult {
64         let r = self.context(method).define_map.find_module_method(module_key, &method.func_key);
65         match r {
66             Some(addr) => {
67                 /*
68                  * 原生方法被重写了
69                  * 重写了原生方法
70                  *  => 根据找到的地址进行后续处理
71                  * */
72                 FindMethodResult::Address(addr)
73             },
74             None => {
75                 /*
76                  * 原生方法没有被重写
77                  *  => 调用原生
78                  * */
79                 FindMethodResult::SingleOptCode(&value.single_optcode)
80             }
81         }
82     }
```
- 以上实现就是根据上文的思路编码的, 贴出来是为了给不方便打开源码的读者一个查看路径

