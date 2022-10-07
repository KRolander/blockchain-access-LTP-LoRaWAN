#Import the necessary modules
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import matplotlib.patches as mpatches
import csv
import math


#Match name of function and number of tag "indexnumber-begin" followed by "indexnumber-end"
#S0="sensor measurement"
S1="use of LTL"
S2="sent by Lorawan"

#for csv file 		
CSV_energy=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
Time_total=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_LTL_tot=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_LRW=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_encoding_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_hash_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_sign_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_enconding_payload=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]

#for mesure tag
data = pd.read_csv('4_bytes_10_mes.csv')
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
		elif(C[i]=='3'):
			B3=X[i]
			Jb3=J[i]
			xx=1
		elif(C[i]=='3E'):
			E3=X[i]
			Jh3=J[i]
		elif(C[i]=='4'):
			B4=X[i]
			Jb4=J[i]
		elif(C[i]=='4E'):
			E4=X[i]
			Jh4=J[i]
		elif(C[i]=='5'):
			B5=X[i]
			Jb5=J[i]
		elif(C[i]=='5E'):
			E5=X[i]
			Jh5=J[i]
		elif(C[i]=='6'):
			B6=X[i]
			Jb6=J[i]
		elif(C[i]=='6E'):
			Jh6=J[i]
			E6=X[i]
		elif(C[i]=='2'):
			Jb2=J[i]
			E2=X[i]	
		elif(C[i]=='2E'):
			fin_mesure=X[i]
			Jh2=J[i]
			Time_total[mesure-1]=fin_mesure-debut_mesure
			CSV_energy[mesure-1]=Jh2-Jb0
			energy_LTL_tot[mesure-1]=Jh6-Jb3
			energy_LRW[mesure-1]=Jh2-Jb2
			energy_encoding_message[mesure-1]=Jh3-Jb3
			energy_hash_message[mesure-1]=Jh4-Jb4
			energy_sign_message[mesure-1]=Jh5-Jb5
			energy_enconding_payload[mesure-1]=Jh6-Jb6
			mesure=mesure+1
			#print(mesure)
Moy_energy=np.mean(CSV_energy)
Moy_time=np.mean(Time_total)
moy_energy_LTL_tot=np.mean(energy_LTL_tot)
moy_energy_LRW=np.mean(energy_LRW)
moy_energy_encoding_message=np.mean(energy_encoding_message)
moy_energy_hash_message=np.mean(energy_hash_message)
moy_energy_sign_message=np.mean(energy_sign_message)
moy_energy_enconding_payload=np.mean(energy_enconding_payload)
Ecarttype_energy=np.std(CSV_energy)
Ecarttype_time=np.std(Time_total)

# ouverture en écriture d'un fichier
with open('energy_tot_4_bytes.csv', 'w', newline='') as file4bytes:

    # on déclare un objet writer 
    ecrivain = csv.writer(file4bytes)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['Capture number', 'Delta E(mJ)', 'Time (s)', 'Ecart type (mJ)'])
    # quelques lignes:
    ecrivain.writerow([ 1, CSV_energy[0],Time_total[0],'Ecart type energy'])
    ecrivain.writerow([ 2, CSV_energy[1],Time_total[1],Ecarttype_energy])
    ecrivain.writerow([ 3, CSV_energy[2],Time_total[2],'Ecart type temps'])
    ecrivain.writerow([ 4, CSV_energy[3],Time_total[3],Ecarttype_time])
    ecrivain.writerow([ 5, CSV_energy[4],Time_total[4],'nan'])
    ecrivain.writerow([ 6, CSV_energy[5],Time_total[5],'nan'])
    ecrivain.writerow([ 7, CSV_energy[6],Time_total[6],'nan'])
    ecrivain.writerow([ 8, CSV_energy[7],Time_total[7],'nan'])
    ecrivain.writerow([ 9, CSV_energy[8],Time_total[8],'nan'])
    ecrivain.writerow([ 10, CSV_energy[9],Time_total[9],'nan'])			
    ecrivain.writerow([ "Average ", Moy_energy,Moy_time,'nan'])	
#for csv file 
		
CSV_energy88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
Time_total88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_LTL_tot88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_LRW88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_encoding_message88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_hash_message88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_sign_message88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
energy_enconding_payload88=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
#for mesure tag
data = pd.read_csv('88_bytes_10_mes.csv')
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
		elif(C[i]=='3'):
			B3=X[i]
			Jb3=J[i]
			xx=1
		elif(C[i]=='3E'):
			E3=X[i]
			Jh3=J[i]
		elif(C[i]=='4'):
			B4=X[i]
			Jb4=J[i]
		elif(C[i]=='4E'):
			E4=X[i]
			Jh4=J[i]
		elif(C[i]=='5'):
			B5=X[i]
			Jb5=J[i]
		elif(C[i]=='5E'):
			E5=X[i]
			Jh5=J[i]
		elif(C[i]=='6'):
			B6=X[i]
			Jb6=J[i]
		elif(C[i]=='6E'):
			Jh6=J[i]
			E6=X[i]
		elif(C[i]=='2'):
			Jb2=J[i]
			E2=X[i]	
		elif(C[i]=='2E'):
			fin_mesure=X[i]
			Jh2=J[i]
			Time_total[mesure-1]=fin_mesure-debut_mesure
			CSV_energy[mesure-1]=Jh2-Jb0
			energy_LTL_tot88[mesure-1]=Jh6-Jb3
			energy_LRW88[mesure-1]=Jh2-Jb2
			energy_encoding_message88[mesure-1]=Jh3-Jb3
			energy_hash_message88[mesure-1]=Jh4-Jb4
			energy_sign_message88[mesure-1]=Jh5-Jb5
			energy_enconding_payload88[mesure-1]=Jh6-Jb6
			mesure=mesure+1
Moy_energy88=np.mean(CSV_energy88)
Moy_time88=np.mean(Time_total88)
moy_energy_LTL_tot88=np.mean(energy_LTL_tot88)
moy_energy_LRW88=np.mean(energy_LRW88)
moy_energy_encoding_message88=np.mean(energy_encoding_message88)
moy_energy_hash_message88=np.mean(energy_hash_message88)
moy_energy_sign_message88=np.mean(energy_sign_message88)
moy_energy_enconding_payload88=np.mean(energy_enconding_payload88)
Ecarttype_energy88=np.std(CSV_energy88)
Ecarttype_time88=np.std(Time_total88)

# ouverture en écriture d'un fichier
with open('energy_tot_88_bytes.csv', 'w', newline='') as file88bytes:

    # on déclare un objet writer 
    ecrivain = csv.writer(file88bytes)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['Capture number', 'Delta E(mJ)', 'Time (s)', 'Ecart type (mJ)'])
    # quelques lignes:
    ecrivain.writerow([ 1, CSV_energy88[0],CSV_energy88[0],'Ecart type energy'])
    ecrivain.writerow([ 2, CSV_energy88[1],CSV_energy88[1],Ecarttype_energy88])
    ecrivain.writerow([ 3, CSV_energy88[2],CSV_energy88[2],'Ecart type temps'])
    ecrivain.writerow([ 4, CSV_energy88[3],CSV_energy88[3],Ecarttype_time88])
    ecrivain.writerow([ 5, CSV_energy88[4],CSV_energy88[4],'nan'])
    ecrivain.writerow([ 6, CSV_energy88[5],CSV_energy88[5],'nan'])
    ecrivain.writerow([ 7, CSV_energy88[6],CSV_energy88[6],'nan'])
    ecrivain.writerow([ 8, CSV_energy88[7],CSV_energy88[7],'nan'])
    ecrivain.writerow([ 9, CSV_energy88[8],CSV_energy88[8],'nan'])
    ecrivain.writerow([ 10, CSV_energy88[9],CSV_energy88[9],'nan'])			
    ecrivain.writerow([ "Average ", Moy_energy88,Moy_time88,'nan'])
    
# ouverture en écriture d'un fichier
with open('tab_4_and_88_bytes.csv', 'w', newline='') as file4n88bytes:

    # on déclare un objet writer 
    ecrivain = csv.writer(file4n88bytes)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['Capture number (4 bytes)', 'E encoding message(mJ)', 'E hash message(mJ)', 'E sign message(mJ)','E encoding payload(mJ)', 'E tot LTL (mJ)', 'E LRW (mJ)'])
    # quelques lignes:
    ecrivain.writerow([ 1, energy_encoding_message[0],energy_hash_message[0],energy_sign_message[0],energy_enconding_payload[0],energy_LTL_tot[0],energy_LRW[0]])
    ecrivain.writerow([ 2, energy_encoding_message[1],energy_hash_message[1],energy_sign_message[1],energy_enconding_payload[1],energy_LTL_tot[1],energy_LRW[1]])
    ecrivain.writerow([ 3, energy_encoding_message[2],energy_hash_message[2],energy_sign_message[2],energy_enconding_payload[2],energy_LTL_tot[2],energy_LRW[2]])
    ecrivain.writerow([ 4, energy_encoding_message[3],energy_hash_message[3],energy_sign_message[3],energy_enconding_payload[3],energy_LTL_tot[3],energy_LRW[3]])
    ecrivain.writerow([ 5, energy_encoding_message[4],energy_hash_message[4],energy_sign_message[4],energy_enconding_payload[4],energy_LTL_tot[4],energy_LRW[4]])
    ecrivain.writerow([ 6, energy_encoding_message[5],energy_hash_message[5],energy_sign_message[5],energy_enconding_payload[5],energy_LTL_tot[5],energy_LRW[5]])
    ecrivain.writerow([ 7, energy_encoding_message[6],energy_hash_message[6],energy_sign_message[6],energy_enconding_payload[6],energy_LTL_tot[6],energy_LRW[6]])
    ecrivain.writerow([ 8, energy_encoding_message[7],energy_hash_message[7],energy_sign_message[7],energy_enconding_payload[7],energy_LTL_tot[7],energy_LRW[7]])
    ecrivain.writerow([ 9, energy_encoding_message[8],energy_hash_message[8],energy_sign_message[8],energy_enconding_payload[8],energy_LTL_tot[8],energy_LRW[8]])
    ecrivain.writerow([ 10, energy_encoding_message[9],energy_hash_message[9],energy_sign_message[9],energy_enconding_payload[9],energy_LTL_tot[9],energy_LRW[9]])			
    ecrivain.writerow([ "Average ",moy_energy_encoding_message, moy_energy_hash_message ,moy_energy_sign_message,moy_energy_enconding_payload,moy_energy_LTL_tot,moy_energy_LRW])
    ecrivain.writerow(['', '', '', '','', '', ''])
    ecrivain.writerow(['Capture number (88 bytes)', 'E encoding message(mJ)', 'E hash message(mJ)', 'E sign message(mJ)','E encoding payload(mJ)', 'E tot LTL (mJ)', 'E LRW (mJ)'])
    ecrivain.writerow([ 1, energy_encoding_message88[0],energy_hash_message88[0],energy_sign_message88[0],energy_enconding_payload88[0],energy_LTL_tot88[0],energy_LRW88[0]])
    ecrivain.writerow([ 2, energy_encoding_message88[1],energy_hash_message88[1],energy_sign_message88[1],energy_enconding_payload88[1],energy_LTL_tot88[1],energy_LRW88[1]])
    ecrivain.writerow([ 3, energy_encoding_message88[2],energy_hash_message88[2],energy_sign_message88[2],energy_enconding_payload88[2],energy_LTL_tot88[2],energy_LRW88[2]])
    ecrivain.writerow([ 4, energy_encoding_message88[3],energy_hash_message88[3],energy_sign_message88[3],energy_enconding_payload88[3],energy_LTL_tot88[3],energy_LRW88[3]])
    ecrivain.writerow([ 5, energy_encoding_message88[4],energy_hash_message88[4],energy_sign_message88[4],energy_enconding_payload88[4],energy_LTL_tot88[4],energy_LRW88[4]])  
    ecrivain.writerow([ 6, energy_encoding_message88[5],energy_hash_message88[5],energy_sign_message88[5],energy_enconding_payload88[5],energy_LTL_tot88[5],energy_LRW88[5]])
    ecrivain.writerow([ 7, energy_encoding_message88[6],energy_hash_message88[6],energy_sign_message88[6],energy_enconding_payload88[6],energy_LTL_tot88[6],energy_LRW88[6]])
    ecrivain.writerow([ 8, energy_encoding_message88[7],energy_hash_message88[7],energy_sign_message88[7],energy_enconding_payload88[7],energy_LTL_tot88[7],energy_LRW88[7]])
    ecrivain.writerow([ 9, energy_encoding_message88[8],energy_hash_message88[8],energy_sign_message88[8],energy_enconding_payload88[8],energy_LTL_tot88[8],energy_LRW88[8]])
    ecrivain.writerow([ 10, energy_encoding_message88[9],energy_hash_message88[9],energy_sign_message88[9],energy_enconding_payload88[9],energy_LTL_tot88[9],energy_LRW88[9]])
    ecrivain.writerow([ "Average ",moy_energy_encoding_message88, moy_energy_hash_message88 ,moy_energy_sign_message88,moy_energy_enconding_payload88,moy_energy_LTL_tot88,moy_energy_LRW88])      
    
    
    
    
    
    
    
    
