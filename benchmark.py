import matplotlib.pyplot as plt
import subprocess
import time
import seaborn as sns
import pandas as pd

sns.set_palette("deep")

def command_builder(num_clients, num_replicas, num_requests_per_client, CRDT_type):
  return ["./target/release/crdt", "-c", str(num_clients), "-r", str(num_replicas), "-n", str(num_requests_per_client), "-t", str(CRDT_type)]

def run(num_clients, num_replicas, num_requests_per_client, CRDT_type):
  start_time = time.time()
  subprocess.run(command_builder(num_clients, num_replicas, num_requests_per_client, CRDT_type))
  end_time = time.time()
  duration = end_time - start_time
  
  return duration

CLIENT_COUNTS = [1, 2, 10, 20]
REPLICA_COUNTS = [1, 2, 10, 20]
REQUEST_PER_CLIENT = [10, 50, 100]


GROUPS = [] 
for ct in CLIENT_COUNTS:
  for i in range(len(REQUEST_PER_CLIENT)):
    GROUPS.append(f"n_clients={ct}\nn_replicas={ct}")
CATEGORIES = [f"request per client={cat}" for cat in REQUEST_PER_CLIENT] * len(CLIENT_COUNTS)

# print(GROUPS)
# print(CATEGORIES)

def avg(nums):
  sum = 0;
  for num in nums:
    sum += num
  return sum / len(nums)


def run_test(crdt_type, num_rums, title, filename):
  total_times = []
  for i in range(len(CLIENT_COUNTS)):
    n_clients, n_replicas = CLIENT_COUNTS[i], REPLICA_COUNTS[i]
    time_for_each_rpc = []
    for req in REQUEST_PER_CLIENT:
      times = []
      for t in range(num_rums):
        times.append(run(n_clients, n_replicas, req, crdt_type))
      time_for_each_rpc.append(avg(times))
    total_times.extend(time_for_each_rpc)
  print(total_times)
  sns.catplot(x=GROUPS, y=total_times, hue=CATEGORIES, errorbar=None, kind="bar")
  plt.yscale("log")
  plt.ylabel("time (s)")
  plt.xlabel("number of nodes")
  plt.title(title)
  plt.savefig(filename, bbox_inches='tight', dpi=300)
  
      
    
def main():
  
  print("RUNNING G COUNTER")
  run_test(0, 1, "G-Counter Benchmark", "./graphs/gcounter.png")
  print("RUNNING LSEQ")
  run_test(1, 1, "LSeq Benchmark", "./graphs/lseq.png")
  print("OR-sets")
  run_test(2, 1, "OR-Set Benchmark", "./graphs/orset.png")
  
if __name__ == "__main__":
  main()
