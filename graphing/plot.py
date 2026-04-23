import matplotlib.pyplot as plt
import csv
import os
import numpy as np

try:
    os.mkdir("results")
except:
    pass

DEADLINE = 7000.0

def parse_data():
    # data[config][concurrency][trial]
    data = {}
    for config in ["native", "host", "bridge"]:
        data[config] = {}
        for concurrency in [1,10,50,100,200]:
            data[config][concurrency] = {}

    for file in os.scandir("csv"):
        if file.is_file():
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
    
    return data

def generate_mean_lat(data):
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
    plt.title("Mean latency vs concurrency")
    plt.show()

def generate_deadline_misses(data):
    for config in ["native", "host", "bridge"]:
        means = []
        stdevs = []
        
        for concurrency in [1,10,50,100,200]:
            percentages = []

            for trial in range(1, 6):
                curr_trial = data[config][concurrency][trial]
                trial_nums = len(curr_trial)

                made_deadline = 0

                for time in curr_trial: 
                    if time < DEADLINE:
                        made_deadline += 1 

                percentages.append((1 - (made_deadline / trial_nums)) * 100)

            means.append(np.mean(percentages))
            stdevs.append(np.std(percentages))
        
        plt.errorbar([1,10,50,100,200], means, yerr=stdevs, label=config, capsize=4, marker='o')

    ax = plt.gca()
    ax.set_ylim([0,100])

    plt.xlabel("Concurrency level")
    plt.ylabel("Percentage of deadline misses")
    plt.title("% of deadline misses vs concurrency")
    plt.legend()
    plt.show()

data = parse_data()
generate_deadline_misses(data)