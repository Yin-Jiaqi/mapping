import random
import math
import infix_to_prefix
import subprocess
import numpy as np


# UCB MAB Class
class UCB_MAB:
  def __init__(self, num_arms, c, sample_gate):
    self.num_arms = num_arms
    self.c = c  # Exploration parameter for UCB
    self.q_values = [0.0] * num_arms
    self.counts = [0] * num_arms
    self.sample_gate = sample_gate
  
  def select_action(self):
    selected_cells = set()

    # Exploration (ensure each arm is tried at least once)
    for arm in range(self.num_arms):
        if self.counts[arm] == 0:
            selected_cells.add(arm)
            if len(selected_cells) == self.sample_gate:
                break

    # Exploitation (choose the remaining based on UCB)
    remaining_cells = [arm for arm in range(self.num_arms) if arm not in selected_cells]
    total_counts = sum(self.counts)
    #print(total_counts)
    ucb_values = [0.0] * self.num_arms
    for arm in remaining_cells:
        if self.counts[arm] >0:
            average_reward = self.q_values[arm]
            ucb_values[arm] = average_reward + self.c * math.sqrt(math.log(total_counts) / self.counts[arm])
    while len(selected_cells) < self.sample_gate:
        if all(math.isinf(val) or math.isnan(val) for val in ucb_values):
            selected_cell = random.choice(remaining_cells)
        else:
            selected_cell = ucb_values.index(max(ucb_values))
        if selected_cell not in selected_cells:
            selected_cells.add(selected_cell)
            ucb_values[selected_cell] = float('-inf')
        
    return list(selected_cells)

  def update(self, selected_arm, reward):
    for arm in selected_arm:
        self.counts[arm] += 1
        self.q_values[arm] = (self.q_values[arm] * self.counts[arm] + reward) / self.counts[arm]



num_arm=len(infix_to_prefix.parse_genlib("7nm.genlib"))
mab = UCB_MAB(num_arm, c=2, sample_gate=round(0.65*num_arm))  
best_cells = None 
best_area=None
best_reward = -float('inf') 
infix_to_prefix.main("7nm.genlib",None)
command=["cargo","run","--","c432"]
result = subprocess.run(command, capture_output=True, text=True)
max_area=np.min([float(i) for i in result.stdout.split("|")[:-1]])*1.2
# Main Loop
num_iterations = 300

for i in range(num_iterations):
  print("Iteration: ", i)
  selected_cells = mab.select_action()
  infix_to_prefix.main("7nm.genlib",selected_cells)
  command=["cargo","run","--","c432"]
  result = subprocess.run(command, capture_output=True, text=True)
  try:
     area=np.min([float(i) for i in result.stdout.split("|")[:-1]])
     reward=-np.sqrt(area/max_area)
     print(f"area:{area},",end="")
  except:
     reward=float('-inf')
     print(f"area:None,",end="")
  if reward > best_reward:
      best_reward = reward
      best_cells = selected_cells
      best_area=area
  print(f"Current best area: {best_area}")
  mab.update(selected_cells, reward)

print("Best Cells:", best_cells)
print("Best Reward:", best_reward)