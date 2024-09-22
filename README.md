# Bullshit Remover CLI

Utilizes the [Bullshit Remover](https://www.bullshitremover.com/) API to remove bullshit from input text, right from your terminal!

Basically, it's a public LLM that filters input text, removing any sort of fluff, corporate PR speak, or anything else redundant, and returns the condensed essence of what the input text is trying to say.

Text can be piped to `rmbs`, or provided as an argument. You can use `--quiet | -q` to suppress the progress animation. 

The `ped` utility in the second demo is just a simple bash script that creates a temporary file, opens it in your editor of choice, then pipes its contents to the specified program before deleting the temporary file. That's why no `|` pipe symbol is present, but rest assured, it's being piped.

https://github.com/user-attachments/assets/3388a661-fb6e-4527-8491-c8cd9f0c2124

https://github.com/user-attachments/assets/b73f3d72-8e3f-4e2c-a414-e6b5fccaf814
