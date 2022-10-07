#Import the necessary modules
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import matplotlib.patches as mpatches
import csv
import math


#Match name of function and number of tag "indexnumber-begin" followed by "indexnumber-end"
S0="sensor measurement"
S1="use of LTL"
S2="sent by Lorawan"

#for csv file 		
CSV_energy=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
Time_total=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
Time_total_lora=[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]

for k in range(1,11):

	#capture n:		
	data = pd.read_csv('capture_n'+str(k)+'_sending_24_bytes_with_ltl.csv')
	df = pd.DataFrame(data)

	#Initialize the lists for X,Y and C 
	X = list(df.iloc[:, 0]) #Time column
	C = list(df.iloc[:, 4]) #UART column to find tag
	J = list(df.iloc[:, 3]) #Jul evolution column

	for i in range(len(C)):
		J[i]=J[i]*1000
		if(pd.isna(J[i])): #Fill blank of current value
			J[i]=J[i-1]
		if (isinstance(C[i], str)): #Find non empty cells in UART
			C[i]=C[i][:-4] #Remove special charactere ln
			
			#***running the UART string***
			if(C[i]=='0'):
				B0=X[i]
				Jb0=J[i]
			elif(C[i]=='0E'):
				E0=X[i]
				Jh0=J[i]
			elif(C[i]=='1'):
				B1=X[i]
				Jb1=J[i]
			elif(C[i]=='1E'):
				E1=X[i]
				Jh1=J[i]
			elif(C[i]=='2'):
				B2=X[i]
				Jb2=J[i]
			elif(C[i]=='2E'):
				E2=X[i]
				Jh2=J[i]
			
	CSV_energy[k-1]=Jh2-Jb0	
	Time_total[k-1]=E2-B0
	Time_total_lora[k-1]=E2-B2
	# end capture n*
	if k==1: #affichage display pour k==1
		deltaElegend_measure=Jh0-Jb0
		deltaElegend_buildligth=Jh1-Jb1
		deltaElegend_lssend=Jh2-Jb2	
		plt.figure(figsize=(16,8))		
		# Plot the data 
		plt.plot(X, J, color='c')

		#Create list for compare two array,Fill area under the curve, init legend
		if ('S0' in globals()):
			begin0 = np.full(len(df.index),B0)
			end0 = np.full(len(df.index),E0)
			plt.fill_between(X, J,0, where =(X>=begin0)&(X<=end0), color='#f9d8f7')
			pink_patch = mpatches.Patch(color='#f9d8f7', label=S0)
			tab=(pink_patch,)
		if ('S1' in globals()):	
			begin1 = np.full(len(df.index),B1)
			end1 = np.full(len(df.index),E1)
			plt.fill_between(X, J,0, where =(X>=begin1)&(X<=end1), color='#d8f9f8')
			green_patch = mpatches.Patch(color='#d8f9f8', label=S1)
			if ('tab' in globals()):
				tab=tab+(green_patch,)
			else: tab=(green_patch,)	
		if ('S2' in globals()):	
			begin2 = np.full(len(df.index),B2)
			end2 = np.full(len(df.index),E2)
			plt.fill_between(X, J,0, where =(X>=begin2)&(X<=end2), color='#f9f6d8')
			yellow_patch = mpatches.Patch(color='#f9f6d8', label=S2)
			
			if ('tab' in globals()):
				tab=tab+(yellow_patch,)
			else: tab=(yellow_patch,)




		#Personalize display

		plt.yticks(fontsize=30)
		plt.xticks(fontsize=30)
		plt.xlim(0,1.85)
		plt.ylim(0,0.4*1000)
		plt.grid()
		#plt.title("My plot")
		plt.xlabel("Time (s)",fontsize=30)
		plt.ylabel("Energy (mJ)",fontsize=30)

		plt.legend(handles=tab,title="Function:",fontsize=30,loc=5)

		plt.tight_layout()
		# Show the plot
		plt.savefig("fig4_with_ltl_with_sensors_energy.png")

		# plt.show()



#avec numpy: 
CSV_energy_average=np.mean(CSV_energy)
Total_time_average=np.mean(Time_total)
# CSV_energy_average=(CSV_energy[0]+CSV_energy[1]+CSV_energy[2]+CSV_energy[3]+CSV_energy[4]+CSV_energy[5]+CSV_energy[6]+CSV_energy[7]+CSV_energy[8]+CSV_energy[9])/10
# Total_time_average=(Time_total[0]+Time_total[1]+Time_total[2]+Time_total[3]+Time_total[4]+Time_total[5]+Time_total[6]+Time_total[7]+Time_total[8]+Time_total[9])/10

#Ecart type
Ecart=[100,100,100,100,100,100,100,100,100,100]
Ecart[0]=abs(CSV_energy_average-CSV_energy[0])
Ecart[1]=abs(CSV_energy_average-CSV_energy[1])
Ecart[2]=abs(CSV_energy_average-CSV_energy[2])
Ecart[3]=abs(CSV_energy_average-CSV_energy[3])
Ecart[4]=abs(CSV_energy_average-CSV_energy[4])
Ecart[5]=abs(CSV_energy_average-CSV_energy[5])
Ecart[6]=abs(CSV_energy_average-CSV_energy[6])
Ecart[7]=abs(CSV_energy_average-CSV_energy[7])
Ecart[8]=abs(CSV_energy_average-CSV_energy[8])
Ecart[9]=abs(CSV_energy_average-CSV_energy[9])

Ecartt=[100,100,100,100,100,100,100,100,100,100]
Ecartt[0]=Ecart[0]*Ecart[0]
Ecartt[1]=Ecart[1]*Ecart[1]
Ecartt[2]=Ecart[2]*Ecart[2]
Ecartt[3]=Ecart[3]*Ecart[3]
Ecartt[4]=Ecart[4]*Ecart[4]
Ecartt[5]=Ecart[5]*Ecart[5]
Ecartt[6]=Ecart[6]*Ecart[6]
Ecartt[7]=Ecart[7]*Ecart[7]
Ecartt[8]=Ecart[8]*Ecart[8]
Ecartt[9]=Ecart[9]*Ecart[9]

Sum_ecart=(Ecartt[0]+Ecartt[1]+Ecartt[2]+Ecartt[3]+Ecartt[4]+Ecartt[5]+Ecartt[6]+Ecartt[7]+Ecartt[8]+Ecartt[9])/10
Ecart_type_energy=math.sqrt(Sum_ecart)

Ecart0=[100,100,100,100,100,100,100,100,100,100]
Ecart0[0]=abs(Total_time_average-Time_total[0])
Ecart0[1]=abs(Total_time_average-Time_total[1])
Ecart0[2]=abs(Total_time_average-Time_total[2])
Ecart0[3]=abs(Total_time_average-Time_total[3])
Ecart0[4]=abs(Total_time_average-Time_total[4])
Ecart0[5]=abs(Total_time_average-Time_total[5])
Ecart0[6]=abs(Total_time_average-Time_total[6])
Ecart0[7]=abs(Total_time_average-Time_total[7])
Ecart0[8]=abs(Total_time_average-Time_total[8])
Ecart0[9]=abs(Total_time_average-Time_total[9])

Ecartt=[100,100,100,100,100,100,100,100,100,100]
Ecartt[0]=Ecart[0]*Ecart[0]
Ecartt[1]=Ecart[1]*Ecart[1]
Ecartt[2]=Ecart[2]*Ecart[2]
Ecartt[3]=Ecart[3]*Ecart[3]
Ecartt[4]=Ecart[4]*Ecart[4]
Ecartt[5]=Ecart[5]*Ecart[5]
Ecartt[6]=Ecart[6]*Ecart[6]
Ecartt[7]=Ecart[7]*Ecart[7]
Ecartt[8]=Ecart[8]*Ecart[8]
Ecartt[9]=Ecart[9]*Ecart[9]

Sum_ecart=(Ecartt[0]+Ecartt[1]+Ecartt[2]+Ecartt[3]+Ecartt[4]+Ecartt[5]+Ecartt[6]+Ecartt[7]+Ecartt[8]+Ecartt[9])/10
Ecart_type_time=math.sqrt(Sum_ecart)
#save energy in a csv file 
with open('energy_sending_24_bytes_with_ltl.csv', 'w', newline='') as file24bytes:

    # on déclare un objet writer 
    ecrivain = csv.writer(file24bytes)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['Capture number', 'Total energy (mJ)','Ecart type Total energy','Total time (s)', 'Ecart type Total time','Total time lora (s)','+'])
    # quelques lignes
    ecrivain.writerow([1,CSV_energy[0],Ecart[0],Time_total[0],Ecart0[0],Time_total_lora[0],'0'])
    ecrivain.writerow([2,CSV_energy[1],Ecart[1],Time_total[1],Ecart0[1],Time_total_lora[1],'0'])
    ecrivain.writerow([3,CSV_energy[2],Ecart[2],Time_total[2],Ecart0[2],Time_total_lora[2],'0'])
    ecrivain.writerow([4,CSV_energy[3],Ecart[3],Time_total[3],Ecart0[3],Time_total_lora[3],'0'])
    ecrivain.writerow([5,CSV_energy[4],Ecart[4],Time_total[4],Ecart0[4],Time_total_lora[4],'0'])
    ecrivain.writerow([6,CSV_energy[5],Ecart[5],Time_total[5],Ecart0[5],Time_total_lora[5],'0'])
    ecrivain.writerow([7,CSV_energy[6],Ecart[6],Time_total[6],Ecart0[6],Time_total_lora[6],'0'])
    ecrivain.writerow([8,CSV_energy[7],Ecart[7],Time_total[7],Ecart0[7],Time_total_lora[7],'0'])
    ecrivain.writerow([9,CSV_energy[8],Ecart[8],Time_total[8],Ecart0[8],Time_total_lora[8],'0'])
    ecrivain.writerow([10,CSV_energy[9],Ecart[9],Time_total[9],Ecart0[9],Time_total_lora[9],'0',])
    ecrivain.writerow(['average',CSV_energy_average,Ecart_type_energy,Total_time_average,Ecart_type_time,'0',])
    ecrivain.writerow(['average',CSV_energy_average,np.std(CSV_energy),Total_time_average,np.std(Time_total),'0',])
    ecrivain.writerow(['average',CSV_energy_average,np.var(CSV_energy),Total_time_average,np.var(Time_total),'0',])
    ecrivain.writerow(['legend_graph_2','delta E measure sensor:(mJ)',deltaElegend_measure,'delta E build_light:(mJ)',deltaElegend_buildligth,'delta E send lora:(mJ)',deltaElegend_lssend])
