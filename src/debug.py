import json


def main():
    input_str = '["Var (test2,AnonymousFn (a))", "Print (Call (Variable (test2))Literal (2)))"]'

    # 移除外层的引号，并使用 JSON 解析
    expressions = json.loads(input_str.strip(']['))
    idx = 0
    # 遍历字符串
    


    # 将解析后的表达式列表转换为 JSON 字符串
    # json_output = json.dumps(parsed_exprs, indent=2)
    # print(json_output)

if __name__ == "__main__":
    main()


class Parser:
    def __init__(self,input_str):
        self.input_str = input_str
        self.idx=0
    
    def consume(self):
        self.idx+=1
        return self.input_str[self.idx-1]
    
    def parse(self):
        while self.idx<len(self.input_str):
            print(self.consume())