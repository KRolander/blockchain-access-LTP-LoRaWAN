import matplotlib.pyplot as plt
import pandas as pd
import csv

#Energy disponible
Eunepile=2.4
Ndepile=2
#Ah-->Wh
Edispo=Eunepile*Ndepile*3

#Scénario
Txinterval=10
Nb_chiffre_significatif=5

#Quantif mode statique 
data_energystatic = pd.read_csv('enrgy_mj_seconde_static.csv')
df_energystatic = pd.DataFrame(data_energystatic)
e_par_sec_mJ=df_energystatic.iat[0,0] #energy

#Quantif energy static (mJ/transaction)
E_static=e_par_sec_mJ*Txinterval

#Quantif without LTL dynamic 
data_energy100_without = pd.read_csv('result_1_edge_24_bytes_without.csv')
df_energy100_without = pd.DataFrame(data_energy100_without)
e100w=df_energy100_without.iat[4,1]#energy mj
t100w=df_energy100_without.iat[4,2] #time en ms
e100w=float(e100w)
t100w=float(t100w)/1000

#Quantif with LTL dynamic 
data_energy100 = pd.read_csv('result_1_edge_24_bytes.csv')
df_energy100 = pd.DataFrame(data_energy100)
e100=df_energy100.iat[4,1] #energy mj
t100=df_energy100.iat[4,2] #time en ms
e100=float(e100)
t100=float(t100)/1000

#Energie et temps tot 1 transaction
e_tot_with=e100+E_static
e_tot_without=e100w+E_static

t_tot_with=Txinterval+t100
t_tot_without=Txinterval+t100w

#Convert e tot mJ en Wh
e_tot_with_wh=(e_tot_with/1000)/3600
e_tot_without_wh=(e_tot_without/1000)/3600

#Nb de transaction (nb entier)
Nb_transac_with=Edispo/e_tot_with_wh
Nb_transac_without=Edispo/e_tot_without_wh
Nb_transac_with=int(Nb_transac_with)
Nb_transac_without=int(Nb_transac_without)

#Passage en temps 
Temps_with_s=Nb_transac_with*t_tot_with
Temps_without_s=Nb_transac_without*t_tot_without
Temps_with_j=((Temps_with_s/60)/60)/24
Temps_without_j=((Temps_without_s/60)/60)/24
print(Temps_with_j)
print(Temps_without_j)
#Temps_with_j=int(Temps_with_j)
#Temps_without_j=int(Temps_without_j)
#print(Temps_with_j)
#print(Temps_without_j)
# ouverture en écriture d'un fichier
with open('result_part_4.csv', 'w', newline='') as files:

    # on déclare un objet writer 
    ecrivain = csv.writer(files)

   # écrire une ligne dans le fichier:
    ecrivain.writerow(['--', 'Without LTL ', 'With LTL'])
    # quelques lignes:
    ecrivain.writerow([ 'E static(mJ/transaction)', E_static,E_static])
    ecrivain.writerow([ 'E dynamique(mJ/transaction)',e100w,e100])
    ecrivain.writerow([ 'E total',e_tot_without,e_tot_with])
    ecrivain.writerow([ 'Estimation en jour', Temps_without_j,Temps_with_j])


