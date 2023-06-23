import numpy as np
import matplotlib.pyplot as plt

OUTPUT_PATH = "unit1/output.txt"
data = np.loadtxt(OUTPUT_PATH)
plt.plot(data[:, 0], data[:, 1])
plt.show()
input("Press enter to exit")
