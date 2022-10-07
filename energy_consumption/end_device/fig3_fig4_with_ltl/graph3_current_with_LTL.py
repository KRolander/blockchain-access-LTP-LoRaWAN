#Import the necessary modules
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import matplotlib.patches as mpatches
import csv

#Match name of function and number of tag "indexnumber-begin" followed by "indexnumber-end"

S0="sensor measurement"
S1="use of LTL"
S2="sent by Lorawan"

data = pd.read_csv('capture_n1_sending_24_bytes_with_ltl.csv')
df = pd.DataFrame(data)

#Initialize the lists for X,Y and C 
X = list(df.iloc[:, 0]) #Time column
Y = list(df.iloc[:, 1]) #Current evolution column
C = list(df.iloc[:, 4]) #UART column to find tag

#to find higher current 
c0=0
c1=0
c2=0
xx2=0
xx1=0
xx0=0

for i in range(len(C)):
	Y[i]=Y[i]*1000
	if(pd.isna(Y[i])): #Fill blank of current value
		Y[i]=Y[i-1]
	if (isinstance(C[i], str)): #Find non empty cells in UART
		C[i]=C[i][:-4] #Remove special charactere ln
	
		#***running the UART string***
		if(C[i]=='0'):
			B0=X[i]
			xx0=1
			cmoy0=Y[i]
			i0=1
		elif(C[i]=='0E'):
			E0=X[i]
			xx0=0	
		elif(C[i]=='1'):
			B1=X[i]
			xx1=1
			cmoy1=Y[i]
			i1=1
		elif(C[i]=='1E'):
			E1=X[i]
			xx1=0
		elif(C[i]=='2'):
			B2=X[i]
			xx2=1
			cmoy2=Y[i]
			i2=1
		elif(C[i]=='2E'):
			E2=X[i]
			xx2=0
		
	if(xx0==1):
		cmoy0=cmoy0+Y[i]
		i0=i0+1	
		if(Y[i]>c0):
			c0=Y[i]
	elif(xx1==1):
		cmoy1=cmoy1+Y[i]
		i1=i1+1
		if(Y[i]>c1):
			c1=Y[i]
	elif(xx2==1):
		cmoy2=cmoy2+Y[i]
		i2=i2+1
		if(Y[i]>c2):
			c2=Y[i]		
		
cmoy0=cmoy0/i0
cmoy1=cmoy1/i1
cmoy2=cmoy2/i2	

plt.figure(figsize=(16,8))		
# Plot the data 
plt.plot(X, Y, color='c')

#Create list for compare two array,Fill area under the curve, init legend
if ('S0' in globals()):
	begin0 = np.full(len(df.index),B0)
	end0 = np.full(len(df.index),E0)
	plt.fill_between(X, Y,0, where =(X>=begin0)&(X<=end0), color='#f9d8f7')
	pink_patch = mpatches.Patch(color='#f9d8f7', label=S0)
	tab=(pink_patch,)	
if ('S1' in globals()):	
	begin1 = np.full(len(df.index),B1)
	end1 = np.full(len(df.index),E1)
	plt.fill_between(X, Y,0, where =(X>=begin1)&(X<=end1), color='#d8f9f8')
	green_patch = mpatches.Patch(color='#d8f9f8', label=S1)
	if ('tab' in globals()):
		tab=tab+(green_patch,)
	else: tab=(green_patch,)	
if ('S2' in globals()):	
	begin2 = np.full(len(df.index),B2)
	end2 = np.full(len(df.index),E2)
	plt.fill_between(X, Y,0, where =(X>=begin2)&(X<=end2), color='#f9f6d8')
	yellow_patch = mpatches.Patch(color='#f9f6d8', label=S2)
	
	if ('tab' in globals()):
		tab=tab+(yellow_patch,)
	else: tab=(yellow_patch,)

#Personalize display
plt.yticks(fontsize=30)
plt.xticks(fontsize=30)
plt.grid()
plt.xlim(0,1.85)
plt.ylim(0,100)
#plt.title("My plot")
plt.xlabel("Time (s)",fontsize=30)
plt.ylabel("Current (mA)",fontsize=30)
plt.legend(bbox_to_anchor=(1,0.5),loc=5,handles=tab,title="Function:",fontsize=30)

plt.tight_layout()
# Show the plot
plt.savefig("fig3_with_ltl_with_sensors_current.png")

plt.show()

with open('legend_current_sending_24_bytes_with_ltl.csv', 'w', newline='') as file24bytes:

    # on déclare un objet writer 
    ecrivain = csv.writer(file24bytes)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['Max current sensors:(mA)', 'Max current ltl:(mA)','Max current sensors:lsSend(mA)'])
    ecrivain.writerow([c0,c1,c2])
    ecrivain.writerow(['time sensors:(ms)', 'time  ltl:(ms)','time lsSend(ms)'])
    ecrivain.writerow([(E0-B0)*1000,(E1-B1)*1000,(E2-B2)*1000])
    ecrivain.writerow(['Moy current sensors:(mA)', 'Moy current ltl:(mA)','Moy current sensors:lsSend(mA)'])
    ecrivain.writerow([cmoy0,cmoy1,cmoy2])
