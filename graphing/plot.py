import matplotlib.pyplot as plt
import csv
import os

path = "csv"

try:
    os.mkdir("results")
except:
    print("don't need to create")

for file in os.scandir(path):
    if file.is_file():
        y = []

        with open(file.path, mode='r') as file:
            csvfile = csv.reader(file)
            for lines in csvfile:
                y.append(int(lines[1]))   

        x = []

        for i in range(0, len(y)):
            x.append(i)


        plt.plot(x, y)

        plt.xlabel("Request number")
        plt.ylabel("Latency (us)")

        plt.title("Latency of transactions: " + file.name)

        files = file.name.split('.')[0].split('/')[1]
        # plt.show()
        plt.savefig("results/" + files)
        plt.clf()