#Import the necessary modules
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import matplotlib.patches as mpatches
import csv
import math

CSV_energy=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
Time_total=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]

data = pd.read_csv('48_bytes_withoutLTL_10_mes.csv')
df = pd.DataFrame(data)
X = list(df.iloc[:, 0]) #Time column
C = list(df.iloc[:, 4]) #UART column to find tag
J = list(df.iloc[:, 3]) #Jul evolution column
mesure=1
for i in range(len(C)):
	J[i]=J[i]*1000
	if(pd.isna(J[i])): #Fill blank of current value
		J[i]=J[i-1]
	if (isinstance(C[i], str)): #Find non empty cells in UART
		C[i]=C[i][:-4] #Remove special charactere ln
		#***running the UART string***
		if(C[i]=='0'):
			debut_mesure=X[i]
			Jb0=J[i]
		elif(C[i]=='2E'):
			fin_mesure=X[i]
			Jh2=J[i]
			Time_total[mesure-1]=fin_mesure-debut_mesure
			CSV_energy[mesure-1]=Jh2-Jb0
			mesure=mesure+1
Moy_energy=np.mean(CSV_energy)
Moy_time=np.mean(Time_total)
Ecarttype_energy=np.std(CSV_energy)
#Ecarttype_time=np.std(Time_total)
					
					
# ouverture en écriture d'un fichier
with open('energy_tot_48_bytes_without.csv', 'w', newline='') as filebytes_without:

    # on déclare un objet writer 
    ecrivain = csv.writer(filebytes_without)

    # écrire une ligne dans le fichier:
    ecrivain.writerow(['Capture number', 'Delta E(J)', 'Time (s)', 'Ecart à la moyenne (J)'])
    # quelques lignes:
    ecrivain.writerow([ 1, CSV_energy[0],Time_total[0],''])
    ecrivain.writerow([ 2, CSV_energy[1],Time_total[1],''])
    ecrivain.writerow([ 3, CSV_energy[2],Time_total[2],''])
    ecrivain.writerow([ 4, CSV_energy[3],Time_total[3],''])
    ecrivain.writerow([ 5, CSV_energy[4],Time_total[4],''])
    ecrivain.writerow([ 6, CSV_energy[5],Time_total[5],''])
    ecrivain.writerow([ 7, CSV_energy[6],Time_total[6],''])
    ecrivain.writerow([ 8, CSV_energy[7],Time_total[7],''])
    ecrivain.writerow([ 9, CSV_energy[8],Time_total[8],''])
    ecrivain.writerow([ 10, CSV_energy[9],Time_total[9],''])			
    ecrivain.writerow([ "Average and ecart type", Moy_energy,Moy_time,Ecarttype_energy])

