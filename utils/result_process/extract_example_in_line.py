def extract_from_line(line):
    """ 从给定的行中提取所需的字段 """
    parts = line.strip().split('/')

    # 确保行中有足够的部分
    if len(parts) < 8:
        return None

    # 提取指定的元素
    zero_element = parts[1]
    fourth_element = parts[3]
    fifth_element = parts[4]
    sixth_element = parts[5]
    seventh_element = parts[6]

    # 返回一个元组
    return (f'"{zero_element}"',f'"{fourth_element}"', f'"{fifth_element}"', f'"{sixth_element}"', f'"{seventh_element}"')


def process_file(input_file_path, output_file_path, target_string):
    """ 处理输入文件中包含特定字符的行并将结果保存到输出文件 """
    with open(input_file_path, 'r') as file:
        lines = file.readlines()

    results = []
    for line in lines:
        if target_string in line:
            result = extract_from_line(line)
            if result:
                results.append(result)

    # 将结果保存到输出文件
    with open(output_file_path, 'a') as file:
        for result in results:
            result_str = f"({', '.join(result)})"
            file.write(f"{result_str},\n")


# 指定输入和输出文件路径
input_file_path1 = ""

output_file_path = 'result_all/output_line.txt'

target_string = 'comparison_results'

with open(output_file_path, 'w') as file:
    pass

process_file(input_file_path1, output_file_path, target_string)

