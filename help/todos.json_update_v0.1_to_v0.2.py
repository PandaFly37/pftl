print('todos.json v0.1.0 -> v0.2.0')
print('注意: 如你所见, 这个代码写的并不好, 所以请确保你的todos.json为pftl程序生成, \
以免出现错误!')
input('请将todos.json放在运行此文件的目录下, 然后按Enter.')
f = open('todos.json', 'r', encoding='utf-8')
ff = f.read().split('\n')
f.close()
cnt = 0
final_s = ''
for i in ff:
    final_s += i + '\n'
    if '{' in i:
        cnt += 1
        final_s += '    "id": ' + str(cnt) + ',\n'
print('请预览以下内容, 确认后按Enter写入todos.json')
print(final_s)
input('请预览以上内容, 确认后按Enter写入todos.json')
f = open('todos.json', 'w', encoding='utf-8')
ff = f.write(final_s)
f.close()
