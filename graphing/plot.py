import matplotlib.pyplot as plt
import csv
import os
import numpy as np

path = "csv"

os.mkdir("results", exist_ok=True)

# data[config][concurrency][trial]
data = {}
for config in ["native", "host", "bridge"]:
    data[config] = {}
    for concurrency in [1,10,50,100,200]:
        data[config][concurrency] = {}

for file in os.scandir(path):
    if file.is_file():
        parts = file.name.split('_')

        config = parts[3].split('.')[0]
        trial = int(parts[2])
        concurrency = int(parts[1])
        
        y = []

        with open("csv/" + file.name, mode='r') as file:
            csvfile = csv.reader(file)
            for lines in csvfile:
                y.append(int(lines[1]))
                 
        parts = file.name.split('_')

        config = parts[3].split('.')[0]
        trial = int(parts[2])
        concurrency = int(parts[1])
        
        data[config][concurrency][trial] = y 

x = []

for i in range(0, len(y)):
    x.append(i)

for config in ["native", "host", "bridge"]:
    means = []
    stdevs = [] 
    
    for concurrency in [1,10,50,100,200]:
        trial_means = []

        for trial in range(1, 6):
            trial_means.append(np.mean(data[config][concurrency][trial]))
        means.append(np.mean(trial_means))
        stdevs.append(np.std(trial_means))
    
    plt.errorbar([1,10,50,100,200], means, yerr=stdevs, label=config, capsize=4, marker='o')

ax = plt.gca()
ax.set_ylim([0, 20000])

plt.xlabel("Concurrency level")
plt.ylabel("Mean latency (us)")
plt.legend()
plt.title("Mean latency vs concurrency")
plt.show()

# for config in data:

# plt.plot(x, y)

# plt.xlabel("Request number")
# plt.ylabel("Latency (us)")

# plt.title("Latency of transactions: " + file.name)

# files = file.name.split('.')[0].split('/')[1]
# # plt.show()
# plt.savefig("results/" + files)
# plt.clf()

# print(data)