import matplotlib.pyplot as plt

data = {}

with open("times.txt", "r") as file:
    for line in file:
        parts = line.strip().split("|")
        if len(parts) != 3:
            continue
        size = int(parts[0].strip())
        threads = int(parts[1].strip())
        time_str = parts[2].strip()

        # convert time string to seconds
        if ':' in time_str:
            mins, secs = time_str.split(":")
            total_seconds = float(mins) * 60 + float(secs)
        else:
            total_seconds = float(time_str)

        if size not in data:
            data[size] = {}
        data[size][threads] = total_seconds


plt.figure(figsize=(10, 6))

for size in sorted(data.keys()):
    threads = sorted(data[size].keys())
    times = [data[size][t] for t in threads]
    plt.plot(threads, times, marker='o', label=f'{size} elements')

plt.title("Execution Time vs Number of Threads")
plt.xlabel("Number of Threads")
plt.ylabel("Execution Time (seconds)")
plt.legend(title="Input Size")
plt.grid(True)
plt.xticks(range(1, 9))
plt.tight_layout()
plt.savefig("benchmark_plot.png")
