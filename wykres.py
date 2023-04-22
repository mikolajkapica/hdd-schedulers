# import csv and make graph of x, y

import csv
import matplotlib.pyplot as plt

# for all csv files in directory
# make 0,0 point in the left top corner


import glob
for filename in glob.glob('csv/*.csv'):
    # new plt
    plt.clf()
    print(filename)
    x = []
    y = []
    with open(filename, 'r') as csvfile:
        plots = csv.reader(csvfile, delimiter=',')
        for row in plots:
            x.append(int(row[1]))
            y.append(float(row[0]))
        plt.plot(x, y)
        plt.gca().invert_yaxis()
        # plt.gca().invert_xaxis()
        plt.xlabel('track')
        plt.ylabel('time')
        # filename as title without .csv
        plt.title(filename.replace('.csv', ''))
        # save to jpg
        plt.savefig(filename.replace('csv/', 'img/').replace('.csv', '.jpg'))