import os


def get_subfolders(path):
    # 获取当前文件夹名称
    folder_name = os.path.basename(os.path.normpath(path))

    # 获取 path 目录下的所有子文件夹名称
    subfolders = [f.name for f in os.scandir(path) if f.is_dir()]

    # 生成符合要求的元组列表
    result = [(folder_name, subfolder) for subfolder in subfolders]

    return result


def write_to_file(data, file_path):
    with open(file_path, 'w') as file:
        for item in data:
            # 将元组格式化为 ("opl", "sub_file_name"),
            file.write(f'("{item[0]}", "{item[1]}"),\n')


# 文件夹路径
path1 = "../../benchmark/c/opl"
path2 = "../../benchmark/c/libopenaptx"
path3 = "../../benchmark/c/cpw"

# 输出文件路径
output_file = "run_res.txt"

# 调用函数获取子文件夹的元组列表
result = get_subfolders(path1)

# 将结果写入文件
write_to_file(result, output_file)
