import matplotlib.pyplot as plt
import subprocess
import time

def command_builder(num_clients, num_replicas, num_requests_per_client, CRDT_type):
  return ["./target/release/crdt", "-c", str(num_clients), "-r", str(num_replicas), "-n", str(num_requests_per_client), "-t", str(CRDT_type)]

def run(num_clients, num_replicas, num_requests_per_client, CRDT_type):
  start_time = time.time()
  subprocess.run(command_builder(num_clients, num_replicas, num_requests_per_client, CRDT_type))
  end_time = time.time()
  duration = end_time - start_time
  
  return duration

CLIENT_COUNTS = [10, 100]
REPLICA_COUNTS = [10,100]
REQUEST_PER_CLIENT = [100, 1000]


def run_test(crdt_type):
  for i in range(len(CLIENT_COUNTS)):
    n_clients, n_replicas = CLIENT_COUNTS[i], REPLICA_COUNTS[i]
    for req in REQUEST_PER_CLIENT:
      print(run(n_clients, n_replicas, req, crdt_type))

def main():
  print("G-counter")
  run_test(0)
  # print("l_seq")
  # run_test(1)
  print("OR-sets")
  run_test(2)
  
if __name__ == "__main__":
  main()