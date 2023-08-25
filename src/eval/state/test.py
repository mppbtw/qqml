f = open("./example.json").read();
for i in range(len(f)):
    if i == 558:
        print(">>>>>>>>>>>>>>>", end="")
    print(f[i], end="")
