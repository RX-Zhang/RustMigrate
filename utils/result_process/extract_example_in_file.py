import os

def get_all_4_level_paths(root_folder):
    all_4_level_paths = []
    for dirpath, dirnames, filenames in os.walk(root_folder):
        # 计算当前目录的深度，根目录为 1 级
        depth = dirpath[len(root_folder):].count(os.sep) + 1
        if depth == 4:  # 只提取深度为4的路径
            all_4_level_paths.append(dirpath)
    return all_4_level_paths


def extract_elements(paths):
    extracted_tuples = []
    for path in paths:
        # 将路径按 / 分割
        parts = path.split(os.sep)
        # 提取第2, 3, 4, 5个元素，注意 Python 索引从0开始
        if len(parts) >= 12:  # 确保路径足够长
            # 提取指定的元素
            zero_element = parts[6]
            fourth_element = parts[8]
            fifth_element = parts[9]
            sixth_element = parts[10]
            seventh_element = parts[11]
            #extracted_tuple = tuple(parts[8:12])
            extracted_tuples.append(f"(\"{zero_element}\",\"{fourth_element}\",\"{fifth_element}\", \"{sixth_element}\", \"{seventh_element}\"),")
    return extracted_tuples


def write_tuples_to_file(tuples, file_name):
    with open(file_name, 'w') as f:
        for tup in tuples:
            f.write(tup)
            f.write("\n")

# 文件夹路径，例如 "./libopenaptx/"
folder_path1 = ".."
folder_path2 = ".."
folder_path3 = ""
folder_path4 = ""


# 获取所有4级目录路径并转化为字符串
all_4_level_paths = get_all_4_level_paths(folder_path4)

# 提取第2, 3, 4, 5个元素组成元组
result_tuples = extract_elements(all_4_level_paths)
# 将这些元组写入到一个txt文件中，路径可以根据需要调整
output_file = "result_all/output_file.txt"
write_tuples_to_file(result_tuples, output_file)