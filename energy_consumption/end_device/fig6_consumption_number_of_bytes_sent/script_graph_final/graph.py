import matplotlib.pyplot as plt
import pandas as pd

data_energy20 = pd.read_csv('energy_tot_4_bytes.csv')
df_energy20 = pd.DataFrame(data_energy20)
e20=df_energy20.iat[10,1] #energy
et20=df_energy20.iat[10,3] #ecartype

data_energy40 = pd.read_csv('energy_tot_32_bytes.csv')
df_energy40 = pd.DataFrame(data_energy40)
e40=df_energy40.iat[10,1] #energy
et40=df_energy40.iat[10,3] #ecartype

data_energy60 = pd.read_csv('energy_tot_48_bytes.csv')
df_energy60 = pd.DataFrame(data_energy60)
e60=df_energy60.iat[10,1]#energy
et60=df_energy60.iat[10,3] #ecartype

data_energy80 = pd.read_csv('energy_tot_64_bytes.csv')
df_energy80 = pd.DataFrame(data_energy80)
e80=df_energy80.iat[10,1] #energy
et80=df_energy80.iat[10,3] #ecartype

data_energy100 = pd.read_csv('energy_tot_88_bytes.csv')
df_energy100 = pd.DataFrame(data_energy100)
e100=df_energy100.iat[10,1] #energy
et100=df_energy100.iat[10,3]#ecartype
#test:
data_energy20_without = pd.read_csv('energy_tot_4_bytes_without.csv')
df_energy20_without = pd.DataFrame(data_energy20_without)
e20w=df_energy20_without.iat[10,1] #energy
et20w=df_energy20_without.iat[10,3] #ecartype

data_energy40_without = pd.read_csv('energy_tot_32_bytes_without.csv')
df_energy40_without = pd.DataFrame(data_energy40_without)
e40w=df_energy40_without.iat[10,1] #energy
et40w=df_energy40_without.iat[10,3] #ecartype

data_energy60_without = pd.read_csv('energy_tot_48_bytes_without.csv')
df_energy60_without = pd.DataFrame(data_energy60_without)
e60w=df_energy60_without.iat[10,1] #energy
et60w=df_energy60_without.iat[10,3] #ecartype

data_energy80_without = pd.read_csv('energy_tot_64_bytes_without.csv')
df_energy80_without = pd.DataFrame(data_energy80_without)
e80w=df_energy80_without.iat[10,1] #energy
et80w=df_energy80_without.iat[10,3] #ecartype

data_energy100_without = pd.read_csv('energy_tot_88_bytes_without.csv')
df_energy100_without = pd.DataFrame(data_energy100_without)
e100w=df_energy100_without.iat[10,1] #energy
et100w=df_energy100_without.iat[10,3] #ecartype

B=[4,32,48,64,88]
E=[e20,e40,e60,e80,e100]
ET=[et20,et40,et60,et80,et100]
Bw=[4,32,48,64,88] 
Ew=[e20w,e40w,e60w,e80w,e100w]
ETw=[et20w,et40w,et60w,et80w,et100w]
print(E)
print("")
print(Ew)

# creating the bar plot
plt.figure(figsize=(16,8))
plt.errorbar(B,E, yerr=ET,capsize=4,color ='g') 
plt.plot(B, E,color ='g',label='With LTL')

plt.errorbar(Bw,Ew, yerr=ETw,capsize=4,color ='orange')
plt.plot(Bw, Ew, color ='orange',label='Without LTL')
plt.yticks(fontsize=30)
plt.xticks(fontsize=30)
plt.xlabel("Sensor data (Bytes)",fontsize=30)
plt.ylabel("Energy (mJ)" ,fontsize=30)
plt.ylim(0,0.4*1000)
#plt.ylim(0.85*1000,0.95*1000)
plt.grid();
#plt.title("Increase in energy consumption as a function of bytes sent ")
plt.tight_layout()
plt.legend(fontsize=30)
plt.savefig("fig6_energy_as_function_of_bytes_sent.png")
plt.show()

