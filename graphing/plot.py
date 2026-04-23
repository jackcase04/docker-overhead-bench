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
    plt.legend()
    plt.show()

def generate_deadline_misses(data):
    fix, ax = plt.subplots()
    cats = ["1","10","50","100","200"]
    w, x = 0.4, np.arange(len(cats))

    width_cluster = 0.7
    width_bar = width_cluster / 3

    index = 0

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

        x_positions = x+(width_bar*index)-width_cluster/2
        ax.bar(x_positions, means, yerr=stdevs, width=width_bar, label=config)

        index += 1

    ax.set_xticks(x)
    ax.set_xticklabels(cats)
    ax.set_ylim([0,100])
    ax.set_ylabel('Percent of missed deadlines')
    ax.set_xlabel('Concurrency level')
    ax.set_title('% missed deadlines vs concurrency')
    ax.legend()

    plt.show()

def generate_percentile(data, percentile):
    for config in ["native", "host", "bridge"]:
        percentiles = []
        
        for concurrency in [1,10,50,100,200]:
            trial_percentiles = []

            for trial in range(1, 6):
                trial_percentiles.append(np.percentile(data[config][concurrency][trial], percentile))
            
            percentiles.append(np.mean(trial_percentiles))
        
        plt.errorbar([1,10,50,100,200], percentiles, label=config, capsize=4, marker='o')

    ax = plt.gca()
    ax.set_ylim([0,20000])

    plt.xlabel("Concurrency level")
    plt.ylabel(f"{percentile}th percentile (us)")
    plt.title(f"{percentile}th percentile vs concurrency")
    plt.legend()
    plt.show()

data = parse_data()
generate_percentile(data, 50)