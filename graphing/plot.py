import matplotlib.pyplot as plt
import csv

y = []

with open('results.csv', mode='r') as file:
    csvfile = csv.reader(file)
    for lines in csvfile:
        y.append(int(lines[1]))   

x = []

for i in range(0, len(y)):
    x.append(i)


plt.plot(x, y)

plt.xlabel("Request number")
plt.ylabel("Latency (us)")

plt.title("Latency of transactions")

plt.show()