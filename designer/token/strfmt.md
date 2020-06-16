## strfmt
### 描述
- 在字符串中引用表达式
- 默认方式 **strfmt"${表达式}"**
	```lions-language
	strfmt"hello ${name}"
	```
- 如果字符串中存在 *${* 和 *}*, 可以指定起始串和终结串
	- **strfmt<"起始字符串"><"终结字符串">"[起始字符串]表达式[终止字符串]"**
	```lions-language
	strfmt<"{{{"><"}}}">"hello {{{name}}}"
	```

