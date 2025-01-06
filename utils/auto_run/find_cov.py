import os


def find_cov_folders(directory, output_file_with_cov='with_cov.txt', output_file_without_cov='without_cov.txt'):
    # 用于存储包含 cov 文件夹的子文件夹路径
    cov_folders = []
    non_cov_folders = []

    # 列出顶级目录中的所有子文件夹
    subfolders = [f.path for f in os.scandir(directory) if f.is_dir()]

    # 遍历顶级子文件夹
    for subfolder in subfolders:
        if 'cov' in os.listdir(subfolder):  # cov    verify_result
            cov_folders.append(subfolder)
        else:  # 如果没有 cov 文件夹
            non_cov_folders.append(subfolder)

    # 将包含 cov 的文件夹路径写入 with_cov.txt
    with open(output_file_with_cov, 'w') as f_cov:
        for folder in cov_folders:
            f_cov.write(folder + '\n')

    # 将不包含 cov 的文件夹路径写入 without_cov.txt
    with open(output_file_without_cov, 'w') as f_non_cov:
        for folder in non_cov_folders:
            f_non_cov.write(folder + '\n')


# 调用函数，传入要搜索的文件夹路径
folder_path = "../../transpilations/.."
find_cov_folders(folder_path)