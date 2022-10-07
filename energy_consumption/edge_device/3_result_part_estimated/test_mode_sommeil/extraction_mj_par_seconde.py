#Import the necessary modules
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import matplotlib.patches as mpatches
import csv
import math


CSV_energy=[100,100,100,100,100,100,100,100,100,100]
Time_total=[100,100,100,100,100,100,100,100,100,100]

#Create a df		
data = pd.read_csv('long_period_sleep.csv')
df = pd.DataFrame(data)

#Initialize the lists for X,Y and C 
X = list(df.iloc[:, 0]) #Time column
J = list(df.iloc[:, 3]) #Energy evolution column

for i in range(len(X)):
	if(X[i]==0):
		Jb0=J[i]
	elif(X[i]==100):
		Jh100=J[i]
		
deltamJ100secondes=(Jh100-Jb0)*1000
mJ1seconde=deltamJ100secondes/100
print('Le mode eadle consomme ',mJ1seconde,'mJ par seconde') 
# ouverture en écriture d'un fichier
with open('enrgy_mj_seconde_static.csv', 'w', newline='') as files:

    # on déclare un objet writer 
    ecrivain = csv.writer(files)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['0','0'])
    ecrivain.writerow([mJ1seconde,'est la conso en mJ par seconde en mode repos'])
