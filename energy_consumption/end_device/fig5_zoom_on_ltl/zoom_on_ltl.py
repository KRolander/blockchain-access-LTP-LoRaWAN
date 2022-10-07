#Import the necessary modules
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import matplotlib.patches as mpatches
import csv

#Match name of function and number of tag "indexnumber-begin" followed by "indexnumber-end"
#S=("Initialisation","build_light_payload()","lsSendLoraWan()","generate_pub_keys()","hash_data()","sign_data()","encoding_protobuff()")

S3="encoding message"
S4="hash message"
S5="sign message"
S6="encoding payload"
#for csv file 
data = pd.read_csv('zoom_ltl_10_mes.csv')
df = pd.DataFrame(data)
X = list(df.iloc[:, 0]) #Time column
C = list(df.iloc[:, 4]) #UART column to find tag
J = list(df.iloc[:, 3]) #Jul evolution column	
Y = list(df.iloc[:, 1]) #Current evolution column	
CSV_energy_encoding_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_energy_hash_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_energy_sign_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_energy_encoding_payload=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_time_encoding_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_time_hash_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_time_sign_message=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
CSV_time_encoding_payload=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
TOT_energy=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
TOT_time=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
cmoy=0.0
ii=0
mesure=1
xx=0
Jj6=1
for i in range(len(C)):
	J[i]=J[i]*1000
	#X[i]=X[i]*1000
	Y[i]=Y[i]*1000
	if(pd.isna(J[i])): #Fill blank of current value
		J[i]=J[i-1]
	if (isinstance(C[i], str)): #Find non empty cells in UART
		C[i]=C[i][:-4] #Remove special charactere ln
		#***running the UART string***
		if(C[i]=='3'):
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
			CSV_energy_encoding_message[mesure-1]=Jh3-Jb3
			CSV_energy_hash_message[mesure-1]=Jh4-Jb4
			CSV_energy_sign_message[mesure-1]=Jh5-Jb5
			CSV_energy_encoding_payload[mesure-1]=Jh6-Jb6
			CSV_time_encoding_message[mesure-1]=(E3-B3)*1000
			CSV_time_hash_message[mesure-1]=(E4-B4)*1000
			CSV_time_sign_message[mesure-1]=(E5-B5)*1000
			CSV_time_encoding_payload[mesure-1]=(E6-B6)*1000
			TOT_energy[mesure-1]=Jh6-Jb3
			TOT_time[mesure-1]=(E6-B3)*1000
			mesure=mesure+1
			xx=0
	if(xx==1):
		cmoy=cmoy+Y[i]
		ii=ii+1
	if (mesure==2): #affichage display pour k==1	
		if (Jj6==1):
			plt.figure(figsize=(16,8))	
			plt.plot(X, Y, color='c')	
			if ('S3' in globals()):
				#colouring 	
				begin3 = np.full(len(df.index),B3)
				end3 = np.full(len(df.index),E3)
				plt.fill_between(X, Y,0, where =(X>=begin3)&(X<=end3), color='#dede00')
				#legend
				red_patch = mpatches.Patch(color='#dede00', label=S3) 
				if ('tab' in globals()):
					tab=tab+(red_patch,)
				else: tab=(red_patch,)
				
			if ('S4' in globals()):
				#colouring 	
				begin4 = np.full(len(df.index),B4)
				end4 = np.full(len(df.index),E4)
				plt.fill_between(X, Y,0, where =(X>=begin4)&(X<=end4), color='#c1005d')
				#legend
				blue_patch = mpatches.Patch(color='#c1005d', label=S4)
				tab=tab+(blue_patch,)
				
			if ('S5' in globals()):
				#colouring 	
				begin5 = np.full(len(df.index),B5)
				end5 = np.full(len(df.index),E5)
				plt.fill_between(X, Y,0, where =(X>=begin5)&(X<=end5), color='#ffe7fe')	
				#legend
				darkgreen_patch = mpatches.Patch(color='#ffe7fe', label=S5)
				
				tab=tab+(darkgreen_patch,)
				
			if ('S6' in globals()):
				#colouring 	
				begin6 = np.full(len(df.index),B6)
				end6 = np.full(len(df.index),E6)
				plt.fill_between(X, Y,0, where =(X>=begin6)&(X<=end6), color='#00ffec')	
				#legend
				color_patch = mpatches.Patch(color='#00ffec', label=S6)
				tab=tab+(color_patch,)
			plt.yticks(fontsize=30)
			plt.xticks(fontsize=30)
			plt.xlim(0,(0.105*1))
			plt.ylim(0,(0.016*1000))
			plt.grid()
			plt.xlabel("Time(ms)",fontsize=30)
			plt.ylabel("Current(mA)",fontsize=30)
			plt.legend(handles=tab,title="Function:",fontsize=30, loc=8)
			plt.tight_layout()
			plt.savefig("fig5_zoom_on_ltl.png")
			Jj6=0
			# plt.show()

cmoy=cmoy/ii
average=[-1,-1,-1,-1,-1,-1,-1,-1]
average[0]=np.mean(CSV_energy_encoding_message)
average[1]=np.mean(CSV_energy_hash_message)
average[2]=np.mean(CSV_energy_sign_message)
average[3]=np.mean(CSV_energy_encoding_payload)
average[4]=np.mean(CSV_time_encoding_message)
average[5]=np.mean(CSV_time_hash_message)
average[6]=np.mean(CSV_time_sign_message)
average[7]=np.mean(CSV_time_encoding_payload)
		
TOT_energy_val=np.mean(TOT_energy)
TOT_time_val=np.mean(TOT_time)

# ouverture en écriture d'un fichier
with open('energy_zoom_ltl.csv', 'w', newline='') as file24bytes:

    # on déclare un objet writer 
    ecrivain = csv.writer(file24bytes)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['Capture number', 'encoding message (mE)','hash message (mE)','sign message (mE)', 'encoding payload (mE)','encoding message (ms)','hash message (ms)','sign message (ms)', 'encoding payload (ms)'])
    # quelques lignes
    ecrivain.writerow([1,CSV_energy_encoding_message[0],CSV_energy_hash_message[0],CSV_energy_sign_message[0],CSV_energy_encoding_payload[0],CSV_time_encoding_message[0],CSV_time_hash_message[0],CSV_time_sign_message[0],CSV_time_encoding_payload[0]])
    ecrivain.writerow([2,CSV_energy_encoding_message[1],CSV_energy_hash_message[1],CSV_energy_sign_message[1],CSV_energy_encoding_payload[1],CSV_time_encoding_message[1],CSV_time_hash_message[1],CSV_time_sign_message[1],CSV_time_encoding_payload[1]])
    ecrivain.writerow([3,CSV_energy_encoding_message[2],CSV_energy_hash_message[2],CSV_energy_sign_message[2],CSV_energy_encoding_payload[2],CSV_time_encoding_message[2],CSV_time_hash_message[2],CSV_time_sign_message[2],CSV_time_encoding_payload[2]])
    ecrivain.writerow([4,CSV_energy_encoding_message[3],CSV_energy_hash_message[3],CSV_energy_sign_message[3],CSV_energy_encoding_payload[3],CSV_time_encoding_message[3],CSV_time_hash_message[3],CSV_time_sign_message[3],CSV_time_encoding_payload[3]])
    ecrivain.writerow([5,CSV_energy_encoding_message[4],CSV_energy_hash_message[4],CSV_energy_sign_message[4],CSV_energy_encoding_payload[4],CSV_time_encoding_message[4],CSV_time_hash_message[4],CSV_time_sign_message[4],CSV_time_encoding_payload[4]])
    ecrivain.writerow([6,CSV_energy_encoding_message[5],CSV_energy_hash_message[5],CSV_energy_sign_message[5],CSV_energy_encoding_payload[5],CSV_time_encoding_message[5],CSV_time_hash_message[5],CSV_time_sign_message[5],CSV_time_encoding_payload[5]])
    ecrivain.writerow([7,CSV_energy_encoding_message[6],CSV_energy_hash_message[6],CSV_energy_sign_message[6],CSV_energy_encoding_payload[6],CSV_time_encoding_message[6],CSV_time_hash_message[6],CSV_time_sign_message[6],CSV_time_encoding_payload[6]])
    ecrivain.writerow([8,CSV_energy_encoding_message[7],CSV_energy_hash_message[7],CSV_energy_sign_message[7],CSV_energy_encoding_payload[7],CSV_time_encoding_message[7],CSV_time_hash_message[7],CSV_time_sign_message[7],CSV_time_encoding_payload[7]])
    ecrivain.writerow([9,CSV_energy_encoding_message[8],CSV_energy_hash_message[8],CSV_energy_sign_message[8],CSV_energy_encoding_payload[8],CSV_time_encoding_message[8],CSV_time_hash_message[8],CSV_time_sign_message[8],CSV_time_encoding_payload[8]])
    ecrivain.writerow([10,CSV_energy_encoding_message[9],CSV_energy_hash_message[9],CSV_energy_sign_message[9],CSV_energy_encoding_payload[9],CSV_time_encoding_message[9],CSV_time_hash_message[9],CSV_time_sign_message[9],CSV_time_encoding_payload[9]])
    ecrivain.writerow(['average',average[0],average[1],average[2],average[3],average[4],average[5],average[6],average[7]])
    ecrivain.writerow(['average current',cmoy,"energy tot moy: (mE)",TOT_energy_val,"time tot moy: (ms)",TOT_time_val,"","",""]) 
