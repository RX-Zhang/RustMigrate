import subprocess

# Define parameter list
# ============ claude2 claude3 gemini gpt4 mistral === BaseRepair CAPR Hinted Restart ============
parameters = [
# ("benchmark-name", "submodule-name"),
# ...
]

# Traverse the parameter list and run the script
for param in parameters:
    command = [
        "python",
        "./driver.py",
        "--benchmark-name", param[0],
        "--submodule-name", param[1],
        "--model", "gpt4",
    ]

    print("Running command:", " ".join(command))

    # Execute command
    result = subprocess.run(command, capture_output=True, text=True)

    # Print standard output and standard error output
    print("Output:", result.stdout)
    print("\n")
    #print("Error:", result_process.stderr)
