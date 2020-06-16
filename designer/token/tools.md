## 工具方法
### 封装lookup_next_one
- 功能
	- 查看下一个字符, 如果读取不到, 从 callback 中读取新的序列
	- 如果读取到了数据, 以回调的方式返回给上层
- 目的
	- 对于需要判断下一个字符是否是所需的字符时, 可以省去处理回调的麻烦
- 代码
```rust
336     fn lookup_next_one_with_cb_wrap<FindF, EndF>(&mut self, mut find_f: FindF, mut end_f: EndF)
337         where FindF: FnMut(char), EndF: FnMut() {
338         match self.content.lookup_next_one() {
339             Some(c) => {
340                 find_f(c);
341             },
342             None => {
343                 match (self.cb)() {
344                     CallbackReturnStatus::Continue(content) => {
345                         self.content_assign(content);
346                         match self.content.lookup_next_one() {
347                             Some(c) => {
348                                 find_f(c);
349                             },
350                             None => {
351                                 panic!("should not happend");
352                             }
353                         }
354                     },
355                     CallbackReturnStatus::End => {
356                         end_f();
357                     }
358                 }
359             }
360         }
361     }
```

- 调用一下 (在回调中调用成员方法试一下)
```rust
495     #[test]
496     fn lookup_next_one_with_cb_wrap_test() {
497         impl<T: FnMut() -> CallbackReturnStatus> LexicalParser<T> {
498             fn test(&mut self) {
499                 self.lookup_next_one_with_cb_wrap(|c| {
500                     self.panic("error");
501                 }, || {
502                 });
503             }
504         }
505     }
```
显示了错误
![img3](images/img3.png)

- 错误原因
	- 因为在 **&mut self** 的作用域中使用了 **&self**, 这是借用检查器不允许的
- 问题解决
	- 借用检查器无法进行细致的检查, 所以这里判断到了作用域问题后, 就提示错误, 但是这里的方法调用中并没有不安全的地方, 这个时候就需要找办法, 避开借用检查器的检查 ***这是一个小技巧***
	```rust
	308     fn lookup_next_one_with_cb_wrap<FindF, EndF>(&mut self, mut find_f: FindF, mut end_f: EndF)
	309         where FindF: FnMut(&mut LexicalParser<T>, char), EndF: FnMut(&mut LexicalParser<T>) {
	310         match self.content.lookup_next_one() {
	311             Some(c) => {
	312                 find_f(self, c);
	313             },
	314             None => {
	315                 match (self.cb)() {
	316                     CallbackReturnStatus::Continue(content) => {
	317                         self.content_assign(content);
	318                         match self.content.lookup_next_one() {
	319                             Some(c) => {
	320                                 find_f(self, c);
	321                             },
	322                             None => {
	323                                 panic!("should not happend");
	324                             }
	325                         }
	326                     },
	327                     CallbackReturnStatus::End => {
	328                         end_f(self);
	329                     }
	330                 }
	331             }
	332         }
	333     }
	```
	- 改进的办法就是将 self 通过参数传递给回调方法体, 这样的话, 在调用的时候就不会存在 **&mut self** 和 **&self** 冲突的作用域了
	```rust
	469     fn lookup_next_one_with_cb_wrap_test() {
	470         impl<T: FnMut() -> CallbackReturnStatus> LexicalParser<T> {
	471             fn test(&mut self) {
	472                 self.lookup_next_one_with_cb_wrap(|parser, c| {
	473                     parser.panic("error");
	474                 }, |parser| {
	475                 });
	476             }
	477         }
	478     }
	```

### 匹配输入字符串
- 功能
	- 如果在 字符序列中找到了匹配的串, 就返回 true, 否则返回false
- 为什么不直接用rust的子字符串查找方法?
	- lions-language的词法分析是采用实时读取的方式(不是一次性读取), 那么如果使用rust的find方法, 那么如果content中不存在数据了(也就是边界条件下)会出现找不到的问题
- 思路
	- 首先需要一个循环, 从字符序列中读取数据(如果数据不够, 就从cb中获取)
		- 先说一下第一种方式: 定义一个和待查找序列(为方便说明, 这里的待查找序列后续使用 src 替代)相同大小的buffer, 循环读取src.len()和字符, 并存入buffer中; 待src.len()存储完毕后, 将buffer和src进行比对, 看看是否相等, 伪代码(不是正确的语法, 只是为了方便阅读)如下
		```rust
		let len = src.len();
		let mut buffer = Vec::new();
		for i in 0..len {
			buffer.push(content[i]);
		}
		if src == buffer {
			return true;
		} else {
			return false;
		}
		```
		这种方式存在效率问题:
		1. 每次都需要在buffer中存储完src.len()个字符才能进行比对, 但其实很多情况下, 比对到第一个就不匹配了, 那么这样就多出很多次的无用迭代
		2. 每次存储完毕后, 需要对两个字符序列进行比对, 这又是一次迭代
		- 那么就出现了第二种优化方式: 每次迭代都与src相应位置的字符进行比较(不用担心从src的相应位置取数据的效率问题, 连续的内存取值是非常快的), 只要不相等就结束当次的迭代
		- 需要一个辅助方法
		```rust
		 1 pub struct U8ArrayIsEqual<'a> {
		 2     src: &'a [u8],
		 3     index: usize,
		 4     length: usize
		 5 }
		 6 
		 7 pub enum U8ArrayIsEqualResult {
		 8     // 没有达到输入序列的长度, 就不匹配了
		 9     NoMatch(usize),
		10     // 当前字符和之前的字符都匹配了
		11     Continue,
		12     Match(usize)
		13 }
		14 
		15 impl<'a> U8ArrayIsEqual<'a> {
		16     pub fn dynamic_match(&mut self, c: char) -> U8ArrayIsEqualResult {
		17         /*
		18          * 动态匹配 与输入的数组相等的数组
		19          * */
		20         match self.src.get(self.index) {
		21             Some(ch) => {
		22                 if ch.clone() as char == c {
		23                     self.index += 1;
		24                     // 在 ch == c 后每次都判断是否等于输入序列的长度
		25                     if self.index == self.length {
		26                         self.index = 0;
		27                         return U8ArrayIsEqualResult::Match(self.length);
		28                     } else {
		29                         return U8ArrayIsEqualResult::Continue;
		30                     }
		31                 } else {
		32                     let r = U8ArrayIsEqualResult::NoMatch(self.index);
		33                     self.index = 0;
		34                     return r;
		35                 }
		36             },
		37             None => {
		38                 /*
		39                  * 如果到达这个分支, 说明 index > length, 这是不可能发生的(注意这里的 index, 在匹配的时候才>    会自增), 因为只要和输入的序列匹配了 (index == length) 的时候, 就会直接返回, 如果中间遇到了不匹配的, 也直接返
		   回了
		40                  * */
		41                 panic!("should not happend");
		42             }
		43         }
		44     }
		45 
		46     pub fn reset(&mut self) {
		47         self.index = 0;
		48     }
		49 
		50     pub fn new(src: &'a [u8]) -> Self {
		51         Self{
		52             src: src,
		53             index: 0,
		54             length: src.len()
		55         }
		56     }
		57 }
		```
