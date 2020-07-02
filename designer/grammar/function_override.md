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
- 每一个类型中存在一个方法重写映射表
	- addrs: vec: 每一个元素指向实际函数定义的位置
	- override_mapping: map[string, Enum(int)] => key: 模块_方法声明; value: vec的索引
- 每一个类型中还存在一个方法定义映射表
	- addrs
	- define_mapping
- override_mapping 的 value 可能是 Override(重写), 也可能是 Reference(引用)
	- Override: 这表示, 这个方法重写的定义在这个模块中
	- Reference: 这表示, 这个方法不定义在这个模块中, 而是定义在其他地方, 而value指向的实际定义的位置; 当方式定义在其他模块, 而当前模块导入的时候会出现这种情况
- 写入override_mapping
	- 包导入时, 找到被导入包中的所有公共的, 且 为重写的方法声明, 在方法声明前面追加本包信息, 将组合后的值作为 override_mapping 的 key, override_mapping 的 value 为 Reference(实际定义的索引)
	- 方法定义的时候, 查找方法在对应类型的 define_mapping 中是否存在
		- 存在: 写入 override_mapping 中
		- 不存在: 写入 define_mapping 中
- 从类型中查找方法
	- 拿着当前包信息去类型的override_mapping中找
		- 找到 => 获取定义的位置
		- 找不到
			- 从 define_mapping 中查找 方法定义
				- 找不到: 报错
				- 找到 => 获取定义的位置

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
	- primeval_set: set[PrimevalMethod]
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
- 查找流程
	- 将函数信息对象转换为 PrimevalMethod 枚举
	- 如果转换成功, 说明输入的操作属于原生操作, 否则不是原生操作, 如果不是原生操作, 不可能在 override_mapping 中
		- 转换成功
			- 根据模块信息查找 override_mapping 中是否存在
				- 存在
					- 将 PrimevalMethod 转换为字符串, 用转换后的字符串在 override_mapping 中查找
						- 找到
							- 调用 override_mapping 中的编译的方法
						- 未找到
							- 根据 PrimevalMethod 执行/编译 宿主语言实现的方法
				- 不存在
					- 根据 PrimevalMethod 执行/编译 宿主语言实现的方法
		- 转换失败
			- 转换失败, 说明不是宿主语言实现的方法, 也就是说这可能是使用 lions-language 拓展的方法, 那么就从define_mapping 中查找; 根据 函数信息对象 拼接字符串, 用得到的字符串在 define_mapping 中查找
				- 找到
					- 调用 define_mapping 中指定方法实现
				- 未找到
					- 不是原生方法, 也没有定义 => 报错

