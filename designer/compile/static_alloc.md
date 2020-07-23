## 何时为静态值分配空间
### 静态编译(先生成全部的字节码, 之后再去读取字节码序列)
- 这种情况下, 在编译期, 将所有的静态区的地址分配好, 然后写入到字节码中

## 函数中的静态量编译
- 遇到静态量, 为其在模块的静态区域分配一个位置, 然后将其写入到字节码中
	- 虚拟机不管是动态执行, 还是静态执行, 都需要从静态区域的记录文件中获取静态量(内存静态缓存区如果没有, 就从文件中加载)
- 静态编译与动态执行所不同的是, 静态编译不会每定义一次静态量, 就往文件中写, 而是存在内存中, 在内存缓存到一定的大小时, 批量写入文件
- 为了控制 静态编译 与 动态执行的不同处理方式, 存在一个 dispatch, 根据上下文获取是静态编译还是动态执行
```rust
trait StaticAlloc {
	fn alloc(&mut self, data: Data) -> AddressKey;
}

impl StaticAlloc for RuntimeStaticAlloc {}
impl StaticAlloc for CompileStaticAlloc {}

enum CompileType {
	Runtime,
	Compile
}

struct StaticAllocDispatch {
	env: CompileType
}

impl StaticAllocDispatch {
	fn switch_env(&mut self, env: CompileType) {
		*self.env = env;
	}

	fn alloc(&mut self, data: Data) {
		match &self.env {
			CompileType::Runtime => {
			},
			CompileType::Compile => {
			}
		}
	}

	fn new(env: CompileType) -> Self {
		Self {
			env: env
		}
	}
}
```

## 函数调用
- 首先判断是否已经定义过了(通过函数声明信息进行判断)
	- 定义过
		- 生成 **执行指定位置字节码** 的指令 (虚拟机在读取到这个指令的时候, 将从代码区找到字节码序列, 然后执行, 这个时候的字节码中已经不会存在 load static 的语句了)
	- 未定义
		- 编译未定义的函数, 生成字节码序列; 并将其写入到函数声明中

